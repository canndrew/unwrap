use std::fmt::Debug;

pub trait VerboseUnwrap {
    type Wrapped;

    fn verbose_unwrap(self, module_path: &str, file: &str, line_number: u32, column: u32) -> Self::Wrapped;
}

impl<T, E: Debug> VerboseUnwrap for Result<T, E> {
    type Wrapped = T;

    fn verbose_unwrap(self, module_path: &str, file: &str, line_number: u32, column: u32) -> T {
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
                
                panic!("\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Result::Err                                              !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}\n\
{:#?}",
                    file, line_number, column, module_path, e);
            },
        }
    }
}

impl<T> VerboseUnwrap for Option<T> {
    type Wrapped = T;

    fn verbose_unwrap(self, module_path: &str, file: &str, line_number: u32, column: u32) -> T {
        match self {
            Some(t) => t,
            None => {
                panic!("\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Option::None                                             !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
{}:{},{} in {}",
                    file, line_number, column, module_path);
            },
        }
    }
}

macro_rules! unwrap(
    ($e:expr) => {
        $crate::VerboseUnwrap::verbose_unwrap($e, module_path!(), file!(), line!(), column!())
    }
);

#[cfg(test)]
mod tests {
    #[test]
    fn unwrap_result_ok() {
        let result: Result<u32, u32> = Ok(32);
        let x = unwrap!(result);
        assert_eq!(x, 32);
    }

    #[test]
    #[should_panic]
    fn unwrap_result_err() {
        let result: Result<u32, u32> = Err(32);
        let _ = unwrap!(result);
    }

    #[test]
    fn unwrap_result_some() {
        let option: Option<u32> = Some(32);
        let x = unwrap!(option);
        assert_eq!(x, 32);
    }

    #[test]
    #[should_panic]
    fn unwrap_result_none() {
        let option: Option<u32> = None;
        let _ = unwrap!(option);
    }
}

