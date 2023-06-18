use crate::{Cons, HList, Nil};

use super::Extend;

/// Flattens one level of nesting in a heterogenous list of heterogenous lists.
///
/// This is useful when you have a heterogenous list of heterogenous lists
/// and you want to remove one level of indirection.
pub trait Flatten: HList {
    /// Flattened heterogenous list.
    type Output: HList;

    /// Flattens a heterogenous list of heterogenous lists, removing one level of indirection.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use hlist2::{hlist, ops::Flatten};
    ///
    /// let data = hlist![hlist![1, 2.0, true, "hello world"], hlist![5, 6.0]];
    /// let flattened = data.flatten();
    /// assert_eq!(flattened, hlist![1, 2.0, true, "hello world", 5, 6.0]);
    /// ```
    ///
    /// Flattening only removes one level of nesting at a time:
    ///
    /// ```
    /// use hlist2::{hlist, ops::Flatten};
    ///
    /// let d3 = hlist![
    ///     hlist![hlist![1, 2], hlist![3, 4]],
    ///     hlist![hlist![5, 6], hlist![7, 8]],
    /// ];
    ///
    /// let d2 = d3.flatten();
    /// assert_eq!(d2, hlist![hlist![1, 2], hlist![3, 4], hlist![5, 6], hlist![7, 8]]);
    ///
    /// let d1 = d3.flatten().flatten();
    /// assert_eq!(d1, hlist![1, 2, 3, 4, 5, 6, 7, 8]);
    /// ```
    ///
    /// Here we see that `flatten()` does not perform a “deep” flatten. Instead, only one level of nesting is removed.
    /// That is, if you `flatten()` a three-dimensional array, the result will be two-dimensional and not one-dimensional.
    /// To get a one-dimensional structure, you have to `flatten()` again.
    fn flatten(self) -> Self::Output;
}

impl Flatten for Nil {
    type Output = Self;

    fn flatten(self) -> Self::Output {
        self
    }
}

impl<Head, Tail> Flatten for Cons<Head, Tail>
where
    Head: Extend,
    Tail: Flatten,
{
    type Output = Head::Output<Tail::Output>;

    fn flatten(self) -> Self::Output {
        let Cons(head, tail) = self;
        let tail = tail.flatten();
        head.extend(tail)
    }
}
