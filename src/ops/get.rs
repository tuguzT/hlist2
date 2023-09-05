use crate::{Cons, HList};

use super::{Here, Index, There};

/// Retrieve element of the heterogenous list by type.
pub trait Get<T, I>: HList
where
    I: Index,
{
    /// Retrieves a reference to the element of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Get};
    ///
    /// let list = hlist![0_i32, 1_i64];
    /// let a: i64 = *list.get();
    /// assert_eq!(a, 1);
    /// ```
    fn get(&self) -> &T;

    /// Retrieves a mutable reference to the element of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Get};
    ///
    /// let mut list = hlist![0_i32, 1_i64];
    /// *list.get_mut() = 5_i32;
    /// let a: i32 = *list.get();
    /// assert_eq!(a, 5);
    /// ```
    fn get_mut(&mut self) -> &mut T;
}

/// Desired type is located in the head of the heterogenous list.
impl<Head, Tail> Get<Head, Here> for Cons<Head, Tail>
where
    Tail: HList + ?Sized,
{
    fn get(&self) -> &Head {
        let Cons(head, _) = self;
        head
    }

    fn get_mut(&mut self) -> &mut Head {
        let Cons(head, _) = self;
        head
    }
}

/// Desired type is located somewhere in the tail of the heterogenous list.
impl<Head, Tail, FromTail, TailIndex> Get<FromTail, There<TailIndex>> for Cons<Head, Tail>
where
    Tail: Get<FromTail, TailIndex> + ?Sized,
    TailIndex: Index,
{
    fn get(&self) -> &FromTail {
        let Cons(_, tail) = self;
        tail.get()
    }

    fn get_mut(&mut self) -> &mut FromTail {
        let Cons(_, tail) = self;
        tail.get_mut()
    }
}
