use std::{env, fs, io};
use std::env::{var, VarError};
use std::error::Error;
use std::fmt::{Display, Formatter};
use heimdall_errors::{implement_error_in_enum, implement_string_error_in_enum};

#[derive(Debug, PartialEq)]
pub enum EnumError {
    IO(String),
    Var(VarError),
    Other,
}

impl Display for EnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other => write!(f, "Unknown error"),
            EnumError::IO(err) => Display::fmt(err, f),
            EnumError::Var(err) => Display::fmt(err, f),
        }
    }
}

implement_string_error_in_enum!(EnumError, io::Error, EnumError::IO);
implement_error_in_enum!(EnumError, VarError, EnumError::Var);


impl Error for EnumError {}

fn foo() -> Result<(), EnumError> {
    let mut path = env::temp_dir();
    path.push("inexist.file.ñ");
    let content = fs::read_to_string(path)?;
    println!("{content}");

    Ok(())
}

fn bar() -> Result<(), EnumError> {
    let value = var("INEXIST_ENV_VAR")?;
    println!("{value}");

    Ok(())
}

fn main() {
    let err = foo().unwrap_err();
    assert_eq!(err, EnumError::IO("No such file or directory (os error 2)".into()));

    let err = bar().unwrap_err();
    assert_eq!(err, EnumError::Var(VarError::NotPresent));
}