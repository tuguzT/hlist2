pub trait Pair {
    type First;
    type Second;

    fn destruct(self) -> (Self::First, Self::Second);
}

impl<First, Second> Pair for (First, Second) {
    type First = First;
    type Second = Second;

    #[inline(always)]
    fn destruct(self) -> (Self::First, Self::Second) {
        self
    }
}
