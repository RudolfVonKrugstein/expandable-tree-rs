#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<A> {
    pub me: Option<A>,
    pub parent: Option<A>,
    pub next_sibling: Option<A>,
    pub prev_sibling: Option<A>,
}

impl<A> Neighbors<A> {

    pub fn map_and_then<B, F>(&self, f: F) -> Neighbors<B>
        where
            F: Fn(&A) -> Option<B> {
        Neighbors {
            me: self.me.as_ref().and_then(&f),
            parent: self.parent.as_ref().and_then(&f),
            next_sibling: self.next_sibling.as_ref().and_then(&f),
            prev_sibling: self.prev_sibling.as_ref().and_then(&f)
        }
    }
}

impl<A> Neighbors<&A> {
    pub fn cloned(&self) -> Neighbors<A>
        where
            A: Clone
    {
        Neighbors {
            me: self.me.cloned(),
            parent: self.parent.cloned(),
            next_sibling: self.next_sibling.cloned(),
            prev_sibling: self.prev_sibling.cloned(),
        }
    }
}

impl Neighbors<usize> {
    pub fn map_and_then_with_values<'a, B>(&self, v: &'a Vec<Option<B>>) -> Neighbors<&'a B> {
        self.map_and_then(
            |i| v.get(*i).and_then(|v| v.as_ref())
        )
    }
}
