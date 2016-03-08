# An `unwrap!` macro for Rust.

The crate provides a macro, `unwrap!` which can be used to unwrap values of
type `Result` or `Option` (or any type that implements `VerboseUnwrap`). The
advantage of using this macro over the `.unwrap()` methods is that, on a panic,
it will print the file name, line number, column number, and function name of
where the macro was called from.

## Example

This code:

```
let x: Result<(), u32> = Err(123);
let y = unwrap!(x);
```

Panics with the following message:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Result::Err                                              !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
example.rs:2,9 in example_module::example_function

Err(
    123
)

```

`unwrap!` can also be called with an extra error message as an optional second
argument.

```
let x: Option<()> = None;
let y = unwrap!(x, "Oh no!");
```

Prints:

```

!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
!   unwrap! called on Option::None                                             !\n\
!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n\
example.rs:2,9 in example_module::example_function
Oh no!

```

