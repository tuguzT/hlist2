use crate::{ops::Index, Cons, HList, Nil};

/// Type of index which is used to index many elements of the heterogenous list.
///
/// This trait is sealed and cannot be implemented outside of this crate.
/// It is implemented only for heterogenous lists where all the elements implement [`Index`] trait.
pub trait ManyIndex: HList + Default + sealed::Sealed {}

impl ManyIndex for Nil {}

impl<Head, Tail> ManyIndex for Cons<Head, Tail>
where
    Head: Index,
    Tail: ManyIndex,
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
