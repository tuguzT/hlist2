use crate::HList;

use super::{ManyIndex, RemoveMany, ToRef};

/// Retrieve many elements from the heterogenous list by type.
///
/// Contrary to the less powerful [`Get`](crate::ops::Get) trait (it cannot return multiple mutable references!),
/// methods of this trait can be called **only** via generics or
/// unqualified syntax because for now Rust cannot infer `T` using return type only.
pub trait GetMany<T, I>: HList
where
    T: ToRef,
    I: ManyIndex,
{
    /// Retrieves a heterogenous list of references to the elements
    /// of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, HList, ops::GetMany};
    ///
    /// let list = hlist![1, 2.0, true, "hello world"];
    /// let many = GetMany::<HList![&str, f32], _>::get_many(&list);
    /// assert_eq!(many, hlist![&"hello world", &2.0]);
    /// ```
    fn get_many(&self) -> T::Ref<'_>;

    /// Retrieves a heterogenous list of mutable references to the elements
    /// of the heterogenous list by type.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, HList, ops::GetMany};
    ///
    /// let mut list = hlist![1, 2.0, true, "hello world"];
    /// let many_mut = GetMany::<HList![&str, f32], _>::get_many_mut(&mut list);
    /// assert_eq!(many_mut, hlist![&mut "hello world", &mut 2.0]);
    /// ```
    fn get_many_mut(&mut self) -> T::RefMut<'_>;
}

impl<L, T, I> GetMany<T, I> for L
where
    T: ToRef,
    L: ToRef,
    I: ManyIndex,
    for<'any> L::Ref<'any>: RemoveMany<T::Ref<'any>, I>,
    for<'any> L::RefMut<'any>: RemoveMany<T::RefMut<'any>, I>,
{
    fn get_many(&self) -> T::Ref<'_> {
        let refs = self.to_ref();
        let (many, _) = refs.remove_many();
        many
    }

    fn get_many_mut(&mut self) -> T::RefMut<'_> {
        let muts = self.to_mut();
        let (many_mut, _) = muts.remove_many();
        many_mut
    }
}
