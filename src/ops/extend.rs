use crate::{Cons, HList, Nil};

/// Extend heterogenous list with another heterogenous list.
///
/// Elements of another heterogenous list will be places at the end
/// of the current heterogenous list in the order of which they was in another list.
pub trait Extend: HList {
    /// Type of heterogenous list extended with elements of another heterogenous list.
    type Output<T>: HList
    where
        T: HList;

    /// Extends heterogenous list with another heterogenous list.
    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList;
}

impl Extend for Nil {
    type Output<T> = T where T: HList;

    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList,
    {
        list
    }
}

impl<Head, Tail> Extend for Cons<Head, Tail>
where
    Tail: Extend,
{
    type Output<T> = Cons<Head, Tail::Output<T>> where T: HList;

    fn extend<T>(self, list: T) -> Self::Output<T>
    where
        T: HList,
    {
        let Cons(head, tail) = self;
        Cons(head, tail.extend(list))
    }
}
