//use std::io::{self, BufRead, Write};
use core::ops::Range;

type FsmIndex = usize;

const FMS_COLUMN_SIZE : usize = 130;
const FSM_END_LINE: usize = 129;

#[derive(Default, Clone, Copy)]
struct FsmAction {
    next: FsmIndex,
    offset : i32,
}


struct FsmColumn {
    transisiton : [FsmAction; FMS_COLUMN_SIZE],
}

impl FsmColumn {
    fn new() -> Self {
        Self {
            transisiton : [Default::default(); FMS_COLUMN_SIZE]
        }
    }
}


struct Regex {
    columns: Vec<FsmColumn>
}

impl Regex {

    fn compile(source: &str) -> Self {
        let mut regex = Self { columns: Vec::new() };
        regex.columns.push(FsmColumn::new());

        for ch in source.chars() {
            let mut col = FsmColumn::new();

            match ch {
                '$' => {
                    col.transisiton[FSM_END_LINE] = FsmAction{
                        next: regex.columns.len() + 1,
                        offset: 1
                    };
                    regex.columns.push(col);
                }
                '.' => {
                    // go through only displaying chars
                    for i in 32..127 {
                        col.transisiton[i] = FsmAction {
                            next: regex.columns.len() + 1,
                            offset: 1
                        }
                    }
                    regex.columns.push(col);
                }
                '*' => {
                    let column_len = regex.columns.len();
                    for t in regex.columns.last_mut().unwrap().transisiton.iter_mut() {
                        if t.next == column_len {
                            t.next = column_len - 1;
                        }
                        else if t.next == 0 {
                            t.next = column_len;
                            t.offset = 0;
                        }
                        else {
                            unreachable!();
                        }
                    }
                }
                _ => {
                    col.transisiton[ch as usize] = FsmAction{
                        next: regex.columns.len() + 1,
                        offset: 1,
                    };
                    regex.columns.push(col);
                }
            }
        }

        regex
    }

    fn match_str(&self, input: &str) -> bool{
        let mut state = 1;
        let mut head = 0;
        let chars = input.chars().collect::<Vec<_>>();
        let chars_len = chars.len();
        while state > 0 && state < self.columns.len() && head <  chars_len {
            let action = self.columns[state].transisiton[chars[head] as usize];
            state = action.next;
            head = (head as i32 + action.offset) as usize;
        }

        if state == 0 {
            return false;
        }

        if state < self.columns.len() {
            let action = self.columns[state].transisiton[FSM_END_LINE];
            state = action.next;
        }

        return state >= self.columns.len();
    }

    fn dump(&self) {
        for symbol in 0..FMS_COLUMN_SIZE {
            print!("{:03} => ", symbol);
            for column in self.columns.iter()  {
                print!("({}, {})",
                       column.transisiton[symbol].next,
                       column.transisiton[symbol].offset);
            }
            println!();
        }
    }
}

fn main()
{
    let src = "a*bc";
    let mut regex = Regex::compile(src);
    regex.dump();

    println!("_______________________");
    println!("Regex is '{}'", src);
    let inputs = vec!["abc", "bbc", "cbc","cbd",  "cbt", "abcd"];
    for input in inputs.iter() {
        println!("{:?} => {:?}", input, regex.match_str(input));
    }

    //https://www.youtube.com/watch?v=MH56D5M9xSQ&t=5204s&ab_channel=TsodingDaily - 2:01:59
}
