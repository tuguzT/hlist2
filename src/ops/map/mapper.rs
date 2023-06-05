/// Thin wrapper around [mapper function](self::MapFn).
///
/// Pass this struct into [`Map::map()`](super::Map::map)
/// to use generic implementation of mapper function.
pub struct Mapper<T>(pub T);

/// Implementation of mapper function for specified input and output types.
pub trait MapFn<Input> {
    /// Type of transformed input value.
    type Output;

    /// Transforms input value into output value.
    fn map(&mut self, input: Input) -> Self::Output;
}

impl<Input, Output, F> MapFn<Input> for Mapper<F>
where
    F: MapFn<Input, Output = Output>,
{
    type Output = Output;

    fn map(&mut self, input: Input) -> Self::Output {
        let Self(mapper) = self;
        mapper.map(input)
    }
}

impl<Input, Output, F> MapFn<Input> for &mut F
where
    F: MapFn<Input, Output = Output>,
{
    type Output = Output;

    fn map(&mut self, input: Input) -> Self::Output {
        (**self).map(input)
    }
}
