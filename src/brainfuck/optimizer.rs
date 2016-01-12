use instruction::Instruction;

pub fn optimize(instructions: Vec<Instruction>) -> Vec<Instruction> {
    compact_binary(instructions)
}

fn compact_binary(mut instructions: Vec<Instruction>) -> Vec<Instruction> {
    use instruction::Instruction::*;

    if instructions.len() < 2 { return instructions; }

    let mut tail = instructions.split_off(2);
    let head = instructions;

    match (head[0], head[1]) {
        (Add(x), Add(y)) => {
            let mut acc = vec!(Add(x + y));
            acc.append(&mut tail);
            compact_binary(acc)
        }
        (Sub(x), Sub(y)) => {
            let mut acc = vec!(Sub(x + y));
            acc.append(&mut tail);
            compact_binary(acc)
        }
        (Right(x), Right(y)) => {
            let mut acc = vec!(Right(x + y));
            acc.append(&mut tail);
            compact_binary(acc)
        }
        (Left(x), Left(y)) => {
            let mut acc = vec!(Left(x + y));
            acc.append(&mut tail);
            compact_binary(acc)
        }
        (Add(x), Sub(y)) | (Sub(x), Add(y)) if x  == y => {
            compact_binary(tail)
        }
        (Right(x), Left(y)) | (Left(x), Right(y)) if x == y => {
            compact_binary(tail)
        }
        (a, b) => {
            tail.insert(0, b);
            let mut acc = vec!(a);
            acc.append(&mut compact_binary(tail));
            acc
        }
    }
}

#[cfg(test)]
mod test {
    use instruction::Instruction;
    use instruction::Instruction::*;
    use super::optimize;

    #[test]
    fn compact_add() {
        assert_eq!(
            vec!(Add(1)),
            optimize(vec!(Add(1)))
        );

        assert_eq!(
            vec!(Out, Add(3), Out),
            optimize(vec!(Out, Add(1), Add(1), Add(1), Out))
        );
    }

    #[test]
    fn compact_sub() {
        assert_eq!(
            vec!(Sub(1)),
            optimize(vec!(Sub(1)))
        );

        assert_eq!(
            vec!(Out, Sub(3), Out),
            optimize(vec!(Out, Sub(1), Sub(1), Sub(1), Out))
        );
    }

    #[test]
    fn compact_add_sub() {
        assert_eq!(
            Vec::<Instruction>::new(),
            optimize(vec!(Add(5), Sub(5)))
        );
    }

    #[test]
    fn compact_right() {
        assert_eq!(
            vec!(Right(10)),
            optimize(vec!(Right(5), Right(5)))
        );
    }

    #[test]
    fn compact_left() {
        assert_eq!(
            vec!(Left(10)),
            optimize(vec!(Left(5), Left(5)))
        );
    }

    #[test]
    fn compact_right_left() {
        assert_eq!(
            Vec::<Instruction>::new(),
            optimize(vec!(Right(5), Left(5)))
        );
    }
}
