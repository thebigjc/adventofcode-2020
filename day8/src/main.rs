static INPUT: &str = include_str!("../day8.txt");
use std::str::FromStr;

#[derive(Debug, Clone)]
enum OpCode {
    Jmp,
    Acc,
    Nop,
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        match input {
            "jmp" => Ok(OpCode::Jmp),
            "acc" => Ok(OpCode::Acc),
            "nop" => Ok(OpCode::Nop),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Op {
    op: OpCode,
    p: isize,
    execd: bool,
}

fn run_ops(ops_orig: &Vec<Op>) -> (isize, bool) {
    let mut ip = 0;
    let mut acc = 0;
    let mut ops = ops_orig.to_vec();

    loop {
        if ops[ip].execd {
            return (acc, false);
        }

        ops[ip].execd = true;

        match &ops[ip].op {
            OpCode::Acc => {
                acc += ops[ip].p;
                ip += 1;
            }
            OpCode::Jmp => {
                if ops[ip].p < 0 {
                    ip = ip - (-ops[ip].p) as usize;
                } else {
                    ip = ip + (ops[ip].p as usize);
                }
            }
            OpCode::Nop => {
                ip += 1;
            }
        }

        if ip == ops.len() {
            return (acc, true);
        }
        if ip > ops.len() {
            return (acc, false);
        }
    }
}

fn main() {
    let mut ops = Vec::new();

    for l in INPUT.lines() {
        let op_p: Vec<&str> = l.split_whitespace().collect();
        let opcode = OpCode::from_str(op_p[0]).unwrap();
        let p: isize = op_p[1].parse().unwrap();

        let op = Op {
            op: opcode,
            p: p,
            execd: false,
        };

        ops.push(op);
    }

    let (acc, _) = run_ops(&ops);

    println!("{}", acc);

    for i in 0..ops.len() {
        let mut ops_copy = ops.to_vec();

        match ops_copy[i].op {
            OpCode::Acc => continue,
            OpCode::Nop => ops_copy[i].op = OpCode::Jmp,
            OpCode::Jmp => ops_copy[i].op = OpCode::Nop,
        }

        let (acc, finished) = run_ops(&ops_copy);
        if finished {
            println!("{}", acc);
        }
    }
}
