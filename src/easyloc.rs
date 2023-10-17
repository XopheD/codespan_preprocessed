use std::borrow::{Borrow, BorrowMut};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut, Range};

/// An easy way to store location associated to data
///
/// To sum up, an easy located is a data with a location
/// added as a metadata. It means that any operation
/// (hash, comparing, printing...) is defined only on
/// the inner data (the location is ignored).
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
    pub fn location(&self) -> &Range<usize>
    {
        &self.loc
    }

    #[inline]
    pub fn into_inner(self) -> X
    {
        self.inner
    }

    #[inline]
    pub fn map<Y,F:FnMut(X) -> Y>(self, mut f:F) -> EasyLocated<Y> {
        EasyLocated { inner: f(self.inner), loc: self.loc }
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

impl<X> AsRef<X> for EasyLocated<X>
{
    #[inline] fn as_ref(&self) -> &X {
        &self.inner
    }
}

impl<X> AsMut<X> for EasyLocated<X>
{
    #[inline] fn as_mut(&mut self) -> &mut X {
        &mut self.inner
    }
}

impl<X> Borrow<X> for EasyLocated<X>
{
    #[inline] fn borrow(&self) -> &X {
        &self.inner
    }
}

impl<X> BorrowMut<X> for EasyLocated<X>
{
    #[inline] fn borrow_mut(&mut self) -> &mut X {
        &mut self.inner
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

impl<'a,X> Into<Range<usize>> for &'a EasyLocated<X>
{
    #[inline]
    fn into(self) -> Range<usize> { self.loc.clone() }
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


impl<X:PartialEq<X>> PartialEq<X> for EasyLocated<X>
{
    #[inline]
    fn eq(&self, other: &X) -> bool {
        <X as PartialEq<X>>::eq(&self.inner, &other)
    }
    #[inline]
    fn ne(&self, other: &X) -> bool {
        <X as PartialEq<X>>::eq(&self.inner, &other)
    }
}

impl<X:PartialEq<X>> PartialEq<EasyLocated<X>> for EasyLocated<X>
{
    #[inline] fn eq(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialEq<X>>::eq(&self.inner, &other.inner)
    }
    #[inline] fn ne(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialEq<X>>::ne(&self.inner, &other.inner)
    }
}

impl<X:Eq> Eq for EasyLocated<X>  {}

impl<X:Hash> Hash for EasyLocated<X>
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        <X as Hash>::hash(&self.inner, state)
    }
}


impl<X:PartialOrd<X>> PartialOrd<X> for EasyLocated<X>
{
    #[inline]
    fn partial_cmp(&self, other: &X) -> Option<Ordering> {
        <X as PartialOrd<X>>::partial_cmp(&self.inner, &other)
    }

    #[inline]
    fn lt(&self, other: &X) -> bool {
        <X as PartialOrd<X>>::lt(&self.inner, &other)
    }

    #[inline]
    fn le(&self, other: &X) -> bool {
        <X as PartialOrd<X>>::le(&self.inner, &other)
    }

    #[inline]
    fn gt(&self, other: &X) -> bool {
        <X as PartialOrd<X>>::gt(&self.inner, &other)
    }

    #[inline]
    fn ge(&self, other: &X) -> bool {
        <X as PartialOrd<X>>::ge(&self.inner, &other)
    }
}


impl<X:PartialOrd<X>> PartialOrd<EasyLocated<X>> for EasyLocated<X>
{
    #[inline]
    fn partial_cmp(&self, other: &EasyLocated<X>) -> Option<Ordering> {
        <X as PartialOrd<X>>::partial_cmp(&self.inner, &other.inner)
    }

    #[inline]
    fn lt(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialOrd<X>>::lt(&self.inner, &other.inner)
    }

    #[inline]
    fn le(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialOrd<X>>::le(&self.inner, &other.inner)
    }

    #[inline]
    fn gt(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialOrd<X>>::gt(&self.inner, &other.inner)
    }

    #[inline]
    fn ge(&self, other: &EasyLocated<X>) -> bool {
        <X as PartialOrd<X>>::ge(&self.inner, &other.inner)
    }
}


impl<X:Ord> Ord for EasyLocated<X>
{
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        <X as Ord>::cmp(&self.inner, &other)
    }
}

impl<X:Default> Default for EasyLocated<X>
{
    #[inline]
    fn default() -> Self {
        Self { inner: X::default(), loc: 0..0 }
    }
}

use std::fmt::Display;

impl<X:Display> Display for EasyLocated<X>
{
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <X as Display>::fmt(&self.inner, f)
    }
}


pub trait EasyLocator {
    fn locate<X>(&self, x:X) -> EasyLocated<X>;
}

impl<'a> EasyLocator for Range<usize> {

    #[inline]
    fn locate<X>(&self, x: X) -> EasyLocated<X> {
        EasyLocated::new(x, (*self).clone())
    }
}

impl<'a> EasyLocator for &'a Range<usize> {

    #[inline]
    fn locate<X>(&self, x: X) -> EasyLocated<X> {
        EasyLocated::new(x, (*self).clone())
    }
}

impl<'a,Y> EasyLocator for EasyLocated<Y> {

    #[inline]
    fn locate<X>(&self, x: X) -> EasyLocated<X> {
        EasyLocated::new(x, self.loc.clone())
    }
}

impl<'a,Y> EasyLocator for &'a EasyLocated<Y> {

    #[inline]
    fn locate<X>(&self, x: X) -> EasyLocated<X> {
        EasyLocated::new(x, self.loc.clone())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::{EasyLocated, EasyLocator};

    #[test]
    fn mapping()
    {
        let x = (0..2).locate(2.5);
        let y: EasyLocated<_> =  x.map(|x| (x * 2.) as u32);

        assert_eq!( *y, 5);
        assert_eq!( *y.location(), 0..2);

        let x = (0..2).locate(Some(2));
        let y = x.transpose().unwrap();
        assert_eq! ( *y, 2);
    }
}