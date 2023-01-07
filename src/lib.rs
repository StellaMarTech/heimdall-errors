//! Macros for auto impl [From<T>] for errors

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

/// Generate the [From<T>] trait implementation for an custom error struct with an specific
/// structure, with ErrorKind, and message attributes. Use only if you want recovery the ErrorKind.
///
/// **Warning**: This macro use the `kind()` method. Make sure that the error implemented this method.
///
/// * Param 1: the custom error type,
/// * Param 2: the error type you wants use in From trait,
/// * Param 3: the corresponding ErrorKind.
///
/// # Example
/// ```
/// use std::fmt::{Display, Formatter, Result, Debug};
/// use std::io::Error;
/// use heimdall_errors::implement_error_with_kind;
///
/// // First, create your ErrorKind
///#[derive(Debug, PartialEq, Clone)]
/// pub (crate) enum ErrorKind {
///     Io(std::io::ErrorKind),
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
/// // Generate implementation
/// implement_error_with_kind!(MyErrorType, std::io::Error, ErrorKind::Io);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error_with_kind!(MyErrorType, std::io::Error, ErrorKind::Io);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<std::io::Error> for MyErrorType {
///    fn from(err: std::io::Error) -> Self {
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

/// Generate the [From<T>] trait implementation for an custom enum error.
/// # Usage
/// ```ignore
///     implement_error_in_enum!($type_, $err_type, $enum_variant);
/// ```
/// * **$type_:** the enum error type,
/// * **$error_type:** the value [T] in [From<T>] trait, this type must implement [ToString] trait,
/// * **$enum_variant:** the enum error type variant.
///
/// # Example
/// ```
/// use std::fmt::Debug;
/// use std::io::Error;
/// use heimdall_errors::implement_error_in_enum;
///
///
/// // First, create your Error type
/// #[derive(Debug)]
/// pub (crate) enum MyErrorType {
///     Io(Error)
/// }
///
///
/// // Generate implementation
/// implement_error_in_enum!(MyErrorType, std::io::Error, MyErrorType::Io);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_error_in_enum!(MyErrorType, std::io::Error, MyErrorType::Io);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<std::io::Error> for MyErrorType {
///    fn from(err: std::io::Error) -> Self {
///        MyErrorType::Io(err)
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_error_in_enum {
    ($type_:ident, $err_type: path, $enum_variant: path) => {
        impl From<$err_type> for $type_ {
            fn from(error: $err_type) -> $type_ {
                $enum_variant(error)
            }
        }
    };
}

/// Generate the [From<T>] trait implementation for an custom enum error using [ToString] trait.
/// # Usage
/// ```ignore
///     implement_error_in_enum!($type_, $err_type, $enum_variant);
/// ```
/// * **$type_:** the enum error type,
/// * **$error_type:** the value [T] in [From<T>] trait, this type must implement [ToString] trait,
/// * **$enum_variant:** the enum error type variant.
///
/// # Example
/// ```
/// use std::fmt::Debug;
/// use std::io::Error;
/// use heimdall_errors::implement_string_error_in_enum;
///
///
/// // First, create your Error type
/// #[derive(Debug)]
/// pub (crate) enum MyErrorType {
///     Io(String)
/// }
///
///
/// // Generate implementation
/// implement_string_error_in_enum!(MyErrorType, std::io::Error, MyErrorType::Io);
/// ```
///
///# Code generated
/// The code
/// ```ignore
/// implement_string_error_in_enum!(MyErrorType, std::io::Error, MyErrorType::Io);
/// ```
///
/// generates the next code
///
///```ignore
/// impl From<std::io::Error> for MyErrorType {
///    fn from(err: std::io::Error) -> Self {
///        MyErrorType::Io(err.to_string())
///     }
/// }
/// ```
#[macro_export]
macro_rules! implement_string_error_in_enum {
    ($type_:ident, $err_type: path, $enum_variant: path) => {
        impl From<$err_type> for $type_ {
            fn from(error: $err_type) -> $type_ {
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