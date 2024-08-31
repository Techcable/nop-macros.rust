//! Procedural macros that do nothing, allowing attributes to be used as metadata.
//!
//! Any code marked with the [`#[nop_macros::nop]`](macro@crate::nop) attribute
//! is passed through without modification.
//! Similarly, using [`#[derive(nop_macros::NopDerive)]`](macro@crate::NopDerive)
//! on a type does not generate any code or implement any traits.
//!
//! Useful for source-code only metadata,
//! readable by tools but ignored at runtime.
//! I use this for configuring build-time source generation (see ["Use Cases"](#use-cases) below).
//!
//! All macros can be used multiple times and renamed with `pub use`.
//!
//! # Example
//! ```
//! pub use nop_macros::nop as example1;
//! pub use nop_macros::nop_noargs as example2;
//! pub use nop_macros::nop as example3;
//! pub use nop_macros::NopDerive as DeriveExample1;
//! pub use nop_macros::NopDerive as DeriveExample2;
//!
//! #[example1(ignored)]
//! #[example2]
//! pub fn foo() -> i32 {
//!     7
//! }
//!
//! #[example2]
//! const BAR: u32 = 42;
//!
//! #[example3(781)]
//! pub fn baz() -> i32 {
//!     18
//! }
//!
//! assert_eq!(foo(), 7);
//! assert_eq!(BAR, 42);
//! assert_eq!(baz(), 18);
//!
//! #[derive(Debug, DeriveExample1, DeriveExample2)]
//! struct Foo {
//!     bar: String
//! }
//! ```
//!
//! # Use Cases
//! I use this for generating source-code at build-time,
//! configured by attributes on rust code.
//!
//! An example of a build-time source generator that parses rust code is [cbindgen](https://github.com/mozilla/cbindgen).
//! However, that project uses comments for configruation, which I want to avoid.
//!
//! Build-time source generation is significantly more powerful and flexible than procedural macros.
//! Maktlad has a [blog post](https://matklad.github.io/2021/02/14/for-the-love-of-macros.html) on this subject.
//!
//! I recomend [genco](https://docs.rs/genco) instead of [quote](https://docs.rs/quote) for build-time quasiquoting.
//! It preserves whitespace and supports languages besides rust.
//!
//! I still use [syn](https://docs.rs/syn) for parsing rust code.
extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

/// A derive macro that does nothing,
/// and performs no validation.
///
/// Does not accept any attributes by default,
/// but those can be added using [`#[nop_macros::nop]`](macro@crate::nop).
///
/// ## Example
/// ```
/// #[derive(nop_macros::NopDerive)]
/// struct Foo {
///     foo: u32
/// }
///
/// // can be done twice!
/// #[derive(nop_macros::NopDerive, nop_macros::NopDerive)]
/// enum Bar {
///     Baz(String),
///     Nop,
/// }
/// ```
#[proc_macro_derive(NopDerive)]
pub fn nop_derive(_input: TokenStream) -> TokenStream {
    TokenStream::default()
}

/// A procedural macro that does nothing to the annotated code,
/// and ignores any arguments.
///
/// Any annotated code is passed through unchanged.
///
/// # Example
/// ```
/// #[nop_macros::nop(this_is_ignored, 7)]
/// pub fn foo() -> i32 {
///     7
/// }
/// assert_eq!(foo(), 7);
/// ```
#[proc_macro_attribute]
pub fn nop(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

/// A procedural macro that does nothing to the annotated code,
/// and accepts no arguments.
///
/// Gives an error if arguments are passed.
///
/// Aside from requiring no arguments,
/// this behaves identically to the `#[nop_macros::nop]` attribute.
///
/// # Example
/// ```
/// #[nop_macros::nop_noargs]
/// fn foo() -> i32 {
///     7
/// }
/// assert_eq!(foo(), 7);
/// ```
///
/// ### Error if args passed
/// ```compile_fail
/// #[nop_macros::nop_noargs(foo)]
/// fn foo() -> i32 {
///     7
/// }
/// assert_eq!(foo(), 7);
/// ```
#[proc_macro_attribute]
pub fn nop_noargs(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Some(first) = attr.into_iter().next() {
        compile_error("Cannot give arguments to #[nop_noargs]", first.span())
            .into_iter()
            .chain(item)
            .collect()
    } else {
        item
    }
}

fn compile_error(msg: &str, span: Span) -> TokenStream {
    let set_span = |mut tree: TokenTree| {
        tree.set_span(span);
        tree
    };
    [
        TokenTree::Ident(Ident::new("compile_error", span)),
        set_span(TokenTree::Punct(Punct::new('!', Spacing::Alone))),
        set_span(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            std::iter::once(set_span(TokenTree::Literal(Literal::string(msg)))).collect(),
        ))),
        set_span(TokenTree::Punct(Punct::new(';', Spacing::Alone))),
    ]
    .into_iter()
    .collect()
}
