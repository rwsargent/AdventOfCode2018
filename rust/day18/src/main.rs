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

fn main() {
    let input = env::args().skip(1).next().unwrap();

    let mut f = File::open(input).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let (s1, r1) = channel();
    let (s2, r2) = channel();

    let c2 = contents.clone();
    let t1 = thread::spawn(move ||{
        run2(&c2, 0, r1, s2)
    });

    let result2 = run2(&contents, 1, r2, s1);

    println!("result: {:?}", result2);
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
    let mut registers = Registers { registers: HashMap::new() };
    let cmds = program.split("\n").filter(|x| !x.is_empty()).map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut idx: i64 = 0;
    while idx >= 0 && (idx as usize) <= cmds.len() {
        let cmd = &cmds[idx as usize];
//        println!("{}\t\t{:?}\t\t{:?}", idx, cmd, registers.registers);
        let mut jumped = false;
        match cmd[0] {
            "snd" => {
                last_sound_played = registers.parse_value(cmd[1]);
            }
            "set" => {
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(cmd[1].to_string(), val);
            }
            "add" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old + val);
            }
            "mul" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old * val);
            }
            "mod" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old % val);
            }
            "rcv" => {
                let val = registers.parse_value(cmd[1]);
                if val != 0 {
                    return last_sound_played;
                    registers.registers.insert(cmd[1].to_string(), last_sound_played);
                }
            }
            "jgz" => {
                let x = registers.parse_value(cmd[1]);
                if x > 0 {
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
    last_sound_played
}


fn run2(program: &String, program_id: i64, receiver: Receiver<i64>, sender: Sender<i64>) -> i64 {
    let mut registers = Registers { registers: HashMap::new() };
    registers.registers.insert("p".to_string(), program_id);
    let mut values_sent = 0;
    let cmds = program.split("\n").filter(|x| !x.is_empty()).map(|x| x.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut idx: i64 = 0;
    while idx >= 0 && (idx as usize) <= cmds.len() {
        let cmd = &cmds[idx as usize];
        println!("{}\t\t{:?}\t\t{:?}", idx, cmd, registers.registers);
        let mut jumped = false;
        match cmd[0] {
            "snd" => {
                values_sent += 1;
                sender.send(registers.parse_value(cmd[1]));
            }
            "set" => {
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(cmd[1].to_string(), val);
            }
            "add" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old + val);
            }
            "mul" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old * val);
            }
            "mod" => {
                let reg = cmd[1].to_string();
                let old = registers.parse_value(cmd[1]);
                let val = registers.parse_value(cmd[2]);
                registers.registers.insert(reg, old % val);
            }
            "rcv" => {
                let val = match receiver.recv_timeout(Duration::from_secs(10)){
                    Ok(x) => x,
                    Err(_) => {
                        return values_sent;
                    }
                };
                registers.registers.insert(cmd[1].to_string(), val);
            }
            "jgz" => {
                let x = registers.parse_value(cmd[1]);
                if x > 0 {
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
    values_sent
}


#[test]
fn graph_test() {
    let mut program = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2".to_string();
    assert_eq!(run(&program), 4);
}
