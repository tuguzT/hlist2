/// Heterogenous list with head and tail values, where tail is another heterogenous list.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
pub struct Cons<Head, Tail>(pub Head, pub Tail)
where
    Tail: ?Sized;

impl<Head, Tail> Cons<Head, Tail> {
    /// Constructs a new [`Cons`] with provided head and tail values.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let list = Cons::new(1, "hello world");
    /// assert_eq!(list, Cons(1, "hello world"));
    /// ```
    pub const fn new(head: Head, tail: Tail) -> Self {
        Self(head, tail)
    }

    /// Converts self into head value, discarding its tail.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let list = Cons::new(1, "hello world");
    /// let head = list.into_head();
    /// assert_eq!(head, 1);
    /// ```
    pub fn into_head(self) -> Head {
        let Self(head, _) = self;
        head
    }

    /// Converts self into tail value, discarding its head.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let list = Cons::new(1, "hello world");
    /// let tail = list.into_tail();
    /// assert_eq!(tail, "hello world");
    /// ```
    pub fn into_tail(self) -> Tail {
        let Self(_, tail) = self;
        tail
    }
}

impl<Head, Tail> Cons<Head, Tail>
where
    Tail: ?Sized,
{
    /// Borrows head value of self by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let list = Cons::new(1, "hello world");
    /// let head = list.head();
    /// assert_eq!(head, &1);
    /// ```
    pub const fn head(&self) -> &Head {
        let Self(head, _) = self;
        head
    }

    /// Borrows head value of self by mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let mut list = Cons::new(1, "hello world");
    /// let head = list.head_mut();
    /// *head = 2;
    /// assert_eq!(list, Cons(2, "hello world"));
    pub fn head_mut(&mut self) -> &mut Head {
        let Self(head, _) = self;
        head
    }

    /// Borrows tail value of self by reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let list = Cons::new(1, "hello world");
    /// let tail = list.tail();
    /// assert_eq!(tail, &"hello world");
    pub const fn tail(&self) -> &Tail {
        let Self(_, tail) = self;
        tail
    }

    /// Borrows tail value of self by mutable reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use hlist2::Cons;
    ///
    /// let mut list = Cons::new(1, "hello world");
    /// let tail = list.tail_mut();
    /// *tail = "привет, мир";
    /// assert_eq!(list, Cons(1, "привет, мир"));
    /// ```
    pub fn tail_mut(&mut self) -> &mut Tail {
        let Self(_, tail) = self;
        tail
    }
}

impl<Head, Tail> AsRef<Cons<Head, Tail>> for Cons<Head, Tail>
where
    Tail: ?Sized,
{
    fn as_ref(&self) -> &Cons<Head, Tail> {
        self
    }
}

impl<Head, Tail> AsMut<Cons<Head, Tail>> for Cons<Head, Tail>
where
    Tail: ?Sized,
{
    fn as_mut(&mut self) -> &mut Cons<Head, Tail> {
        self
    }
}

impl<Head, Tail> From<(Head, Tail)> for Cons<Head, Tail> {
    fn from(value: (Head, Tail)) -> Self {
        let (head, tail) = value;
        Self(head, tail)
    }
}

impl<Head, Tail> From<Cons<Head, Tail>> for (Head, Tail) {
    fn from(value: Cons<Head, Tail>) -> Self {
        let Cons(head, tail) = value;
        (head, tail)
    }
}
