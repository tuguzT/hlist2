use crate::{HList, Nil};

use super::{ManyIndex, RemoveMany};

/// Shuffle current heterogenous list, or change order of its elements.
///
/// Index generic parameter is used to determine a way to construct shuffled list,
/// because there may be more than one if type of elements is not unique.
pub trait Shuffle<T, I>: RemoveMany<T, I, Remainder = Nil>
where
    T: HList,
    I: ManyIndex,
{
    /// Shuffles current heterogenous list, or changes order of its elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, HList, ops::Shuffle};
    ///
    /// let list = hlist![1, 2.0, true, "hello world"];
    /// let shuffled: HList![bool, i32, &str, f32] = list.shuffle();
    /// assert_eq!(shuffled, hlist![true, 1, "hello world", 2.0]);
    /// ```
    fn shuffle(self) -> T;
}

impl<T, L, I> Shuffle<L, I> for T
where
    L: HList,
    T: RemoveMany<L, I, Remainder = Nil>,
    I: ManyIndex,
{
    fn shuffle(self) -> L {
        let (list, _) = self.remove_many();
        list
    }
}
