#![allow(clippy::module_inception)]

use crate::{Cons, HList, Nil};

use super::{MapFn, Mapper};

/// Transform one heterogenous list into another.
pub trait Map<Mapper>: HList {
    /// Type of new heterogenous list after transformation.
    type Output: HList;

    /// Transforms the heterogenous list into another heterogenous list
    /// by applying an operation to each element by mapper.
    ///
    /// # Examples
    ///
    /// You can map the list if it is homogenous (all elements have the same type):
    ///
    /// ```
    /// use hlist2::{hlist, ops::Map};
    ///
    /// let list = hlist![1, 2, 3];
    /// assert_eq!(list.map(|x| 2 * x), hlist![2, 4, 6]);
    /// ```
    ///
    /// Mapping of heterogenous list is possible with heterogenous list of closures as folder:
    ///
    /// ```
    /// use hlist2::{hlist, ops::Map};
    ///
    /// let list = hlist![1, 2.0, true];
    /// let list = list.map(
    ///     hlist![
    ///         |i| i + 2,
    ///         |f| f - 2.0,
    ///         |b: bool| !b,
    ///     ]
    /// );
    /// assert_eq!(list, hlist![3, 0.0, false]);
    /// ```
    ///
    /// Or with special implementation of [mapper function](MapFn):
    ///
    /// ```
    /// use hlist2::{
    ///     hlist,
    ///     ops::{Map, MapFn, Mapper},
    /// };
    ///
    /// struct MyMapFn;
    ///
    /// impl MapFn<i32> for MyMapFn {
    ///     type Output = i32;
    ///     fn map(&mut self, n: i32) -> i32 { n + 3 }
    /// }
    /// impl MapFn<f32> for MyMapFn {
    ///     type Output = f32;
    ///     fn map(&mut self, f: f32) -> f32 { f + 8959.0 }
    /// }
    /// impl MapFn<bool> for MyMapFn {
    ///     type Output = bool;
    ///     fn map(&mut self, b: bool) -> bool { !b }
    /// }
    ///
    /// let list = hlist![1, false, 42f32];
    /// let list = list.map(Mapper(MyMapFn));
    /// assert_eq!(list, hlist![4, true, 9001.0]);
    /// ```
    fn map(self, mapper: Mapper) -> Self::Output;
}

impl<M> Map<M> for Nil {
    type Output = Nil;

    fn map(self, _: M) -> Self::Output {
        self
    }
}

impl<M, R, Head, Tail> Map<M> for Cons<Head, Tail>
where
    M: FnMut(Head) -> R,
    Tail: Map<M>,
{
    type Output = Cons<R, Tail::Output>;

    fn map(self, mut mapper: M) -> Self::Output {
        let Cons(head, tail) = self;
        let head = mapper(head);
        let tail = tail.map(mapper);
        Cons(head, tail)
    }
}

impl<MHead, MTail, Head, Tail, R> Map<Cons<MHead, MTail>> for Cons<Head, Tail>
where
    MHead: FnOnce(Head) -> R,
    Tail: Map<MTail>,
{
    type Output = Cons<R, Tail::Output>;

    fn map(self, mapper: Cons<MHead, MTail>) -> Self::Output {
        let Cons(head, tail) = self;
        let Cons(mapper_head, mapper_tail) = mapper;
        let head = mapper_head(head);
        let tail = tail.map(mapper_tail);
        Cons(head, tail)
    }
}

impl<M, Head, Tail> Map<Mapper<M>> for Cons<Head, Tail>
where
    M: MapFn<Head>,
    Tail: Map<Mapper<M>>,
{
    type Output = Cons<M::Output, Tail::Output>;

    fn map(self, mut mapper: Mapper<M>) -> Self::Output {
        let Cons(head, tail) = self;
        let head = mapper.map(head);
        let tail = tail.map(mapper);
        Cons(head, tail)
    }
}
