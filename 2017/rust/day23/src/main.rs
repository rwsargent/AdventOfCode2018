use std::cmp;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::iter::FromIterator;
use std::num;
use std::ops::Index;
use std::slice::Split;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main2() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let result = run(&contents);

    println!("result: {:?}", result);
}

fn main3() {
    let mut b: i64 = 106500;
    let mut c: i64 = 123500;
    let mut d: i64 = 2;
    let mut e: i64 = 2;
    let mut f: i64 = 1;
    let mut g: i64 = 2;
    let mut h: i64 = 0;


    while b <= c {
        println!("loop");
        f = 1;
        d = 2;
        while f == 1 && d != b {
            e = 2;
            while f == 1 && e != b {
                if d * e == b {
                    f = 0;
                }
                e += 1;
            }

            d += 1
        }
        if f == 0 {
            h += 1;
        }

        b += 17
    }

    println!("h: {}", h);
}

fn main() {
    let mut b: i64 = 106500;
    let mut c: i64 = 123500;
    let mut d: i64 = 2;
    let mut h: i64 = 0;

    while b <= c {
        println!("loop");
        d = 2;

        while d < b {
            if b % d == 0 {
                h += 1;
                break;
            }
            d += 1
        }
        b += 17
    }

    println!("h: {}", h);
}

struct Registers {
    registers: HashMap<String, i64>
}

impl Registers {
    fn parse_value(&self, thing: &str) -> i64 {
        match thing.parse::<i64>() {
            Ok(x) => x,
            Err(_) => {
                self.registers.get(&thing.to_string()).map(|x| *x).unwrap_or_else(|| 0)
            }
        }
    }
}

fn run(program: &String) -> i64 {
    let mut last_sound_played = 0;
    let mut num_muls = 0;
    let mut registers = Registers { registers: HashMap::new() };
    registers.registers.insert("a".to_string(), 1);
    let cmds = program.split("\n").filter(|x| !x.is_empty()).map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut idx: i64 = 0;
    while idx >= 0 && (idx as usize) < cmds.len() {
        let cmd = &cmds[idx as usize];
        println!("{}\t\t{:?}\t\t{:?}", idx, cmd, registers.registers);
        let mut jumped = false;
        match cmd[0] {
            "set" => {
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(cmd[1].to_string(), val);
            }
            "sub" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old - val);
            }
            "mul" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old * val);
                num_muls += 1;
            }
            "jnz" => {
                let val = registers.parse_value(cmd[1]);
                if val != 0 {
                    idx += registers.parse_value(cmd[2]);
                    jumped = true;
                }
            }
            _ => unimplemented!()
        }
        if !jumped {
            idx += 1;
        }
    }
    num_muls
}

fn run2(program: &String) -> i64 {
    let mut registers = Vec::new();
    registers.push(1);
    registers.push(0);
    registers.push(0);
    registers.push(0);
    registers.push(0);
    registers.push(0);
    registers.push(0);
    registers.push(0);
    let mut idx = 0;

    let cmds = program.split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.split_whitespace().collect::<Vec<_>>())
        .map(parse_instruction)
        .collect::<Vec<_>>();

    let mut instructions = 0 as u64;
    while idx >= 0 && (idx as usize) < cmds.len() {
        idx += cmds[idx as usize](&mut registers);
        instructions += 1;
        if instructions % 10000000 == 0 {
            println!("{}: {:?}", instructions, registers);
        }
    }
    registers[7] // register 'h'
}


fn parse_instruction(cmd: Vec<&str>) -> Box<Fn(&mut Vec<i64>) -> i64> {
    match cmd[0] {
        "set" => {
            let i = Value::str_to_idx(cmd[1]);
            match Value::parse(cmd[2]) {
                Value::Const(x) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] = x;
                    1
                }),
                Value::Register(j) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] = regs[j];
                    1
                })
            }
        }
        "sub" => {
            let i = Value::str_to_idx(cmd[1]);
            match Value::parse(cmd[2]) {
                Value::Const(x) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] -= x;
                    1
                }),
                Value::Register(j) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] -= regs[j];
                    1
                })
            }
        }
        "mul" => {
            let i = Value::str_to_idx(cmd[1]);
            match Value::parse(cmd[2]) {
                Value::Const(x) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] *= x;
                    1
                }),
                Value::Register(j) => Box::new(move |regs: &mut Vec<i64>| {
                    regs[i] *= regs[j];
                    1
                })
            }
        }
        "jnz" => {
            let x = Value::parse(cmd[1]);
            let y = Value::parse(cmd[2]);
            match x {
                Value::Const(x) => {
                    if x != 0 {
                        match y {
                            Value::Const(y) => Box::new(move |_| y),
                            Value::Register(y) => Box::new(move |regs: &mut Vec<i64>| regs[y])
                        }
                    } else {
                        Box::new(|_| 1)
                    }
                }
                Value::Register(x) => {
                    match y {
                        Value::Const(y) => Box::new(move |regs: &mut Vec<i64>| {
                            if regs[x] != 0 {
                                y
                            } else {
                                1
                            }
                        }),
                        Value::Register(y) => Box::new(move |regs: &mut Vec<i64>| {
                            if regs[x] != 0 {
                                regs[y]
                            } else {
                                1
                            }
                        })
                    }
                }
            }
        }
        _ => unimplemented!()
    }
}

enum Value {
    Const(i64),
    Register(usize),
}

impl Value {
    fn parse(str: &str) -> Value {
        match str.parse::<i64>() {
            Ok(x) => Value::Const(x),
            Err(_) => Value::Register(Value::str_to_idx(str))
        }
    }
    fn str_to_idx(str: &str) -> usize {
        match str {
            "a" => 0,
            "b" => 1,
            "c" => 2,
            "d" => 3,
            "e" => 4,
            "f" => 5,
            "g" => 6,
            "h" => 7,
            _ => unimplemented!()
        }
    }
}
