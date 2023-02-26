//! Traits for conversion from and into heterogenous list types.
//!
//! This trait can be implemented for any struct, because any struct
//! can be represented as a heterogenous list of its fields.

use crate::HList;

/// Used to convert heterogenous list into a value of another type.
/// It is the reciprocal of [`IntoHList`].
///
/// Similar to [`From`] trait, but uses associated type instead of generic parameter.
pub trait FromHList {
    /// Type of heterogenous list from which conversion will be performed.
    type HList: HList;

    /// Converts heterogenous list into a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, convert::FromHList};
    ///
    /// let list = hlist!(1, 2.0, true, "hello world");
    /// let tuple: (_, _, _, _) = FromHList::from_hlist(list);
    /// assert_eq!(tuple, (1, 2.0, true, "hello world"));
    /// ```
    fn from_hlist(hlist: Self::HList) -> Self;
}

impl<T> FromHList for T
where
    T: HList,
{
    type HList = T;

    fn from_hlist(hlist: Self::HList) -> Self {
        hlist
    }
}

macro_rules! hlist_from_tuple {
    ($($types:ident),*) => {
        impl<$($types),*> From<($($types,)*)> for $crate::HList!($($types,)*) {
            #[allow(non_snake_case)]
            fn from(value: ($($types,)*)) -> Self {
                let ($($types,)*) = value;
                $crate::hlist!($($types,)*)
            }
        }

        impl<$($types),*> FromHList for ($($types,)*) {
            type HList = $crate::HList!($($types,)*);

            fn from_hlist(hlist: Self::HList) -> Self {
                Self::from(hlist)
            }
        }
    };
}

// Conversion from tuple to heterogenous list is implemented for tuples of size 12 and less
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J, K);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I, J);
hlist_from_tuple!(A, B, C, D, E, F, G, H, I);
hlist_from_tuple!(A, B, C, D, E, F, G, H);
hlist_from_tuple!(A, B, C, D, E, F, G);
hlist_from_tuple!(A, B, C, D, E, F);
hlist_from_tuple!(A, B, C, D, E);
hlist_from_tuple!(A, B, C, D);
hlist_from_tuple!(A, B, C);
hlist_from_tuple!(A, B);
hlist_from_tuple!(A);
hlist_from_tuple!();

/// Used to convert value of some type into heterogenous list.
/// The opposite of [`FromHList`].
///
/// Similar to [`Into`] trait, but uses associated type instead of generic parameter.
pub trait IntoHList {
    /// Type of heterogenous list value of some type will be converted to.
    type HList: HList;

    /// Converts self into heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::{hlist, convert::IntoHList};
    ///
    /// let tuple = (1, 2.0, true, "hello world");
    /// assert_eq!(tuple.into_hlist(), hlist!(1, 2.0, true, "hello world"));
    /// ```
    fn into_hlist(self) -> Self::HList;
}

impl<T> IntoHList for T
where
    T: HList,
{
    type HList = T;

    fn into_hlist(self) -> Self::HList {
        self
    }
}

macro_rules! tuple_from_hlist {
    ($($types:ident),*) => {
        impl<$($types),*> From<$crate::HList!($($types,)*)> for ($($types,)*) {
            #[allow(non_snake_case, clippy::unused_unit)]
            fn from(value: $crate::HList!($($types,)*)) -> Self {
                let $crate::hlist!($($types,)*) = value;
                ($($types,)*)
            }
        }

        impl<$($types),*> IntoHList for ($($types,)*) {
            type HList = $crate::HList!($($types,)*);

            fn into_hlist(self) -> Self::HList {
                self.into()
            }
        }
    };
}

// Conversion from heterogenous list to tuple is implemented for tuples of size 12 and less
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J, K);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I, J);
tuple_from_hlist!(A, B, C, D, E, F, G, H, I);
tuple_from_hlist!(A, B, C, D, E, F, G, H);
tuple_from_hlist!(A, B, C, D, E, F, G);
tuple_from_hlist!(A, B, C, D, E, F);
tuple_from_hlist!(A, B, C, D, E);
tuple_from_hlist!(A, B, C, D);
tuple_from_hlist!(A, B, C);
tuple_from_hlist!(A, B);
tuple_from_hlist!(A);
tuple_from_hlist!();
