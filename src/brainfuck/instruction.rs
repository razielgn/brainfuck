#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Instruction {
    Add(u8),
    Sub(u8),
    Right(usize),
    Left(usize),
    Out,
    In,
    Open,
    Close,
}
