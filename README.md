# nop-macros
[![Crates.io Version](https://img.shields.io/crates/v/nop-macros)](https://crates.io/crates/nop-macros)
[![docs.rs](https://img.shields.io/docsrs/nop-macros)](https://docs.rs/nop-macros/latest/)

<!-- cargo-rdme start -->

Procedural macros that do nothing, allowing attributes to be used as metadata.

Any code marked with the [`#[nop_macros::nop]`](macro@crate::nop) attribute
is passed through without modification.

Useful for source-code only metadata,
readable by tools but ignored at runtime.

## Example
```rust
pub use nop_macros::nop as example1;
pub use nop_macros::nop_noargs as example2;
pub use nop_macros::nop as example3;

#[example1(ignored)]
#[example2]
pub fn foo() -> i32 {
    7
}

#[example2]
const BAR: u32 = 42;

#[example3(781)]
pub fn baz() -> i32 {
    18
}

assert_eq!(foo(), 7);
assert_eq!(BAR, 42);
assert_eq!(baz(), 18);
```

<!-- cargo-rdme end -->
