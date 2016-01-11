use std::io;
use std::iter::{self, FromIterator};
use std::ops::Range;
use std::result;

const TAPE_SIZE: usize = 30_000;

pub type Result = result::Result<(), Error>;
pub type Position = (usize, usize);

pub struct Brainfuck<'a> {
    program: &'a [u8],
    ip: usize,
    data: Vec<u8>,
    dp: usize,
    stack: Vec<(usize, Position)>,
    pos: Position,
}

#[derive(Debug)]
pub enum Error {
    ReadError(io::Error),
    WriteError(io::Error),
    UnbalancedParens(Position),
}

impl<'a> Brainfuck<'a> {
    pub fn new(program: &str) -> Brainfuck {
        Brainfuck {
            program: program.as_bytes(),
            ip: 0,
            data: Vec::from_iter(iter::repeat(0).take(TAPE_SIZE)),
            dp: 0,
            stack: Vec::new(),
            pos: (1, 1),
        }
    }

    #[allow(dead_code)]
    pub fn data_pointer(&self) -> usize {
        self.dp
    }

    #[allow(dead_code)]
    pub fn data(&self, range: Range<usize>) -> &[u8] {
        &self.data[range]
    }

    #[allow(dead_code)]
    pub fn run_pure(&mut self) -> Result {
        self.run(
            &mut io::empty(),
            &mut io::sink(),
        )
    }

    pub fn run<R, W>(&mut self, input: &mut R, output: &mut W) -> Result
        where R: io::Read, W: io::Write
    {
        loop {
            match self.current() {
                Some('>') => {
                    if self.dp < self.data.len() - 1 {
                        self.dp += 1;
                    }
                },
                Some('<') => {
                    if self.dp > 0 {
                        self.dp -= 1;
                    }
                },
                Some('+') => {
                    if self.data[self.dp] == 254 {
                        self.data[self.dp] = 0;
                    } else {
                        self.data[self.dp] += 1;
                    }
                },
                Some('-') => {
                    if self.data[self.dp] == 0 {
                        self.data[self.dp] = 254;
                    } else {
                        self.data[self.dp] -= 1;
                    }
                },
                Some('.') => {
                    let _ = try!(
                        output
                            .write(&[self.data[self.dp]])
                            .map_err(Error::WriteError)
                    );
                },
                Some(',') => {
                    let mut buffer = [0; 1];
                    let _ = try!(
                        input
                            .read(&mut buffer)
                            .map_err(Error::ReadError)
                    );
                    self.data[self.dp] = buffer[0];
                },
                Some('[') => {
                    if self.data[self.dp] == 0 {
                        self.advance_to_matching_paren();
                    } else {
                        self.push();
                    }
                },
                Some(']') => {
                    if self.data[self.dp] != 0 {
                        try!(self.return_to_matching_paren());
                    } else {
                        self.pop();
                    }
                },
                Some('\n') => {
                    self.pos.0 += 1;
                    self.pos.1 = 1;
                },
                Some(_) => {},
                None => {
                    break;
                }
            };

            self.advance();
        }

        Ok(())
    }

    fn advance(&mut self) {
        self.ip += 1;
        self.pos.1 += 1;
    }

    fn current(&self) -> Option<char> {
        self.program
            .get(self.ip)
            .map(|byte| *byte as char)
    }

    fn pop(&mut self) {
        let _ = self.stack.pop();
    }

    fn push(&mut self) {
        self.stack.push((self.ip, self.pos));
    }

    fn advance_to_matching_paren(&mut self) {
        let mut c = 0;

        loop {
            self.advance();

            match self.current() {
                None | Some(']') if c == 0 =>
                    break,
                Some(']') =>
                    c -= 1,
                Some('[') =>
                    c += 1,
                _ => {}
            }
        }
    }

