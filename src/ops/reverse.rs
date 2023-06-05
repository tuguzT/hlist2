use crate::{Cons, HList, Nil};

/// Reverse elements of the heterogenous list.
pub trait Reverse: HList {
    /// Type of new heterogenous list with the opposite order of elements.
    type Output: HList;

    /// Reverses elements of the heterogenous list.
    ///
    /// New heterogenous list will contain all elements of the old one
    /// with the opposite order.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, ops::Reverse};
    ///
    /// let list = hlist![1, 2.0, true, "hello world"];
    /// assert_eq!(list.reverse(), hlist!["hello world", true, 2.0, 1]);
    /// ```
    fn reverse(self) -> Self::Output;
}

impl<T> Reverse for T
where
    T: Rewind<Nil>,
{
    type Output = T::Output;

    fn reverse(self) -> Self::Output {
        self.rewind(Nil)
    }
}

pub trait Rewind<Done>: HList
where
    Done: HList,
{
    type Output: HList;

    fn rewind(self, done: Done) -> Self::Output;
}

impl<Done> Rewind<Done> for Nil
where
    Done: HList,
{
    type Output = Done;

    fn rewind(self, done: Done) -> Self::Output {
        done
    }
}

impl<Done, Next, Tail> Rewind<Done> for Cons<Next, Tail>
where
    Done: HList,
    Tail: Rewind<Cons<Next, Done>>,
{
    type Output = Tail::Output;

    fn rewind(self, done: Done) -> Self::Output {
        let Cons(next, tail) = self;
        tail.rewind(Cons(next, done))
    }
}
