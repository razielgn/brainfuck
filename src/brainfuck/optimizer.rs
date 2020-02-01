use crate::instruction::Instruction;
use std::collections::VecDeque;

pub fn optimize(instructions: VecDeque<Instruction>) -> VecDeque<Instruction> {
    compact_binary(instructions)
}

fn compact_binary(
    mut instructions: VecDeque<Instruction>,
) -> VecDeque<Instruction> {
    use Instruction::*;

    if instructions.len() < 2 {
        return instructions;
    }

    let a = instructions.pop_front().unwrap();
    let b = instructions.pop_front().unwrap();

    match (a, b) {
        (Add(x), Add(y)) => {
            instructions.push_front(Add(x + y));
            compact_binary(instructions)
        }
        (Sub(x), Sub(y)) => {
            instructions.push_front(Sub(x + y));
            compact_binary(instructions)
        }
        (Right(x), Right(y)) => {
            instructions.push_front(Right(x + y));
            compact_binary(instructions)
        }
        (Left(x), Left(y)) => {
            instructions.push_front(Left(x + y));
            compact_binary(instructions)
        }
        (Add(x), Sub(y)) | (Sub(x), Add(y)) if x == y => {
            compact_binary(instructions)
        }
        (Right(x), Left(y)) | (Left(x), Right(y)) if x == y => {
            compact_binary(instructions)
        }
        _ => {
            instructions.push_front(b);
            let mut rest = compact_binary(instructions);
            rest.push_front(a);
            rest
        }
    }
}

#[cfg(test)]
mod test {
    use crate::instruction::Instruction::{self, *};
    use std::{collections::VecDeque, iter::FromIterator};

    fn optimize(vec: Vec<Instruction>) -> Vec<Instruction> {
        Vec::from_iter(
            super::optimize(VecDeque::from_iter(vec.into_iter())).into_iter(),
        )
    }

    #[test]
    fn compact_add() {
        assert_eq!(vec!(Add(1)), optimize(vec!(Add(1))));

        assert_eq!(
            vec!(Out, Add(3), Out),
            optimize(vec!(Out, Add(1), Add(1), Add(1), Out))
        );
    }

    #[test]
    fn compact_sub() {
        assert_eq!(vec!(Sub(1)), optimize(vec!(Sub(1))));

        assert_eq!(
            vec!(Out, Sub(3), Out),
            optimize(vec!(Out, Sub(1), Sub(1), Sub(1), Out))
        );
    }

    #[test]
    fn compact_add_sub() {
        assert_eq!(Vec::<Instruction>::new(), optimize(vec!(Add(5), Sub(5))));
    }

    #[test]
    fn compact_right() {
        assert_eq!(vec!(Right(10)), optimize(vec!(Right(5), Right(5))));
    }

    #[test]
    fn compact_left() {
        assert_eq!(vec!(Left(10)), optimize(vec!(Left(5), Left(5))));
    }

    #[test]
    fn compact_right_left() {
        assert_eq!(Vec::<Instruction>::new(), optimize(vec!(Right(5), Left(5))));
    }
}
