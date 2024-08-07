//! A procedural-macro attribute that does nothing.
//!
//! Any code marked with [`#[nop_attr::nop]`](macro@crate::nop) is
//! is passed through without modification.
//!
//! Useful to annotate code with metadata that is ignored atUseful to annotate code with metadata that is ignored atUseful to annotate code with metadata that is ignored atUseful to annotate code with metadata that is ignored at
//!
//! # Example
//! ```
//! pub use nop_attr::nop as example1;
//! pub use nop_attr::nop_noargs as example2;
//! pub use nop_attr::nop as example3;
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
//! ```
//!
extern crate proc_macro;

use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};


/// A procedural macro that does nothing to the annotated code,
/// and ignores any arguments.
///
/// Any annotated code is passed through unchanged.
///
/// # Example
/// ```
/// #[nop_attr::nop(this_is_ignored, 7)]
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
/// this behaves identically to the `#[nop_attr::nop]` attribute.
///
/// # Example
/// ```
/// #[nop_attr::nop_noargs]
/// fn foo() -> i32 {
///     7
/// }
/// assert_eq!(foo(), 7);
/// ```
///
/// ### Error if args passed
/// ```compile_fail
/// #[nop_attr::nop_noargs(foo)]
/// fn foo() -> i32 {
///     7
/// }
/// assert_eq!(foo(), 7);
/// ```
#[proc_macro_attribute]
pub fn nop_noargs(attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Some(first) = attr.into_iter().next() {
        compile_error(
            "Cannot give arguments to #[nop_noargs]",
            first.span(),
        ).into_iter().chain(item.into_iter()).collect()
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
            std::iter::once(set_span(
                TokenTree::Literal(Literal::string(msg))
            )).collect()
        ))),
        set_span(TokenTree::Punct(Punct::new(';', Spacing::Alone)))
    ].into_iter().collect()
}

