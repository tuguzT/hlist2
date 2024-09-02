/// An empty heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Nil;

impl Nil {
    /// Length (count of elements) of the heterogenous list.
    ///
    /// For [`Nil`], this is always `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Nil;
    ///
    /// assert_eq!(Nil::LEN, 0);
    /// ```
    pub const LEN: usize = 0;

    /// Constructs a new empty heterogenous list.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Nil;
    ///
    /// let nil = Nil::new();
    /// assert!(nil.is_empty());
    /// ```
    pub const fn new() -> Self {
        Self
    }

    /// Returns the length (count of elements) of the heterogenous list.
    ///
    /// For [`Nil`], this is always `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Nil;
    ///
    /// assert_eq!(Nil.len(), 0);
    /// ```
    pub const fn len(&self) -> usize {
        Self::LEN
    }

    /// Checks if the heterogenous list is empty.
    ///
    /// For [`Nil`], this is always `true`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Nil;
    ///
    /// assert!(Nil.is_empty());
    /// ```
    pub const fn is_empty(&self) -> bool {
        true
    }
}

impl AsRef<Nil> for Nil {
    fn as_ref(&self) -> &Nil {
        self
    }
}

impl AsMut<Nil> for Nil {
    fn as_mut(&mut self) -> &mut Nil {
        self
    }
}
