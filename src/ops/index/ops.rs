use core::ops::{Add, Index, IndexMut, Sub};

use crate::{Cons, HList};

use super::{Dec, Here, Index as Idx, There};

impl<T> Add<T> for Here
where
    T: Idx,
{
    type Output = T;

    fn add(self, rhs: T) -> Self::Output {
        rhs
    }
}

impl<T, U> Add<T> for There<U>
where
    U: Add<T>,
{
    type Output = There<U::Output>;

    fn add(self, _: T) -> Self::Output {
        Default::default()
    }
}

impl Sub<Here> for Here {
    type Output = Here;

    fn sub(self, rhs: Here) -> Self::Output {
        rhs
    }
}

impl<T> Sub<Here> for There<T>
where
    T: Idx,
{
    type Output = There<T>;

    fn sub(self, _: Here) -> Self::Output {
        Default::default()
    }
}

impl<T, U> Sub<There<T>> for There<U>
where
    T: Idx,
    U: Idx + Sub<T>,
{
    type Output = U::Output;

    fn sub(self, rhs: There<T>) -> Self::Output {
        self.dec() - rhs.dec()
    }
}

/// Retrieves the first element of the heterogenous list in immutable contexts.
///
/// # Examples
///
/// ```
/// use hlist2::{hlist, ops::Here};
///
/// let list = hlist![1, 2.0, false];
/// let a = list[Here];
/// assert_eq!(a, 1);
/// ```
impl<Head, Tail> Index<Here> for Cons<Head, Tail>
where
    Tail: HList + ?Sized,
{
    type Output = Head;

    fn index(&self, _: Here) -> &Self::Output {
        let Cons(head, _) = self;
        head
    }
}

/// Retrieves the first element of the heterogenous list in mutable contexts.
///
/// # Examples
///
/// ```
/// use hlist2::{hlist, ops::Here};
///
/// let mut list = hlist![1, 2.0, false];
/// list[Here] = 5;
///
/// let a = list[Here];
/// assert_eq!(a, 5);
/// ```
impl<Head, Tail> IndexMut<Here> for Cons<Head, Tail>
where
    Tail: HList + ?Sized,
{
    fn index_mut(&mut self, _: Here) -> &mut Self::Output {
        let Cons(head, _) = self;
        head
    }
}

/// Performs indexing operation in the tail
/// of the heterogenous list in immutable contexts.
///
/// # Examples
///
/// ```
/// use hlist2::{hlist, ops::{Here, Inc}};
///
/// let list = hlist![1, 2.0, false];
///
/// let index = Here.inc();
/// assert_eq!(list[index], 2.0);
///
/// let index = index.inc();
/// assert_eq!(list[index], false);
/// ```
impl<Head, Tail, TailIndex> Index<There<TailIndex>> for Cons<Head, Tail>
where
    Tail: Index<TailIndex> + ?Sized,
    TailIndex: Idx,
{
    type Output = Tail::Output;

    fn index(&self, index: There<TailIndex>) -> &Self::Output {
        let Cons(_, tail) = self;
        let index = index.dec();
        tail.index(index)
    }
}

/// Performs indexing operation in the tail
/// of the heterogenous list in mutable contexts.
///
/// # Examples
///
/// ```
/// use hlist2::{hlist, ops::{Here, Inc}};
///
/// let mut list = hlist![1, 2.0, false];
///
/// let index = Here.inc();
/// list[index] = 16_f32.sqrt();
///
/// let b = list[index];
/// assert_eq!(b, 4.0);
/// ```
impl<Head, Tail, TailIndex> IndexMut<There<TailIndex>> for Cons<Head, Tail>
where
    Tail: IndexMut<TailIndex> + ?Sized,
    TailIndex: Idx,
{
    fn index_mut(&mut self, index: There<TailIndex>) -> &mut Self::Output {
        let Cons(_, tail) = self;
        let index = index.dec();
        tail.index_mut(index)
    }
}
