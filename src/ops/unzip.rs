use crate::{Cons, HList, Nil};

use super::Pair;

/// Convert a heterogenous list of pairs into a pair of heterogenous lists.
pub trait Unzip: HList {
    /// Type of the first heterogenous list from the resulting pair.
    type First: HList;
    /// Type of the second heterogenous list from the resulting pair.
    type Second: HList;

    /// Converts a heterogenous list of pairs into a pair of heterogenous lists.
    ///
    /// It consumes an entire list of pairs, producing two lists:
    /// one from the left elements of the pairs, and one from the right elements.
    ///
    /// This method is, in some sense, the opposite of [`Zip::zip`][zip] method.
    ///
    /// [zip]: crate::ops::Zip::zip()
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Unzip};
    ///
    /// let list = hlist![(1, 2), (3, 4), (5, 6)];
    /// let (first, second) = list.unzip();
    ///
    /// assert_eq!(first, hlist![1, 3, 5]);
    /// assert_eq!(second, hlist![2, 4, 6]);
    /// ```
    fn unzip(self) -> (Self::First, Self::Second);
}

impl Unzip for Nil {
    type First = Nil;
    type Second = Nil;

    fn unzip(self) -> (Self::First, Self::Second) {
        (self, self)
    }
}

impl<Head, Tail> Unzip for Cons<Head, Tail>
where
    Head: Pair,
    Tail: Unzip,
{
    type First = Cons<Head::First, Tail::First>;
    type Second = Cons<Head::Second, Tail::Second>;

    fn unzip(self) -> (Self::First, Self::Second) {
        let Cons(head, tail) = self;
        let (head_first, head_second) = head.destruct();
        let (tail_first, tail_second) = tail.unzip();

        let first = Cons(head_first, tail_first);
        let second = Cons(head_second, tail_second);
        (first, second)
    }
}
