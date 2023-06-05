use crate::{Cons, HList, Nil};

use super::{FoldFn, Folder};

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
    ///
    /// Or with special implementation of [folder function](FoldFn):
    ///
    /// ```
    /// use hlist2::{
    ///     hlist,
    ///     ops::{FoldFn, Folder, RFold},
    /// };
    ///
    /// struct MyFoldFn;
    ///
    /// impl FoldFn<f32, i32> for MyFoldFn {
    ///     fn fold(&mut self, acc: f32, i: i32) -> f32 {
    ///         i as f32 + acc
    ///     }
    /// }
    /// impl FoldFn<f32, bool> for MyFoldFn {
    ///     fn fold(&mut self, acc: f32, b: bool) -> f32 {
    ///         if !b && acc > 42.0 { 9000.0 } else { 0.0 }
    ///     }
    /// }
    /// impl FoldFn<f32, f32> for MyFoldFn {
    ///     fn fold(&mut self, acc: f32, f: f32) -> f32 {
    ///         f + acc
    ///     }
    /// }
    ///
    /// let list = hlist!(1, false, 42.0);
    /// let folded = list.rfold(8918.0, Folder(MyFoldFn));
    /// assert_eq!(folded, 9001.0);
    /// ```
    fn rfold(self, init: Accumulator, folder: Folder) -> Accumulator;
}

impl<A, F> RFold<A, F> for Nil {
    fn rfold(self, init: A, _: F) -> A {
        init
    }
}

impl<A, F, Head, Tail> RFold<A, F> for Cons<Head, Tail>
where
    F: FnMut(A, Head) -> A,
    Tail: RFoldWithFolder<A, F>,
{
    fn rfold(self, init: A, folder: F) -> A {
        let Cons(head, tail) = self;
        let (init, mut folder) = tail.rfold_with_folder(init, folder);
        folder(init, head)
    }
}

impl<A, FHead, FTail, Head, Tail> RFold<A, Cons<FHead, FTail>> for Cons<Head, Tail>
where
    FHead: FnOnce(A, Head) -> A,
    Tail: RFold<A, FTail>,
{
    fn rfold(self, init: A, folder: Cons<FHead, FTail>) -> A {
        let Cons(head, tail) = self;
        let Cons(folder_head, folder_tail) = folder;
        let init = tail.rfold(init, folder_tail);
        folder_head(init, head)
    }
}

impl<A, F, Head, Tail> RFold<A, Folder<F>> for Cons<Head, Tail>
where
    F: FoldFn<A, Head>,
    Tail: RFoldWithFolder<A, Folder<F>>,
{
    fn rfold(self, init: A, folder: Folder<F>) -> A {
        let Cons(head, tail) = self;
        let (init, mut folder) = tail.rfold_with_folder(init, folder);
        folder.fold(init, head)
    }
}

trait RFoldWithFolder<Accumulator, Folder>: HList {
    fn rfold_with_folder(self, init: Accumulator, folder: Folder) -> (Accumulator, Folder);
}

impl<A, F> RFoldWithFolder<A, F> for Nil {
    fn rfold_with_folder(self, init: A, folder: F) -> (A, F) {
        (init, folder)
    }
}

impl<A, F, Head, Tail> RFoldWithFolder<A, F> for Cons<Head, Tail>
where
    F: FnMut(A, Head) -> A,
    Tail: RFoldWithFolder<A, F>,
{
    fn rfold_with_folder(self, init: A, folder: F) -> (A, F) {
        let Cons(head, tail) = self;
        let (init, mut folder) = tail.rfold_with_folder(init, folder);
        let init = folder(init, head);
        (init, folder)
    }
}

impl<A, F, Head, Tail> RFoldWithFolder<A, Folder<F>> for Cons<Head, Tail>
where
    F: FoldFn<A, Head>,
    Tail: RFoldWithFolder<A, Folder<F>>,
{
    fn rfold_with_folder(self, init: A, folder: Folder<F>) -> (A, Folder<F>) {
        let Cons(head, tail) = self;
        let (init, mut folder) = tail.rfold_with_folder(init, folder);
        let init = folder.fold(init, head);
        (init, folder)
    }
}
