# nop-attr
[![Crates.io Version](https://img.shields.io/crates/v/nop-attr)](https://crates.io/crates/nop-attr)
[![docs.rs](https://img.shields.io/docsrs/nop-attr)](https://docs.rs/nop-attr/latest/)

<!-- cargo-rdme start -->

A procedural-macro attribute that does nothing.

Any code marked with [`#[nop_attr::nop]`](macro@crate::nop) is
is passed through without modification.

Useful to annotate code with metadata.

## Example
```rust
pub use nop_attr::nop as example1;
pub use nop_attr::nop_noargs as example2;
pub use nop_attr::nop as example3;

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
