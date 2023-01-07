use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{env, fs, io};
use std::env::{var, VarError};
use heimdall_errors::{implement_error, implement_error_with_kind};

#[derive(Debug, PartialEq)]
pub enum ErrorKind {
    IO(io::ErrorKind),
    Var,
}

#[derive(Debug)]
pub struct StructError {
    kind: ErrorKind,
    message: String,
}

impl StructError {
    pub fn new(kind: ErrorKind, message: String) -> Self {
        Self { kind, message}
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for StructError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Error for StructError {}

implement_error_with_kind!(StructError, io::Error, ErrorKind::IO);
implement_error!(StructError, VarError, ErrorKind::Var);


fn foo() -> Result<(), StructError> {
    let mut path = env::temp_dir();
    path.push("inexist.file.ñ");
    let content = fs::read_to_string(path)?;
    println!("{content}");

    Ok(())
}

fn bar() -> Result<(), StructError> {
    let value = var("INEXIST_ENV_VAR")?;
    println!("{value}");

    Ok(())
}

fn main() {
    let err = foo().unwrap_err();
    assert_eq!(err.kind(), &ErrorKind::IO(io::ErrorKind::NotFound));

    let err = bar().unwrap_err();
    assert_eq!(err.kind(), &ErrorKind::Var);
}

#[cfg(test)]
mod tests {
    #[test]
    fn works() {
        super::main();
    }
}
