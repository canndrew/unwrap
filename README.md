# An `unwrap!` macro for Rust.

The crate provides two macros, `unwrap!` and `unwrap_err!`. The former can be
used to unwrap values of type `Result` or `Option` (or any type that implements
`VerboseUnwrap`) and is comparable to calling `unwrap()`. The latter can be used
to unwrap an error from a `Result` (or any type that implements
`VerboseUnwrapErr`) and is comparable to calling `unwrap_err()`.

The advantage of using these macros over the `.unwrap()`, `.expect()`,
`.unwrap_err()` or `.expect_err()` methods is that, on a panic, they will print
the file name, line number, column number, and function name of where the macro
was called from.

[Documentation](https://docs.rs/unwrap)

## Example

This code:

```rust
let x: Result<(), u32> = Err(123);
let y = unwrap!(x);
```

Panics with the following message:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
!   unwrap! called on Result::Err                                              !
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
example.rs:2,9 in example_module::example_function

Err(123)

```

`unwrap!` can also be called with an optional error message. This is supplied
as a format string and arguments.

```rust
let x: Option<()> = None;
let y = unwrap!(x, "Oh no! {}", 123);
```

Prints:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
!   unwrap! called on Option::None                                             !
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
example.rs:2,9 in example_module::example_function
Oh no! 123

```

Similarly, for `unwrap_err!` this code:

```rust
let x: Result<u32, ()> = Ok(456);
let y = unwrap_err!(x);
```

Panics with the following message:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
!   unwrap_err! called on Result::Ok                                           !
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
example.rs:2,9 in example_module::example_function

Ok(456)

```

## Implementation

The unwrap crate provides a trait for types which can be unwrapped.

```rust
trait VerboseUnwrap {
    type Wrapped;
    fn verbose_unwrap(self, message: Option<std::fmt::Arguments>,
                            module_path: &str,
                            file: &str,
                            line_number: u32,
                            column: u32) -> Self::Wrapped;
}
```

This is implemented by both `Result` and `Option`. The `unwrap!` macro simply
calls this trait method:

```rust
macro_rules! unwrap(
    ($e:expr) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, None, module_path!(), file!(), line!(), column!())
    );
    ($e:expr, $($arg:tt)*) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, Some(format_args!($($arg)*)), module_path!(), file!(), line!(), column!())
    );
);
```

Likewise there's a trait for types which can have inner error types unwrapped.

```rust
pub trait VerboseUnwrapErr {
    type Wrapped;
    fn verbose_unwrap_err(self, message: Option<Arguments>,
                                module_path: &str,
                                file: &str,
                                line_number: u32,
                                column: u32) -> Self::Wrapped;
}
```

This is implemented by `Result`, and the `unwrap_err!` macro calls this trait
method:

```rust
macro_rules! unwrap_err(
    ($e:expr) => (
        $crate::VerboseUnwrapErr::verbose_unwrap_err($e, None, module_path!(), file!(), line!(), column!())
    );
    ($e:expr, $($arg:tt)*) => (
        $crate::VerboseUnwrapErr::verbose_unwrap_err($e, Some(format_args!($($arg)*)), module_path!(), file!(), line!(), column!())
    );
);
```

## Usage

Add this to your dependencies in `Cargo.toml`

```
unwrap = "~1.1.0"
```

Then import it using `#[macro_use]`

```
#[macro_use]
extern crate unwrap;
```
