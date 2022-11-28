use std::fmt::Debug;
use std::ops::{Deref, DerefMut, Range};

/// An easy way to store location associated to data
#[derive(Clone,Debug)]
pub struct EasyLocated<X> {
    inner: X,
    loc: Range<usize>
}

impl<X> EasyLocated<X>
{
    #[inline]
    pub fn new(x: X, loc: Range<usize>) -> Self
    {
        Self { inner: x, loc }
    }

    #[inline]
    pub fn map<Y,F:FnMut(X) -> Y>(self, mut f:F) -> EasyLocated<Y>
    {
        EasyLocated { inner: (f)(self.inner), loc: self.loc }
    }

    #[inline]
    pub fn location(&self) -> &Range<usize>
    {
        &self.loc
    }

    #[inline]
    pub fn into_inner(self) -> X
    {
        self.inner
    }
}

impl<X> EasyLocated<Option<X>>
{
    #[inline]
    pub fn transpose(self) -> Option<EasyLocated<X>>
    {
        self.inner.map(|x| EasyLocated::new(x,self.loc))
    }

    #[inline]
    pub fn and_then<Y,F:FnMut(X) -> Option<Y>>(self, f:F) -> EasyLocated<Option<Y>>
    {
        EasyLocated {
            inner: self.inner.and_then(f),
            loc: self.loc
        }
    }
}

impl<X,E> EasyLocated<Result<X,E>>
{
    #[inline]
    pub fn transpose(self) -> Result<EasyLocated<X>,E>
    {
        match self.inner {
            Ok(x) => { Ok(EasyLocated::new(x, self.loc)) }
            Err(e) => { Err(e) }
        }
    }

    #[inline]
    pub fn and_then<Y,F:FnMut(X) -> Result<Y,E>>(self, f:F) -> EasyLocated<Result<Y,E>>
    {
        EasyLocated {
            inner: self.inner.and_then(f),
            loc: self.loc
        }
    }
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


impl<X> Into<(X,Range<usize>)> for EasyLocated<X>
{
    #[inline]
    fn into(self) -> (X,Range<usize>) { (self.inner,self.loc) }
}

impl<'a,X> Into<(&'a X,&'a Range<usize>)> for &'a EasyLocated<X>
{
    #[inline]
    fn into(self) -> (&'a X,&'a Range<usize>) { (&self.inner,&self.loc) }
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

impl<X> From<EasyLocated<Option<X>>> for Option<EasyLocated<X>>
{
    #[inline]
    fn from(x: EasyLocated<Option<X>>) -> Self { x.transpose() }
}


impl<X,E> From<EasyLocated<Result<X,E>>> for Result<EasyLocated<X>,E>
{
    #[inline]
    fn from(x: EasyLocated<Result<X,E>>) -> Self { x.transpose() }
}



#[cfg(test)]
mod tests {
    use crate::EasyLocated;

    #[test]
    fn mapping()
    {
        let x = EasyLocated::new(2.5, 0..2);
        let y = x.map(|x| (x*2.) as u32);

        assert_eq!( *y, 5);
        assert_eq!( *y.location(), 0..2);

        let x = EasyLocated::new(Some(2), 0..2);
        let y = x.transpose().unwrap();
        assert_eq! ( *y, 2);
    }

}