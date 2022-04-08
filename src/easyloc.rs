use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Range};

/// An easy way to store location associated to data
#[derive(Clone,Debug)]
pub struct EasyLocated<X> {
    inner: X,
    loc: Range<usize>
}

impl<X> Deref for EasyLocated<X> {
    type Target = X;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<X> DerefMut for EasyLocated<X> {

    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<X> From<(X,Range<usize>)> for EasyLocated<X>
{
    #[inline]
    fn from((inner, loc): (X, Range<usize>)) -> Self {
        Self { inner, loc }
    }
}

impl<X> Into<Range<usize>> for EasyLocated<X>
{
    #[inline]
    fn into(self) -> Range<usize> { self.loc }
}

impl<X> Into<Range<usize>> for &EasyLocated<X>
{
    #[inline]
    fn into(self) -> Range<usize> {
        self.loc.clone()
    }
}

