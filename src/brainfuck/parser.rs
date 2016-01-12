use instruction::Instruction;

pub fn parse(bytes: &[u8]) -> Vec<Instruction> {
    let mut instructions = Vec::with_capacity(bytes.len());

    for b in bytes {
        if let Some(i) = parse_byte(b) {
            instructions.push(i);
        }
    }

    instructions
}

fn parse_byte(b: &u8) -> Option<Instruction> {
    match *b as char {
        '+' => Some(Instruction::Add(1)),
        '-' => Some(Instruction::Sub(1)),
        '>' => Some(Instruction::Right(1)),
        '<' => Some(Instruction::Left(1)),
        '.' => Some(Instruction::Out),
        ',' => Some(Instruction::In),
        '[' => Some(Instruction::Open),
        ']' => Some(Instruction::Close),
        _   => None,
    }
}
