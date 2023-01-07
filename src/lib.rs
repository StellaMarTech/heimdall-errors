//! Macros for auto impl [From<T>] for errors

/// Implement the [`From`] trait for an struct with kind structure
///
/// # Params
/// ```ignore
/// implement_in_error_in_struct($struct_error, $error_type, $error_kind);
/// ```
/// # Example
/// ```
/// use heimdall_errors::implement_error;
/// use std::env::VarError;
///
/// pub enum ErrorKind {
///     Var,
/// }
///
/// pub struct StructError {
///     kind: ErrorKind,
///     message: String,
/// }
///
/// // Implement From<VarError> for StructError.
/// implement_error!(StructError, VarError, ErrorKind::Var);
///
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error!(StructError, VarError, ErrorKind::Var);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<VarError> for StructError {
///    fn from(err: VarError) -> Self {
///        Self {
///            kind: ErrorKind::Var,
///            message: err.to_string(),
///        }
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_error {
    ($struct_error:ident, $error_type: path, $error_kind: path) => {
        impl From<$error_type> for $struct_error {
            fn from(error: $error_type) -> $struct_error {
                $struct_error {
                    kind: $error_kind,
                    message: error.to_string(),
                }
            }
        }
    };
}

/// Implement the [`From`] trait for an struct with an specific structure, with ErrorKind,
/// and message attributes.
///
/// # Params
/// ```ignore
/// implement_in_error_in_struct($struct_error, $error_type, $error_kind);
/// ```
/// # Example
/// ```
/// use heimdall_errors::implement_error_with_kind;
/// use std::io;
///
/// pub enum ErrorKind {
///     IO(io::ErrorKind),
/// }
///
/// pub struct StructError {
///     kind: ErrorKind,
///     message: String,
/// }
/// // Implement From<io::Error> for StructError.
/// implement_error_with_kind!(StructError, io::Error, ErrorKind::IO);
///
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error_with_kind!(StructError, io::Error, ErrorKind::IO);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<io::Error> for StructError {
///    fn from(err: io::Error) -> Self {
///        Self {
///            kind: ErrorKind::Io(err.kind()),
///            message: err.to_string(),
///        }
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_error_with_kind {
    ($err:ident, $t: path, $kind: path) => {
        impl From<$t> for $err {
            fn from(error: $t) -> $err {
                $err {
                    kind: $kind(error.kind().clone()),
                    message: error.to_string(),
                }
            }
        }
    };
}

/// Implement the [`From`] trait for an enum.
///
/// **WARNING**: you might prefer to use [thiserror] instead of this macro.
///
/// # Params
/// ```ignore
///     implement_error_in_enum!($type_, $err_type, $enum_variant);
/// ```
///
/// # Example
///```
/// use std::fmt::Debug;
/// use std::io::Error;
/// use heimdall_errors::implement_error_in_enum;
///
///
/// pub (crate) enum EnumError {
///     IO(Error)
/// }
///
///
/// // Implement From<Error> for StructError.
/// implement_error_in_enum!(EnumError, Error, EnumError::IO);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error_in_enum!(EnumError, Error, EnumError::IO);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<Error> for EnumError {
///    fn from(err: Error) -> Self {
///        EnumError::Io(err)
///     }
/// }
/// ```
///
/// [thiserror]:https://crates.io/crates/thiserror
#[macro_export]
macro_rules! implement_error_in_enum {
    ($enum_error:ident, $err_type: path, $enum_variant: path) => {
        impl From<$err_type> for $enum_error {
            fn from(error: $err_type) -> $enum_error {
                $enum_variant(error)
            }
        }
    };
}

/// Generate the [From<T>] trait implementation for an custom enum error using [ToString] trait.
/// # Params
/// ```ignore
///     implement_error_in_enum!($enum_error, $err_type, $enum_variant);
/// ```
/// # Example
/// ```
/// use std::fmt::Debug;
/// use heimdall_errors::implement_string_error_in_enum;
///
///
/// pub (crate) enum EnumError {
///     IO(String)
/// }
///
///
/// // Implement From<Error> for StructError.
/// implement_string_error_in_enum!(EnumError, std::io::Error, EnumError::IO);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_string_error_in_enum!(EnumError, std::io::Error, EnumError::IO);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<std::io::Error> for EnumError {
///    fn from(err: std::io::Error) -> Self {
///        EnumError::Io(err.to_string())
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_string_error_in_enum {
    ($enum_error:ident, $err_type: path, $enum_variant: path) => {
        impl From<$err_type> for $enum_error {
            fn from(error: $err_type) -> $enum_error {
                $enum_variant(error.to_string())
            }
        }
    };
}

/// Implement the [`From`] trait for an struct with an specific structure.
///
/// # Params
/// ```ignore
/// implement_in_error_in_struct($struct_error, $err_type, $kind);
/// ```
/// # Example
/// ```
/// use heimdall_errors::implement_in_error_in_struct;
///
/// pub enum ErrorKind{
///     IO,
/// }
/// pub struct StructError {
///     kind: ErrorKind,
///     message: String,
///     source: Option<Box<dyn std::error::Error>>
/// }
///
/// // Implement From<std::io::Error> for StructError.
/// implement_in_error_in_struct!(StructError, std::io::Error, ErrorKind::IO);
///
/// ```
#[macro_export]
macro_rules! implement_in_error_in_struct {
    ($struct_error:ident, $err_type: path, $kind: path) => {
        impl From<$err_type> for $struct_error {
            fn from(err: $err_type) -> Self {
                Self {
                    kind: $kind,
                    message: err.to_string(),
                    source: Some(Box::new(err)),
                }
            }
        }
    };
}