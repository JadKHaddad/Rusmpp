/// An enum representing either one of two iterator types.
pub enum EitherIterator<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> EitherIterator<L, R> {
    /// Creates an [`EitherIterator::Left`].
    pub const fn left(left: L) -> Self {
        EitherIterator::Left(left)
    }

    /// Creates an [`EitherIterator::Right`].
    pub const fn right(right: R) -> Self {
        EitherIterator::Right(right)
    }
}

impl<L, R, T> Iterator for EitherIterator<L, R>
where
    L: Iterator<Item = T>,
    R: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            EitherIterator::Left(left) => left.next(),
            EitherIterator::Right(right) => right.next(),
        }
    }
}