    fn return_to_matching_paren(&mut self) -> Result {
        match self.stack.last() {
            Some(&(ip, pos)) => {
                self.ip = ip;
                self.pos = pos;
            },
            None =>
                return Err(Error::UnbalancedParens(self.pos)),
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use super::{Brainfuck, Error};

    #[test]
    fn initialized() {
        let brainfuck = Brainfuck::new("");

        assert_eq!(0, brainfuck.data_pointer());
        assert_eq!(&[0, 0, 0, 0], brainfuck.data(0..4));
    }

    #[test]
    fn instruction_greater_than() {
        let mut brainfuck = Brainfuck::new(">");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(1, brainfuck.data_pointer());
    }

    #[test]
    fn instruction_less_than() {
        let mut brainfuck = Brainfuck::new("<");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(0, brainfuck.data_pointer());
    }

    #[test]
    fn instruction_less_than_2() {
        let mut brainfuck = Brainfuck::new(">><");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(1, brainfuck.data_pointer());
    }

    #[test]
    fn instruction_plus() {
        let mut brainfuck = Brainfuck::new("+");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[1], brainfuck.data(0..1));
    }

    #[test]
    fn instruction_plus_2() {
        let mut brainfuck = Brainfuck::new("++>++>++");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[2, 2, 2], brainfuck.data(0..3));
    }

    #[test]
    fn instruction_minus() {
        let mut brainfuck = Brainfuck::new("-");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[254], brainfuck.data(0..1));
    }

    #[test]
    fn instruction_minus_2() {
        let mut brainfuck = Brainfuck::new("-->-->--");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[253, 253, 253], brainfuck.data(0..3));
    }

    #[test]
    fn instruction_dot() {
        let mut output: Vec<u8> = Vec::new();
        let mut brainfuck = Brainfuck::new(".");
        let result = brainfuck.run(&mut io::empty(), &mut output);

        assert_eq!((), result.unwrap());
        assert_eq!(vec!(0), output);
    }

    #[test]
    fn instruction_dot_2() {
        let mut output = Vec::new();
        let mut brainfuck = Brainfuck::new("+>++>+++.<.<.");
        let result = brainfuck.run(&mut io::empty(), &mut output);

        assert_eq!((), result.unwrap());
        assert_eq!(vec!(3, 2, 1), output);
    }

    #[test]
    fn instruction_comma() {
        let input = [5, 4, 3];
        let mut brainfuck = Brainfuck::new(",>,>,");
        let result = brainfuck.run(&mut input.as_ref(), &mut io::sink());

        assert_eq!((), result.unwrap());
        assert_eq!(&[5, 4, 3], brainfuck.data(0..3));
    }

    #[test]
    fn instruction_comma_2() {
        let input = [5, 4, 3];
        let mut output = Vec::new();
        let mut brainfuck = Brainfuck::new(",.>,.>,.");
        let result = brainfuck.run(&mut input.as_ref(), &mut output);

        assert_eq!((), result.unwrap());
        assert_eq!(vec!(5, 4, 3), output);
    }

    #[test]
    fn hello_world() {
        let mut brainfuck = Brainfuck::new(
            "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---\
            .+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.\n"
        );

        let mut output = Vec::new();
        let result = brainfuck.run(&mut io::empty(), &mut output);

        assert_eq!((), result.unwrap());
        assert_eq!(
            "Hello World!\n",
            String::from_utf8(output).unwrap()
        );
    }

    #[test]
    fn hello_world_complex() {
        let mut brainfuck = Brainfuck::new(
            ">++++++++[-<+++++++++>]<.>>+>-[+]++>++>+++[>[->+++<<+++>]<<]\
            >-----.>->+++..+++.>-.<<+[>[+>+]>>]<--------------.>>.+++.---\
            ---.--------.>+.>+."
        );

        let mut output = Vec::new();
        let result = brainfuck.run(&mut io::empty(), &mut output);

        assert_eq!((), result.unwrap());
        assert_eq!(
            "Hello World!\n",
            String::from_utf8(output).unwrap()
        );
    }
}
