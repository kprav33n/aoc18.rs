use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

/// Find the number of samples that behave like three or more opcodes.
///
/// # Examples
///
/// ```
/// use aoc18::day16::num_samples;
///
/// assert_eq!(1, num_samples("Before: [3, 2, 1, 1]
/// 9 2 1 2
/// After:  [3, 2, 2, 1]"));
/// ```
pub fn num_samples(input: &str) -> usize {
    let parts: Vec<&str> = input.split("\n\n\n\n").collect();
    let samples = parts[0];
    samples
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(parse_sample)
        .filter(|(before, op, after)| {
            let mut count = 0;
            for i in op.enumerate_all() {
                if before.execute(&i) == *after {
                    count += 1;
                }
            }
            count >= 3
        })
        .count()
}

/// Find the register 0 in the result after executing the test program.
pub fn result_r0(input: &str) -> i64 {
    let parts: Vec<&str> = input.split("\n\n\n\n").collect();
    let samples = parts[0];
    let program = parts[1];
    // Populate a list of set of possible instructions for every OP code.
    let mut op_table: HashMap<i64, Rc<RefCell<Vec<HashSet<Operation>>>>> = HashMap::new();
    for sample in samples
        .split("\n\n")
        .filter(|s| !s.trim().is_empty())
        .map(parse_sample)
    {
        let (before, op, after) = sample;
        let mut set = HashSet::new();
        for i in op.enumerate_all() {
            if before.execute(&i) == after {
                set.insert(i.op);
            }
        }
        if let Operation::OP(o) = op.op {
            op_table
                .entry(o)
                .or_insert_with(|| Rc::new(RefCell::new(vec![])))
                .borrow_mut()
                .push(set);
        }
    }

    let mut known = vec![Operation::OP(-1); 16];
    let mut unprocessed = vec![HashSet::new(); 16];

    // First find the most certain instructions for a given OP code.
    for (o, l) in op_table.iter() {
        let all_ops: HashSet<Operation> = [
            Operation::ADDR,
            Operation::ADDI,
            Operation::MULR,
            Operation::MULI,
            Operation::BANR,
            Operation::BANI,
            Operation::BORR,
            Operation::BORI,
            Operation::SETR,
            Operation::SETI,
            Operation::GTIR,
            Operation::GTRI,
            Operation::GTRR,
            Operation::EQIR,
            Operation::EQRI,
            Operation::EQRR,
        ]
        .iter()
        .cloned()
        .collect();
        let reduced: HashSet<_> = l
            .borrow()
            .iter()
            .fold(all_ops, |acc, x| acc.intersection(&x).cloned().collect());
        if reduced.len() == 1 {
            known[*o as usize] = reduced.iter().next().unwrap().clone();
        } else {
            unprocessed[*o as usize] = reduced;
        }
    }

    loop {
        // Eliminate redundant choices based on known choices.
        let mut should_break = true;
        for j in 0..16 {
            let c = known[j].clone();
            if c == Operation::OP(-1) {
                continue;
            }
            for (i, u) in unprocessed.iter_mut().enumerate() {
                u.remove(&c);
                if u.len() > 1 {
                    should_break = false;
                } else if u.len() == 1 {
                    known[i] = u.iter().next().unwrap().clone();
                }
            }
        }
        if should_break {
            break;
        }
    }

    program
        .split('\n')
        .filter(|s| !s.trim().is_empty())
        .fold(
            Device {
                registers: [0, 0, 0, 0],
            },
            |acc, s| {
                let i = Instruction::from_str(s).unwrap();
                if let Operation::OP(o) = i.op {
                    let ni = Instruction {
                        op: known[o as usize].clone(),
                        a: i.a,
                        b: i.b,
                        c: i.c,
                    };
                    acc.execute(&ni)
                } else {
                    acc
                }
            },
        )
        .registers[0]
}

