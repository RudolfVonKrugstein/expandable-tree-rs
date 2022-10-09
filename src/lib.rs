mod navigator;
mod tree;
mod values;

pub use tree::Builder;
pub use tree::FlatTree;
pub use tree::RefFlatTree;

pub type FlatTreeVec<A> = FlatTree<Vec<A>>;
pub type RefFlatTreeVec<'a, A> = RefFlatTree<'a, Vec<A>>;

#[cfg(test)]
mod tests;
