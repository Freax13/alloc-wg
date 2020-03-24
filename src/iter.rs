use crate::{alloc::AllocRef, collections::TryReserveError};

/// Extend a collection "fallibly" with the contents of an iterator.
pub trait TryExtend<A> {
    type Err;
    /// Extends a collection "fallibly" with the contents of an iterator.
    ///
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use alloc_wg::{iter::TryExtend, vec};
    ///
    /// // You can extend a Vec<char> with some chars:
    /// let mut message = vec!['a', 'b', 'c'];
    ///
    /// message.try_extend(['d', 'e', 'f'].iter())?;
    ///
    /// assert_eq!(vec!['a', 'b', 'c', 'd', 'e', 'f'], message);
    /// # Ok::<(), alloc_wg::collections::TryReserveError>(())
    /// ```
    fn try_extend<T: IntoIterator<Item = A>>(&mut self, iter: T) -> Result<(), Self::Err>;
}

pub trait FromIteratorIn<T, A: AllocRef> {
    fn from_iter_in<I: IntoIterator<Item = T>>(iter: I, allocator: A) -> Self;

    fn try_from_iter_in<I: IntoIterator<Item = T>>(
        iter: I,
        allocator: A,
    ) -> Result<Self, TryReserveError>
    where
        Self: Sized;
}

pub trait IteratorExt: Iterator + Sized {
    #[inline]
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn collect_in<T: FromIteratorIn<Self::Item, A>, A: AllocRef>(self, allocator: A) -> T {
        FromIteratorIn::from_iter_in(self, allocator)
    }

    #[inline]
    #[must_use = "if you really need to exhaust the iterator, consider `.for_each(drop)` instead"]
    fn try_collect_in<T: FromIteratorIn<Self::Item, A>, A: AllocRef>(
        self,
        allocator: A,
    ) -> Result<T, TryReserveError> {
        FromIteratorIn::try_from_iter_in(self, allocator)
    }
}

impl<T> IteratorExt for T where T: Iterator {}
