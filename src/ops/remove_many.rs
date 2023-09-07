use crate::{Cons, HList, Nil};

use super::{Index, Remove};

/// Type of index which is used to remove many elements from the heterogenous list.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is implemented only for heterogenous lists where all the elements implement [`Index`] trait.
pub trait RemoveManyIndex: HList + Default + sealed::Sealed {}

impl RemoveManyIndex for Nil {}

impl<Head, Tail> RemoveManyIndex for Cons<Head, Tail>
where
    Head: Index,
    Tail: RemoveManyIndex,
{
}

mod sealed {
    use crate::{ops::Index, Cons, Nil};

    pub trait Sealed {}

    impl Sealed for Nil {}

    impl<Head, Tail> Sealed for Cons<Head, Tail>
    where
        Head: Index,
        Tail: Sealed,
    {
    }
}

/// Move many elements out of the heterogenous list by their types.
pub trait RemoveMany<T, I>: HList
where
    T: HList,
    I: RemoveManyIndex,
{
    /// Remaining part of the heterogenous list without removed elements.
    type Remainder: HList;

    /// Moves many elements out of the heterogenous list by their types.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, HList, ops::RemoveMany};
    ///
    /// let list = hlist![1, 2.0, true, "hello world"];
    /// let (list, remainder): (HList![f32, i32, &str], _) = list.remove_many();
    /// assert_eq!(list, hlist![2.0, 1, "hello world"]);
    /// assert_eq!(remainder, hlist![true]);
    /// ```
    fn remove_many(self) -> (T, Self::Remainder);
}

impl<T> RemoveMany<Nil, Nil> for T
where
    T: HList,
{
    type Remainder = Self;

    fn remove_many(self) -> (Nil, Self::Remainder) {
        (Nil, self)
    }
}

impl<Head, Tail, OtherHead, OtherTail, IndexHead, IndexTail>
    RemoveMany<Cons<OtherHead, OtherTail>, Cons<IndexHead, IndexTail>> for Cons<Head, Tail>
where
    OtherTail: HList,
    IndexHead: Index,
    IndexTail: RemoveManyIndex,
    Self: Remove<OtherHead, IndexHead>,
    <Self as Remove<OtherHead, IndexHead>>::Remainder: RemoveMany<OtherTail, IndexTail>,
{
    type Remainder = <<Self as Remove<OtherHead, IndexHead>>::Remainder as RemoveMany<
        OtherTail,
        IndexTail,
    >>::Remainder;

    fn remove_many(self) -> (Cons<OtherHead, OtherTail>, Self::Remainder) {
        let (head, remainder) = self.remove();
        let (tail, remainder) = remainder.remove_many();
        let list = Cons(head, tail);
        (list, remainder)
    }
}
