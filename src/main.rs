use brainfuck::{Brainfuck, Error};
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

fn main() -> io::Result<()> {
    let mut args = env::args();

    let _ = args.next();
    let path = args.next().expect("pass file!");
    let mut program = String::new();

    let mut f = File::open(&Path::new(&path))?;
    f.read_to_string(&mut program)?;

    let mut stderr = io::stderr();
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut brainfuck = Brainfuck::new(&program);
    let result = brainfuck.run(&mut stdin, &mut stdout);

    match result {
        Err(Error::ReadError(err)) => {
            writeln!(stderr, "Read error: {:?}.", err)?;
        }
        Err(Error::WriteError(ref err))
            if err.kind() != io::ErrorKind::BrokenPipe =>
        {
            writeln!(stderr, "Write error: {:?}.", err)?;
        }
        Err(Error::UnbalancedParens) => {
            writeln!(stderr, "Unbalanced parens found.")?;
        }
        _ => {}
    }

    Ok(())
}
