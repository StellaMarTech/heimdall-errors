//! Macro for auto impl From errors

/// Generate the [From<T>] trait implementation for an custom error struct with an specific
/// structure, with ErrorKind, and message attributes;
///
/// * Param 1: the custom error type,
/// * Param 2: the error type you wants use in From trait,
/// * Param 3: the corresponding ErrorKind.
///
/// # Example
/// ```
/// use std::fmt::{Display, Formatter, Result, Debug};
/// use std::io::Error;
/// use std::env::VarError;
/// use heimdall_errors::implement_error;
///
/// // First, create your ErrorKind
///#[derive(Debug, PartialEq, Copy, Clone)]
/// pub (crate) enum ErrorKind {
///     Io,
///     Env
/// }
///
/// impl ToString for ErrorKind {
///     fn to_string(&self) -> String {
///         format!("{:?}", &self)
///     }
/// }
///
/// // Next, create your Error struct
/// #[derive(Debug, PartialEq, Clone)]
/// pub (crate) struct MyErrorType {
///     kind: ErrorKind,
///     message: String
/// }
///
/// // Implement the Display trait
/// impl Display for MyErrorType {
///     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
///         write!(
///             f,
///             "kind: {} message: {}",
///             self.kind.to_string(),
///             self.message
///         )
///     }
/// }
///
/// // Generate implementations
/// implement_error!(MyErrorType, std::io::Error, ErrorKind::Io);
/// implement_error!(MyErrorType, VarError, ErrorKind::Env);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error!(MyErrorType, std::io::Error, ErrorKind::Io);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<std::io::Error> for MyErrorType {
///    fn from(err: std::io::Error) -> Self {
///        Self {
///            kind: ErrorKind::Io,
///            message: err.to_string(),
///        }
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_error {
    ($err:ident, $t: path, $kind: path) => {
        impl From<$t> for $err {
            fn from(error: $t) -> $err {
                $err {
                    kind: $kind,
                    message: error.to_string(),
                }
            }
        }
    };
}
