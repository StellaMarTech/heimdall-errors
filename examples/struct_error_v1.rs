use std::error::Error;
use std::fmt::{Display, Formatter};
use std::{env, fs, io};
use std::env::{var, VarError};
use heimdall_errors::implement_in_error_in_struct;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    IO,
    Var,
}

#[derive(Debug)]
pub struct StructError {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn Error>>
}

impl StructError {
    pub fn new(kind: ErrorKind, message: String, source: Option<Box<dyn Error>>) -> Self {
        Self { kind, message, source }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn source(&self) -> Option<&Box<dyn Error>> {
        self.source.as_ref()
    }
}

impl Display for StructError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.message, f)
    }
}

impl Error for StructError {}

implement_in_error_in_struct!(StructError, io::Error, ErrorKind::IO);
implement_in_error_in_struct!(StructError, VarError, ErrorKind::Var);


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
    assert_eq!(err.kind(), ErrorKind::IO);

    let err = bar().unwrap_err();
    assert_eq!(err.kind(), ErrorKind::Var);
}