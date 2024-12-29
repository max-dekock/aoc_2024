use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{u8, u64, line_ending},
    multi::separated_list1,
    sequence::{delimited, preceded}
};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
    ip: usize
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operand {
    Literal(u8),
    RegA,
    RegB,
    RegC
}

impl Operand {
    fn parse_literal(byte: u8) -> Operand {
        debug_assert!(byte <= 7);
        Operand::Literal(byte)
    }

    fn parse_combo(byte: u8) -> Operand {
        debug_assert!(byte <= 7);
        match byte {
            b @ 0..=3 => Operand::Literal(b),
            4 => Operand::RegA,
            5 => Operand::RegB,
            6 => Operand::RegC,
            _ => panic!("invalid combo operand")
        }
    }

    fn get_value(&self, registers: &Registers) -> u64 {
        match self {
            Operand::Literal(byte) => *byte as u64,
            Operand::RegA => registers.a,
            Operand::RegB => registers.b,
            Operand::RegC => registers.c
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Instruction {
    Adv(Operand),
    Bxl(Operand),
    Bst(Operand),
    Jnz(Operand),
    Bxc,
    Out(Operand),
    Bdv(Operand),
    Cdv(Operand)
}

impl Instruction {
    fn parse(opcode: u8, operand: u8) -> Instruction {
        match opcode {
            0 => Instruction::Adv(Operand::parse_combo(operand)),
            1 => Instruction::Bxl(Operand::parse_literal(operand)),
            2 => Instruction::Bst(Operand::parse_combo(operand)),
            3 => Instruction::Jnz(Operand::parse_literal(operand)),
            4 => Instruction::Bxc,
            5 => Instruction::Out(Operand::parse_combo(operand)),
            6 => Instruction::Bdv(Operand::parse_combo(operand)),
            7 => Instruction::Cdv(Operand::parse_combo(operand)),
            invalid => panic!("invalid opcode: {}", invalid)
        }
    }

    fn execute(&self, registers: &Registers) -> (Option<u8>, Registers) {
        match self {
            Instruction::Adv(operand) => {
                let numerator = registers.a;
                let denominator = 1 << operand.get_value(registers);
                let new_registers = Registers {
                    a: numerator / denominator,
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Bdv(operand) => {
                let numerator = registers.a;
                let denominator = 1 << operand.get_value(registers);
                let new_registers = Registers {
                    b: numerator / denominator,
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Cdv(operand) => {
                let numerator = registers.a;
                let denominator = 1 << operand.get_value(registers);
                let new_registers = Registers {
                    c: numerator / denominator,
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Bxl(operand) => {
                let new_registers = Registers {
                    b: registers.b ^ operand.get_value(registers),
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Bst(operand) => {
                let new_registers = Registers {
                    b: operand.get_value(registers) % 8,
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Jnz(operand) => {
                let ip = if registers.a == 0 { registers.ip + 2 } else { operand.get_value(registers) as usize };
                let new_registers = Registers {
                    ip,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Bxc => {
                let new_registers = Registers {
                    b: registers.b ^ registers.c,
                    ip: registers.ip + 2,
                    ..*registers
                };
                (None, new_registers)
            },
            Instruction::Out(operand) => {
                let new_registers = Registers {
                    ip: registers.ip + 2,
                    ..*registers
                };
                let output = (operand.get_value(registers) % 8) as u8;
                (Some(output), new_registers)
            }
        }
    }
}

fn execute_code(initial_registers: &Registers, code: &[u8]) -> Vec<u8> {
    let mut registers = *initial_registers;
    let mut out_buf = vec![];
    while registers.ip <= code.len() - 2 {
        let opcode = code[registers.ip];
        let operand = code[registers.ip + 1];
        let instruction = Instruction::parse(opcode, operand);
        //println!("{:?}", instruction);
        let (output, new_registers) = instruction.execute(&registers);
        if let Some(output) = output {
            out_buf.push(output);
        }
        registers = new_registers;
        //println!("{:#?}", registers);
    }
    out_buf
}

fn parse_input(input: &str) -> IResult<&str, (Registers, Vec<u8>)> {
    let (input, a) = delimited(tag("Register A: "), u64, line_ending)(input)?;
    let (input, b) = delimited(tag("Register B: "), u64, line_ending)(input)?;
    let (input, c) = delimited(tag("Register C: "), u64, line_ending)(input)?;
    let (input, _) = line_ending(input)?;
    let (input, code) = preceded(tag("Program: "), separated_list1(tag(","), u8))(input)?;
    let registers = Registers { a, b, c, ip: 0 };
    Ok((input, (registers, code)))
}

fn part_1(input: &str) -> String {
    let (_, (registers, code)) = parse_input(input).unwrap();
    let output = execute_code(&registers, &code);
    output.into_iter().join(",")
}

// by reverse engineering the input, i've deduced that every 3 bits in A results in one output
// we can figure out the correct value by backtracking
fn find_quine(code: &[u8], thus_far: u64, idx: usize) -> Option<u64> {
    for x in 0..=7 {
        let a = thus_far << 3 | x;
        let registers = Registers { a, b: 0, c: 0, ip: 0 };
        let output = execute_code(&registers, code);
        if output[0] == code[idx] {
            if idx == 0 {
                return Some(a);
            }
            if let Some(solution) = find_quine(code, a, idx - 1) {
                return Some(solution);
            }
        }
    }
    None
}

fn part_2(input: &str) -> u64 {
    let (_, (_, code)) = parse_input(input).unwrap();
    if let Some(solution) = find_quine(&code, 0, code.len() - 1) {
        return solution;
    }
    panic!("couldn't find a solution!");
}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}
