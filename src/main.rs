extern crate brainfuck;

use brainfuck::{Brainfuck, Error};
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    let mut args = env::args();

    let _ = args.next();
    let path = args.next().expect("pass file!");
    let mut program = String::new();

    File::open(&Path::new(&path))
        .expect("couldn't open file")
        .read_to_string(&mut program)
        .expect("couldn't read from file");

    let mut stderr = io::stderr();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut brainfuck = Brainfuck::new(&program);
    let result = brainfuck.run(&mut stdin, &mut stdout);

    match result {
        Err(Error::ReadError(err)) =>
            writeln!(stderr, "Read error: {:?}.", err).unwrap(),
        Err(Error::WriteError(ref err)) if err.kind() != io::ErrorKind::BrokenPipe =>
            writeln!(stderr, "Write error: {:?}.", err).unwrap(),
        Err(Error::UnbalancedParens) =>
            writeln!(stderr, "Unbalanced parens found.").unwrap(),
        _ => {}
    }
}
