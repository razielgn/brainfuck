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
        '+' => Some(Instruction::Add),
        '-' => Some(Instruction::Sub),
        '>' => Some(Instruction::Right),
        '<' => Some(Instruction::Left),
        '.' => Some(Instruction::Out),
        ',' => Some(Instruction::In),
        '[' => Some(Instruction::Open),
        ']' => Some(Instruction::Close),
        _   => None,
    }
}
