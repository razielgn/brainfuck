#[derive(Debug, PartialEq, Eq)]
pub enum Instruction {
    Add,
    Sub,
    Right,
    Left,
    Out,
    In,
    Open,
    Close,
}
