# nop-macros
[![Crates.io Version](https://img.shields.io/crates/v/nop-macros)](https://crates.io/crates/nop-macros)
[![docs.rs](https://img.shields.io/docsrs/nop-macros)](https://docs.rs/nop-macros/latest/)

<!-- cargo-rdme start -->

Procedural macros that do nothing, allowing attributes to be used as metadata.

Any code marked with the [`#[nop_macros::nop]`](macro@crate::nop) attribute
is passed through without modification.
Similarly, using [`#[derive(nop_macros::NopDerive)]`](macro@crate::NopDerive)
on a type does not generate any code or implement any traits.

Useful for source-code only metadata,
readable by tools but ignored at runtime.
I use this for configuring build-time source generation (see ["Use Cases"](#use-cases) below).

All macros can be used multiple times and renamed with `pub use`.

## Example
```rust
pub use nop_macros::nop as example1;
pub use nop_macros::nop_noargs as example2;
pub use nop_macros::nop as example3;
pub use nop_macros::NopDerive as DeriveExample1;
pub use nop_macros::NopDerive as DeriveExample2;

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

#[derive(Debug, DeriveExample1, DeriveExample2)]
struct Foo {
    bar: String
}
```

## Use Cases
I use this for generating source-code at build-time,
configured by attributes on rust code.

An example of a build-time source generator that parses rust code is [cbindgen](https://github.com/mozilla/cbindgen).
However, that project uses comments for configruation, which I want to avoid.

Build-time source generation is significantly more powerful and flexible than procedural macros.
Maktlad has a [blog post](https://matklad.github.io/2021/02/14/for-the-love-of-macros.html) on this subject.

I recomend [genco](https://docs.rs/genco) instead of [quote](https://docs.rs/quote) for build-time quasiquoting.
It preserves whitespace and supports languages besides rust.

I still use [syn](https://docs.rs/syn) for parsing rust code.

## Limitations
Currently, the [`#[nop]`](macro@crate::nop) attribute cannot be used on fields.

The following code fails to compile:
```rust
pub use nop_macros::nop as custom;

pub struct Foo {
    #[custom]
    bar: String,
}
```

<!-- cargo-rdme end -->
