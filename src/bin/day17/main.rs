#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Registers {
    a: u64,
    b: u64,
    c: u64
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
            7 => panic!("invalid combo operand")
        }
    }

    fn get_value(&self, registers: &Registers) -> u64 {
        match self {
            Operand::Literal(byte) => byte as u64,
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

    fn execute(&self, registers: &Registers) -> (Option<u64>, Registers) {
        match self {
            Instruction::Adv(operand) => {
                let numerator = registers.a;
                let denominator = 1 << operand.get_value(registers);
                (None, Registers { ..registers, a: numerator / denominator })
            },
            Instruction::Bdv(operand) => {
                let numerator = registers.b;
                let denominator = 1 << operand.get_value(registers);
                (None, Registers { ..registers, b: numerator / denominator })
            },
            Instruction::Cdv(operand) => {
                let numerator = registers.c;
                let denominator = 1 << operand.get_value(registers);
                (None, Registers { ..registers, c: numerator / denominator })
            },
            Instruction::Bxl(operand) => {
                (None, Registers { ..registers, b: registers.b ^ operand.get_value() })
            },
            Instruction::Bst(operand) => {
                (None, Registers { ..

}

fn part_1(input: &str) -> String {

}

fn main() {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();

    println!("Part 1: {}", part_1(&input));
    //println!("Part 2: {}", part_2(&input));
}
