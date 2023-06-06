use core::marker::PhantomData;

/// Used as an index into an [`HList`](trait@crate::HList) which points to the head of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of [`Get`](super::Get) trait based on knowledge
/// where the actual type is located: in the head or somewhere in the tail of the list.
pub struct Here {
    _never: Never,
}

/// Used as an index into an [`HList`](trait@crate::HList) which points to the tail of the heterogenous list.
///
/// This type of index exists due to lack of specialization in Rust.
/// This allows multiple implementations of [`Get`](super::Get) trait based on knowledge
/// where the actual type is located: in the head or somewhere in the tail of the list.
pub struct There<T> {
    _marker: PhantomData<fn() -> T>,
    _never: Never,
}

/// Make sure that [`Here`] and [`There`] indices cannot be constructed.
enum Never {}
