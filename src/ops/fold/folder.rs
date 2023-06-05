/// Thin wrapper around [folder function](self::FoldFn).
///
/// Pass this struct into [`Fold::fold()`](super::Fold::fold)
/// or into [`RFold::rfold()`](super::RFold::rfold)
/// to use generic implementation of folder function.
pub struct Folder<T>(pub T);

/// Implementation of folder function
/// for specified item and accumulator types.
pub trait FoldFn<Accumulator, Item> {
    /// Folds an item into an accumulator.
    fn fold(&mut self, accumulator: Accumulator, item: Item) -> Accumulator;
}

impl<A, T, F> FoldFn<A, T> for Folder<F>
where
    F: FoldFn<A, T>,
{
    fn fold(&mut self, accumulator: A, item: T) -> A {
        let Self(folder) = self;
        folder.fold(accumulator, item)
    }
}

impl<A, T, F> FoldFn<A, T> for &mut F
where
    F: FoldFn<A, T>,
{
    fn fold(&mut self, accumulator: A, item: T) -> A {
        (**self).fold(accumulator, item)
    }
}
