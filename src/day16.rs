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

fn parse_sample(input: &str) -> (Device, Instruction, Device) {
    let parts: Vec<&str> = input.trim().split('\n').collect();
    (
        Device::from_str(parts[0].replace("Before: ", "").as_str()).unwrap(),
        Instruction::from_str(parts[1]).unwrap(),
        Device::from_str(parts[2].replace("After:  ", "").as_str()).unwrap(),
    )
}

#[derive(Debug)]
enum Instruction {
    ADDR(i64, i64, i64),
    ADDI(i64, i64, i64),
    MULR(i64, i64, i64),
    MULI(i64, i64, i64),
    BANR(i64, i64, i64),
    BANI(i64, i64, i64),
    BORR(i64, i64, i64),
    BORI(i64, i64, i64),
    SETR(i64, i64, i64),
    SETI(i64, i64, i64),
    GTIR(i64, i64, i64),
    GTRI(i64, i64, i64),
    GTRR(i64, i64, i64),
    EQIR(i64, i64, i64),
    EQRI(i64, i64, i64),
    EQRR(i64, i64, i64),

    OP(i64, i64, i64, i64),
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Vec<i64> = s
            .trim()
            .split(' ')
            .map(|s| s.trim().parse().unwrap())
            .collect();
        Ok(Instruction::OP(r[0], r[1], r[2], r[3]))
    }
}

impl Instruction {
    fn enumerate_all(&self) -> Vec<Self> {
        if let Instruction::OP(_, a, b, c) = &self {
            return vec![
                Instruction::ADDR(*a, *b, *c),
                Instruction::ADDI(*a, *b, *c),
                Instruction::MULR(*a, *b, *c),
                Instruction::MULI(*a, *b, *c),
                Instruction::BANR(*a, *b, *c),
                Instruction::BANI(*a, *b, *c),
                Instruction::BORR(*a, *b, *c),
                Instruction::BORI(*a, *b, *c),
                Instruction::SETR(*a, *b, *c),
                Instruction::SETI(*a, *b, *c),
                Instruction::GTIR(*a, *b, *c),
                Instruction::GTRI(*a, *b, *c),
                Instruction::GTRR(*a, *b, *c),
                Instruction::EQIR(*a, *b, *c),
                Instruction::EQRI(*a, *b, *c),
                Instruction::EQRR(*a, *b, *c),
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
        match i {
            Instruction::ADDR(a, b, c) => {
                result.registers[*c as usize] =
                    result.registers[*a as usize] + result.registers[*b as usize]
            }
            Instruction::ADDI(a, b, c) => {
                result.registers[*c as usize] = result.registers[*a as usize] + *b
            }
            Instruction::MULR(a, b, c) => {
                result.registers[*c as usize] =
                    result.registers[*a as usize] * result.registers[*b as usize]
            }
            Instruction::MULI(a, b, c) => {
                result.registers[*c as usize] = result.registers[*a as usize] * *b
            }
            Instruction::BANR(a, b, c) => {
                result.registers[*c as usize] =
                    result.registers[*a as usize] & result.registers[*b as usize]
            }
            Instruction::BANI(a, b, c) => {
                result.registers[*c as usize] = result.registers[*a as usize] & *b
            }
            Instruction::BORR(a, b, c) => {
                result.registers[*c as usize] =
                    result.registers[*a as usize] | result.registers[*b as usize]
            }
            Instruction::BORI(a, b, c) => {
                result.registers[*c as usize] = result.registers[*a as usize] | *b
            }
            Instruction::SETR(a, _, c) => {
                result.registers[*c as usize] = result.registers[*a as usize]
            }
            Instruction::SETI(a, _, c) => result.registers[*c as usize] = *a,
            Instruction::GTIR(a, b, c) => {
                result.registers[*c as usize] = if *a > result.registers[*b as usize] {
                    1
                } else {
                    0
                }
            }
            Instruction::GTRI(a, b, c) => {
                result.registers[*c as usize] = if result.registers[*a as usize] > *b {
                    1
                } else {
                    0
                }
            }
            Instruction::GTRR(a, b, c) => {
                result.registers[*c as usize] =
                    if result.registers[*a as usize] > result.registers[*b as usize] {
                        1
                    } else {
                        0
                    }
            }
            Instruction::EQIR(a, b, c) => {
                result.registers[*c as usize] = if *a == result.registers[*b as usize] {
                    1
                } else {
                    0
                }
            }
            Instruction::EQRI(a, b, c) => {
                result.registers[*c as usize] = if result.registers[*a as usize] == *b {
                    1
                } else {
                    0
                }
            }
            Instruction::EQRR(a, b, c) => {
                result.registers[*c as usize] =
                    if result.registers[*a as usize] == result.registers[*b as usize] {
                        1
                    } else {
                        0
                    }
            }
            _ => {}
        }
        result
    }
}
