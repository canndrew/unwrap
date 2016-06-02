# An `unwrap!` macro for Rust.

The crate provides a macro, `unwrap!` which can be used to unwrap values of
type `Result` or `Option` (or any type that implements `VerboseUnwrap`). The
advantage of using this macro over the `.unwrap()`/`.expect()` methods is that,
on a panic, it will print the file name, line number, column number, and
function name of where the macro was called from.

## Example

This code:

```
let x: Result<(), u32> = Err(123);
let y = unwrap!(x);
```

Panics with the following message:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
!   unwrap! called on Result::Err                                              !
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
example.rs:2,9 in example_module::example_function

Err(
    123
)

```

`unwrap!` can also be called with an optional error message. This is supplied
as a format string and arguments.

```
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

## Implementation

The unwrap crate provides a trait for types which can be unwrapped.

```
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

```
macro_rules! unwrap(
    ($e:expr) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, None, module_path!(), file!(), line!(), column!())
    );
    ($e:expr, $($arg:tt)*) => (
        $crate::VerboseUnwrap::verbose_unwrap($e, Some(format_args!($($arg)*)), module_path!(), file!(), line!(), column!())
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

