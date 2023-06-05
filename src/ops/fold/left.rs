use crate::{Cons, HList, Nil};

/// Fold every element of the heterogenous list into an accumulator.
pub trait Fold<Accumulator, Folder>: HList {
    /// Folds every element into an accumulator
    /// by applying an operation via folder, returning the final result.
    ///
    /// This left folds the heterogenous list from its beginning to the end,
    /// or combines elements in a *left-associative* fashion.
    /// For a *right-associative* version of `fold()`, see [`RFold::rfold()`][rfold].
    ///
    /// [rfold]: crate::ops::RFold::rfold()
    ///
    /// # Examples
    ///
    /// You can fold the list if it is homogenous (all elements have the same type):
    ///
    /// ```
    /// use hlist2::{hlist, ops::Fold};
    ///
    /// let list = hlist!(1, 2, 3);
    /// let sum = list.fold(0, |acc, x| acc + x);
    /// assert_eq!(sum, 6);
    /// ```
    ///
    /// Folding of heterogenous list is possible with heterogenous list of closures as folder:
    ///
    /// ```
    /// use hlist2::{hlist, ops::Fold};
    ///
    /// let list = hlist!(1, false, 42.0);
    /// let folded = list.fold(
    ///     1.0,
    ///     hlist!(
    ///         |acc, i| i as f32 + acc,
    ///         |acc, b: bool| if !b && acc > 42.0 { 9000.0 } else { 0.0 },
    ///         |acc, f| f + acc,
    ///     ),
    /// );
    /// assert_eq!(folded, 42.0);
    /// ```
    fn fold(self, init: Accumulator, folder: Folder) -> Accumulator;
}

impl<A, F> Fold<A, F> for Nil {
    fn fold(self, init: A, _: F) -> A {
        init
    }
}

impl<A, F, Head, Tail> Fold<A, F> for Cons<Head, Tail>
where
    F: FnMut(A, Head) -> A,
    Tail: Fold<A, F>,
{
    fn fold(self, init: A, mut folder: F) -> A {
        let Cons(head, tail) = self;
        let init = folder(init, head);
        tail.fold(init, folder)
    }
}

impl<A, FHead, FTail, Head, Tail> Fold<A, Cons<FHead, FTail>> for Cons<Head, Tail>
where
    FHead: FnOnce(A, Head) -> A,
    Tail: Fold<A, FTail>,
{
    fn fold(self, init: A, folder: Cons<FHead, FTail>) -> A {
        let Cons(head, tail) = self;
        let Cons(folder_head, folder_tail) = folder;
        let init = folder_head(init, head);
        tail.fold(init, folder_tail)
    }
}
