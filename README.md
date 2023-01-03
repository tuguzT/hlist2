# hlist2

[![Crate](https://img.shields.io/crates/v/hlist2.svg)](https://crates.io/crates/hlist2)
[![Docs](https://docs.rs/hlist2/badge.svg)](https://docs.rs/hlist2)
![License](https://img.shields.io/badge/license-MIT%20OR%20Apache%202.0-blue.svg)

This crate defines a way to create compile-time heterogenous lists, or lists consisting of multiple types.

## Heterogenous lists

This crate defines types for an empty list, `Nil`,
and for pair of list head and its remainder, `Cons`.
Heterogenous list consists of many conses contained recursively one in another,
and the last cons with the last element contains nil as the remainder.
For example, heterogenous list of integer, double and bool can be represented as
`Cons(1, Cons(2.0, Cons(true, Nil)))` with type of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.

## Recursive nature and behavior

Such recursive nature of heterogenous list allows us to implement various traits recursively
and without any restrictions on the size of such list or types contained in it.
Unlike tuples, traits can be implemented for all heterogenous lists
and even for those which count of elements is bigger than 12, lack of which for tuples is a problem sometimes.

All heterogenous lists implement `HList` trait, so it can be used in generics.
For example, this can be useful to bound generic type to be heterogenous list.
To implement your trait for all heterogenous lists of any size,
first implement it on `Nil` type, which is `HList` too.
Then, implement your trait on `Cons` struct with head and tail generic types
where tail type is heterogenous list too (or which implement `HList` trait).

Examples of these technique can be viewed in `ops` module, where
all the specific operations for all heterogenous list types are implemented.
For example, to append any value to the end of the list, use `Append` trait;
to prepend any value to the beginning of the list, use `Prepend` trait, and so on.

## Constructing and destructing heterogenous lists

But such recursive nature can be a problem when we try to name the type of heterogenous list
or use pattern matching with values of heterogenous lists.
To simplify creation of lists and naming of list types the crate defines two macros, `hlist!` and `HList!`.
The first one should be used for creation of heterogenous lists or for pattern matching,
while the second one should be used to name the type of heterogenous list.

So instead of writing `Cons(1, Cons(2.0, Cons(true, Nil)))`
we can write more readable and tuple-like expression like `hlist!(1, 2.0, true)`.

To name the type of such list, we can write `HList!(i32, f64, bool)`
instead of `Cons<i32, Cons<f64, Cons<bool, Nil>>>`.

## Tuple compatibility

Also this crate has a compatibility with tuples which is defined in `tuple` module.
It implements conversion between heterogenous lists and their tuple forms
when tuple has length of 12 and less, and vise versa.

## Features

This crate uses **no unsafe code** to provide the same safety guarantees the Rust programming language provides.

This crate is `no_std`, so it can be used freely and with no fear in embedded environment.

## Inspirations

This crate is intended to be an alternative to unmaintained [hlist](https://github.com/Sgeo/hlist) crate.
This is also inspired by another open source crate [frunk](https://github.com/lloydmeta/frunk).

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
