use crate::{Cons, HList, Nil};

/// Right fold every element of the heterogenous list into an accumulator.
pub trait RFold<Accumulator, Folder>: HList {
    /// Folds every element into an accumulator from the back
    /// by applying an operation via folder, returning the final result.
    ///
    /// This is the reverse version of [`Fold::fold()`][fold]:
    /// it takes elements starting from the back of the heterogenous list.
    ///
    /// This right folds the heterogenous list from its end to the beginning,
    /// or combines elements in a *right-associative* fashion.
    /// For a *left-associative* version of `rfold()`, see [`Fold::fold()`][fold].
    ///
    /// [fold]: crate::ops::Fold::fold()
    ///
    /// # Examples
    ///
    /// You can right fold the list if it is homogenous (all elements have the same type):
    ///
    /// ```
    /// use hlist2::{hlist, ops::RFold};
    ///
    /// let list = hlist!(1, 2, 3, 4, 5);
    /// let zero = "0".to_string();
    /// let result = list.rfold(zero, |acc, x| {
    ///     format!("({x} + {acc})")
    /// });
    /// assert_eq!(result, "(1 + (2 + (3 + (4 + (5 + 0)))))")
    /// ```
    ///
    /// Folding of heterogenous list is possible with heterogenous list of closures as folder:
    ///
    /// ```
    /// use hlist2::{hlist, ops::RFold};
    ///
    /// let list = hlist!(1, false, 42.0);
    /// let folded = list.rfold(
    ///     1.0,
    ///     hlist!(
    ///         |acc, i| i as f32 + acc,
    ///         |acc, b: bool| if !b && acc > 42.0 { 9000.0 } else { 0.0 },
    ///         |acc, f| f + acc,
    ///     ),
    /// );
    /// assert_eq!(folded, 9001.0);
    /// ```
    fn rfold(self, init: Accumulator, f: Folder) -> Accumulator;
}

impl<A, F> RFold<A, F> for Nil {
    fn rfold(self, init: A, _: F) -> A {
        init
    }
}

impl<A, F, Head, Tail> RFold<A, F> for Cons<Head, Tail>
where
    F: FnMut(A, Head) -> A,
    Tail: for<'a> RFold<A, &'a mut F>,
{
    fn rfold(self, init: A, mut f: F) -> A {
        let Cons(head, tail) = self;
        let init = tail.rfold(init, &mut f);
        f(init, head)
    }
}

impl<A, FHead, FTail, Head, Tail> RFold<A, Cons<FHead, FTail>> for Cons<Head, Tail>
where
    FHead: FnOnce(A, Head) -> A,
    FTail: HList,
    Tail: RFold<A, FTail>,
{
    fn rfold(self, init: A, f: Cons<FHead, FTail>) -> A {
        let Cons(head, tail) = self;
        let Cons(f_head, f_tail) = f;
        let init = tail.rfold(init, f_tail);
        f_head(init, head)
    }
}
