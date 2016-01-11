use std::io;
use std::ops::Range;
use std::result;

const TAPE_SIZE: usize = 30_000;

pub type Result = result::Result<(), Error>;
pub type Position = (usize, usize);

pub struct Brainfuck<'a> {
    program: &'a [u8],
    ip: usize,
    tape: [u8; TAPE_SIZE],
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
            tape: [0; TAPE_SIZE],
            dp: 0,
            stack: Vec::new(),
            pos: (1, 1),
        }
    }

    #[allow(dead_code)]
    pub fn tape_pointer(&self) -> usize {
        self.dp
    }

    #[allow(dead_code)]
    pub fn tape(&self, range: Range<usize>) -> &[u8] {
        &self.tape[range]
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
                    if self.dp < self.tape.len() - 1 {
                        self.dp += 1;
                    }
                },
                Some('<') => {
                    self.dp = self.dp.checked_sub(1).unwrap_or(0);
                },
                Some('+') => {
                    let byte = self.get_byte().checked_add(1).unwrap_or(0);
                    self.set_byte(byte);
                },
                Some('-') => {
                    let byte = self.get_byte().checked_sub(1).unwrap_or(255);
                    self.set_byte(byte);
                },
                Some('.') => {
                    let _ = try!(
                        output
                            .write(&[self.get_byte()])
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
                    self.set_byte(buffer[0]);
                },
                Some('[') => {
                    if self.get_byte() == 0 {
                        self.advance_to_matching_paren();
                    } else {
                        self.push();
                    }
                },
                Some(']') => {
                    if self.get_byte() != 0 {
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

    #[inline(always)]
    fn set_byte(&mut self, byte: u8) {
        self.tape[self.dp] = byte;
    }

    #[inline(always)]
    fn get_byte(&self) -> u8 {
        self.tape[self.dp]
    }

    #[inline(always)]
    fn advance(&mut self) {
        self.ip += 1;
        self.pos.1 += 1;
    }

    #[inline(always)]
    fn current(&self) -> Option<char> {
        self.program
            .get(self.ip)
            .map(|byte| *byte as char)
    }

    #[inline(always)]
    fn pop(&mut self) {
        let _ = self.stack.pop();
    }

    #[inline(always)]
    fn push(&mut self) {
        self.stack.push((self.ip, self.pos));
    }

    #[inline(always)]
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

    #[inline(always)]
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

        assert_eq!(0, brainfuck.tape_pointer());
        assert_eq!(&[0, 0, 0, 0], brainfuck.tape(0..4));
    }

    #[test]
    fn instruction_greater_than() {
        let mut brainfuck = Brainfuck::new(">");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(1, brainfuck.tape_pointer());
    }

    #[test]
    fn instruction_less_than() {
        let mut brainfuck = Brainfuck::new("<");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(0, brainfuck.tape_pointer());
    }

    #[test]
    fn instruction_less_than_2() {
        let mut brainfuck = Brainfuck::new(">><");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(1, brainfuck.tape_pointer());
    }

    #[test]
    fn instruction_plus() {
        let mut brainfuck = Brainfuck::new("+");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[1], brainfuck.tape(0..1));
    }

    #[test]
    fn instruction_plus_2() {
        let mut brainfuck = Brainfuck::new("++>++>++");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[2, 2, 2], brainfuck.tape(0..3));
    }

    #[test]
    fn instruction_minus() {
        let mut brainfuck = Brainfuck::new("-");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[255], brainfuck.tape(0..1));
    }

    #[test]
    fn instruction_minus_2() {
        let mut brainfuck = Brainfuck::new("-->-->--");
        let result = brainfuck.run_pure();

        assert_eq!((), result.unwrap());
        assert_eq!(&[254, 254, 254], brainfuck.tape(0..3));
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
        assert_eq!(&[5, 4, 3], brainfuck.tape(0..3));
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
