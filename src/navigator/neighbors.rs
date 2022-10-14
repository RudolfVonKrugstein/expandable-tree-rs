/** Structure containing neighboring information for a node.
*
* Internally (for the navigator) this structure is used with
* A=usize to store how neighboring and (therby navigaiton)
* information for nodes (which noce is the parent node i.E.).
*
* But it can also be used (and that is why it is public)
* for other neighboring information, like all the neighboring
* values of nodes.
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Neighbors<A> {
    // The node itself
    pub me: Option<A>,
    // The node above us
    pub parent: Option<A>,
    // The first of out child nodes
    pub first_child: Option<A>,
    // The prev node on the same level
    pub next_sibling: Option<A>,
    // The next node on the same level
    pub prev_sibling: Option<A>,
}

impl<A> Neighbors<A> {
    /** Map all the neigbors (which are not None)
     * to some other value using the provided function.
     *
     * # Arguments
     *
     * - f - The function used for mapping the values.
     *
     * # Result
     *
     * A neighbors structure, where the neighbors are exchanged
     * by the corresponding result of f.
     */
    pub fn map<B, F>(&self, f: F) -> Neighbors<B>
    where
        F: Fn(&A) -> B,
    {
        Neighbors {
            me: self.me.as_ref().map(&f),
            parent: self.parent.as_ref().map(&f),
            first_child: self.first_child.as_ref().map(&f),
            next_sibling: self.next_sibling.as_ref().map(&f),
            prev_sibling: self.prev_sibling.as_ref().map(&f),
        }
    }

    /** Same as map, but removes double "Option" for a
     * mapping function that returns an option.
     */
    pub fn map_and_then<B, F>(&self, f: F) -> Neighbors<B>
    where
        F: Fn(&A) -> Option<B>,
    {
        Neighbors {
            me: self.me.as_ref().and_then(&f),
            first_child: self.first_child.as_ref().and_then(&f),
            parent: self.parent.as_ref().and_then(&f),
            next_sibling: self.next_sibling.as_ref().and_then(&f),
            prev_sibling: self.prev_sibling.as_ref().and_then(&f),
        }
    }
}

impl<A> Neighbors<&A> {
    /** Clone the contained values (which have to be references)*/
    pub fn cloned(&self) -> Neighbors<A>
    where
        A: Clone,
    {
        Neighbors {
            me: self.me.cloned(),
            first_child: self.first_child.cloned(),
            parent: self.parent.cloned(),
            next_sibling: self.next_sibling.cloned(),
            prev_sibling: self.prev_sibling.cloned(),
        }
    }
}

impl Neighbors<usize> {
    /** Do `map_end_then` but don't use a mapping function but
     * take the values directly form an vec.*/
    pub fn map_and_then_with_values<'a, B>(&self, v: &'a Vec<Option<B>>) -> Neighbors<&'a B> {
        self.map_and_then(|i| v.get(*i).and_then(|v| v.as_ref()))
    }
}
