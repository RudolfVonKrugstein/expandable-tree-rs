/*! The navigator is basically a tree
without values.
It implements the structure and logic
on how the tree is build and how nodes find
there parents as well as their children.

How the navigator does this, is an implementation detail.
Currently it stores an vector of neighboring information,
but don't be sure that this documentation changes when
that is changed.
*/

mod builder;
mod navigator_impl;
mod neighbors;

// We use the navigator and builder only in this crate!
pub use builder::Builder;
pub use navigator_impl::Navigator;
pub use neighbors::Neighbors;