fn parse_sample(input: &str) -> (Device, Instruction, Device) {
    let parts: Vec<&str> = input.trim().split('\n').collect();
    (
        Device::from_str(parts[0].replace("Before: ", "").as_str()).unwrap(),
        Instruction::from_str(parts[1]).unwrap(),
        Device::from_str(parts[2].replace("After:  ", "").as_str()).unwrap(),
    )
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Operation {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,

    OP(i64),
}

#[derive(Clone, Debug, PartialEq)]
struct Instruction {
    op: Operation,
    a: i64,
    b: i64,
    c: i64,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Vec<i64> = s
            .trim()
            .split(' ')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Instruction {
            op: Operation::OP(r[0]),
            a: r[1],
            b: r[2],
            c: r[3],
        })
    }
}

impl Instruction {
    fn enumerate_all(&self) -> Vec<Self> {
        if let Operation::OP(_) = &self.op {
            return vec![
                Instruction {
                    op: Operation::ADDR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::ADDI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::MULR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::MULI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::BANR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::BANI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::BORR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::BORI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::SETR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::SETI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::GTIR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::GTRI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::GTRR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::EQIR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::EQRI,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
                Instruction {
                    op: Operation::EQRR,
                    a: self.a,
                    b: self.b,
                    c: self.c,
                },
            ];
        }
        panic!("not a generic instruction")
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Device {
    registers: [i64; 4],
}

impl FromStr for Device {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Vec<i64> = s
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Device {
            registers: [r[0], r[1], r[2], r[3]],
        })
    }
}

impl Device {
    fn execute(&self, i: &Instruction) -> Self {
        let mut result = self.clone();
        match i.op.clone() {
            Operation::ADDR => {
                result.registers[i.c as usize] =
                    result.registers[i.a as usize] + result.registers[i.b as usize]
            }
            Operation::ADDI => {
                result.registers[i.c as usize] = result.registers[i.a as usize] + i.b
            }
            Operation::MULR => {
                result.registers[i.c as usize] =
                    result.registers[i.a as usize] * result.registers[i.b as usize]
            }
            Operation::MULI => {
                result.registers[i.c as usize] = result.registers[i.a as usize] * i.b
            }
            Operation::BANR => {
                result.registers[i.c as usize] =
                    result.registers[i.a as usize] & result.registers[i.b as usize]
            }
            Operation::BANI => {
                result.registers[i.c as usize] = result.registers[i.a as usize] & i.b
            }
            Operation::BORR => {
                result.registers[i.c as usize] =
                    result.registers[i.a as usize] | result.registers[i.b as usize]
            }
            Operation::BORI => {
                result.registers[i.c as usize] = result.registers[i.a as usize] | i.b
            }
            Operation::SETR => result.registers[i.c as usize] = result.registers[i.a as usize],
            Operation::SETI => result.registers[i.c as usize] = i.a,
            Operation::GTIR => {
                result.registers[i.c as usize] = if i.a > result.registers[i.b as usize] {
                    1
                } else {
                    0
                }
            }
            Operation::GTRI => {
                result.registers[i.c as usize] = if result.registers[i.a as usize] > i.b {
                    1
                } else {
                    0
                }
            }
            Operation::GTRR => {
                result.registers[i.c as usize] =
                    if result.registers[i.a as usize] > result.registers[i.b as usize] {
                        1
                    } else {
                        0
                    }
            }
            Operation::EQIR => {
                result.registers[i.c as usize] = if i.a == result.registers[i.b as usize] {
                    1
                } else {
                    0
                }
            }
            Operation::EQRI => {
                result.registers[i.c as usize] = if result.registers[i.a as usize] == i.b {
                    1
                } else {
                    0
                }
            }
            Operation::EQRR => {
                result.registers[i.c as usize] =
                    if result.registers[i.a as usize] == result.registers[i.b as usize] {
                        1
                    } else {
                        0
                    }
            }
            _ => panic!("not implemented"),
        }
        result
    }
}
