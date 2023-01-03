use crate::{Cons, HList, Nil};

/// Merge two heterogenous lists into a single heterogenous list of pairs.
pub trait Zip<Other>: HList
where
    Other: HList,
{
    /// Type of new heterogenous list after merging.
    type Output: HList;

    /// Merges, or 'zips up' two heterogenous lists into a single heterogenous list of pairs.
    ///
    /// It returns a new heterogenous list where the first element comes from the first list,
    /// and the second element comes from the second list.
    ///
    /// In other words, it zips two lists together, into a single one.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Zip};
    ///
    /// let first = hlist![1, 2, 3];
    /// let second = hlist![4, 5, 6];
    ///
    /// let zipped = first.zip(second);
    /// assert_eq!(zipped, hlist![(1, 4), (2, 5), (3, 6)]);
    /// ```
    fn zip(self, other: Other) -> Self::Output;
}

impl Zip<Nil> for Nil {
    type Output = Nil;

    fn zip(self, _: Nil) -> Self::Output {
        self
    }
}

impl<Head, Tail, OHead, OTail> Zip<Cons<OHead, OTail>> for Cons<Head, Tail>
where
    Tail: Zip<OTail>,
    OTail: HList,
{
    type Output = Cons<(Head, OHead), Tail::Output>;

    fn zip(self, other: Cons<OHead, OTail>) -> Self::Output {
        let Cons(head, tail) = self;
        let Cons(o_head, o_tail) = other;
        Cons((head, o_head), tail.zip(o_tail))
    }
}
