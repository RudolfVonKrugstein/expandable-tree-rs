mod navigator;
mod tree;
mod values;

pub use tree::Builder;
pub use tree::FlatTree;
pub use tree::RefFlatTree;

pub type FlatTree1<A> = FlatTree<Vec<A>>;
pub type FlatTree2<A, B> = FlatTree<values::Zip2Values<A, B>>;
pub type FlatTree3<A, B, C> = FlatTree<values::Zip3Values<A, B, C>>;
pub type RefFlatTree1<'a, A> = RefFlatTree<'a, Vec<A>>;
pub type RefFlatTree2<'a, A, B> = RefFlatTree<'a, values::Zip2Values<A, B>>;
pub type RefFlatTree3<'a, A, B, C> = RefFlatTree<'a, values::Zip3Values<A, B, C>>;

#[cfg(test)]
mod tests;
