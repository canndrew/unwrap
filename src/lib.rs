use std::fmt::Debug;

/// Types which can be unwrapped and which may want to print a verbose error message when they are
/// unwrapped incorrectly. This trait is implemented for `Result` and `Option` as a replacement for
/// their inherent `unwrap` methods. This trait is intended to be used via this crate's `unwrap!`
/// macro.
pub trait VerboseUnwrap {
    /// The wrapped type.
    type Wrapped;

    /// Unwrap the value into it's inner type or panicks with an error message when the value
    /// cannot be unwrapped. This method is intended to be called via this crate's `unwrap!` macro.
    ///
    /// # Panics
    ///
    /// When the value cannot be unwrapped. Eg. on an `Err` or `None` value.
    ///
    /// # Arguments
    ///
    /// These arguments are used to print a useful diagnostic when the method panicks.
    /// 
    ///  * `message`: An optional message, printed alongside the rest of the info.
    ///  * `module_path`: The module path where this method is being called from. Eg.
    ///    `my_crate::my_module::my_function`
    ///  * `file`: The filename where this method is being called from.
    ///  * `line_number`: The line number where this method is being called from.
    ///  * `column`: The column number where this method is being called from
    fn verbose_unwrap(self, message: Option<&str>, module_path: &str, file: &str, line_number: u32, column: u32) -> Self::Wrapped;
}

impl<T, E: Debug> VerboseUnwrap for Result<T, E> {
    type Wrapped = T;

    fn verbose_unwrap(self, message: Option<&str>, module_path: &str, file: &str, line_number: u32, column: u32) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                // TODO(canndrew): As soon as impl specialisation lands specialise this to display
                // the error and it's chain of causes.
                /*
                let mut error_str = String::new();
                let mut error: &Error = &e;
                loop {
                    error_str.push_str(format!("{}\n", error));
                    error = match error.cause() {
                        Some(e) => e,
                        None => break,
                    }
                }
                */
               
                match message {
                    Some(m) => {
                        panic!("\n\
\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Result::Err                                              !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}\n\
{}\n\
\n\
{:#?}\n\
\n", file, line_number, column, module_path, m, Err::<(), E>(e));
                    },
                    None => {
                        panic!("\n\
\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Result::Err                                              !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}\n\
\n\
{:#?}\n\
\n", file, line_number, column, module_path, Err::<(), E>(e));
                    },
                }
            },
        }
    }
}

impl<T> VerboseUnwrap for Option<T> {
    type Wrapped = T;

    fn verbose_unwrap(self, message: Option<&str>, module_path: &str, file: &str, line_number: u32, column: u32) -> T {
        match self {
            Some(t) => t,
            None => {
                match message {
                    Some(m) => {
                        panic!("\n\
\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Option::None                                             !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}\n\
{}\n\
\n", file, line_number, column, module_path, m);
                    },
                    None => {
                        panic!("\n\
\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Option::None                                             !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}\n\
\n", file, line_number, column, module_path);
                    },
                }
            },
        }
    }
}

/// A replacement for calling `unwrap()` on a `Result` or `Option`.
///
/// This macro is intended to be used in all cases where one would `unwrap` a `Result` or `Option`
/// to deliberately panic in case of error, e.g. in test-cases. Such `unwrap`s don't give a precise
/// point of failure in the code and instead indicate some line number in the Rust core library.
/// This macro provides a precise point of failure and decorates the failure for easy viewing.
///
/// # Examples
///
/// ```
/// # #[macro_use]
/// # extern crate unwrap;
/// # fn main() {
/// let some_option = Some("Hello".to_string());
/// let string_length = unwrap!(some_option, "This is an optional user-supplied text.").len();
/// assert_eq!(string_length, 5);
/// # }
/// ```
#[macro_export]
macro_rules! unwrap(
    ($e:expr) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, None, module_path!(), file!(), line!(), column!())
    );
    ($e:expr, $message:expr) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, Some($message), module_path!(), file!(), line!(), column!())
    );
);

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_result_ok() {
        let result: Result<u32, u32> = Ok(32);
        let x = unwrap!(result);
        let y = unwrap!(result, "Here's a message");
        assert_eq!(x, 32);
        assert_eq!(y, 32);
    }

    #[test]
    #[should_panic]
    fn unwrap_result_err_message() {
        let result: Result<u32, u32> = Err(32);
        let _ = unwrap!(result, "Here's a message");
    }

    #[test]
    #[should_panic]
    fn unwrap_result_err_no_message() {
        let result: Result<u32, u32> = Err(32);
        let _ = unwrap!(result);
    }

    #[test]
    fn unwrap_result_some() {
        let option: Option<u32> = Some(32);
        let x = unwrap!(option);
        let y = unwrap!(option, "Here's a message");
        assert_eq!(x, 32);
        assert_eq!(y, 32);
    }

    #[test]
    #[should_panic]
    fn unwrap_result_none_message() {
        let option: Option<u32> = None;
        let _ = unwrap!(option, "Here's a message");
    }

    #[test]
    #[should_panic]
    fn unwrap_result_none_no_message() {
        let option: Option<u32> = None;
        let _ = unwrap!(option);
    }
}

