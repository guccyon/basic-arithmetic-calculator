use std::io::{stdin, stdout,BufRead, BufReader, Write};
use std::fmt;
use super::{lexer, parser, interpreter};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Error {
    Parser(parser::ParseError),
    Interpreter(interpreter::InterpretError),
}

impl From<parser::ParseError> for Error {
    fn from(e: parser::ParseError) -> Self {
        Error::Parser(e)
    }
}

impl From<interpreter::InterpretError> for Error {
    fn from(e: interpreter::InterpretError) -> Self {
        Error::Interpreter(e)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error was occurred")
    }
}

impl std::error::Error for parser::ParseError {}
impl std::error::Error for interpreter::InterpretError {}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::Error::*;
        match self {
            Parser(e) => Some(e),
            Interpreter(e) => Some(e)
        }
    }
}

pub struct Repl {
    parser: parser::Parser,
    interpreter: interpreter::Interpreter
}

impl Repl {
    pub fn new() -> Self {
        Self {
            parser: parser::Parser::new(),
            interpreter: interpreter::Interpreter::new()
        }
    }

    pub fn run(&self) {
        if let Err(e) = self.run_loop() {
            eprintln!("{}", e);
            self.run();
        }
    }

    fn run_loop(&self) -> Result<(), Error> {
        let stdin = stdin();
        let stdin = stdin.lock();
        let stdin = BufReader::new(stdin);
        let mut lines = stdin.lines();

        loop {
            self.prompt().unwrap();

            if let Some(Ok(line)) = lines.next() {
                let tokens = lexer::Lexer::new(&line).lex();

                let ast = self.parser.parse(tokens)?;

                let n = self.interpreter.eval(&ast)?;

                println!("{}", n);
            } else {
                break;
            }
        }
        Ok(())

    }

    fn prompt(&self) -> std::io::Result<()> {
        let stdout = stdout();
        let mut stdout = stdout.lock();
        stdout.write("> ".as_bytes())?;
        stdout.flush()
    }
}
