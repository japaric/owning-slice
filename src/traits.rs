use core::slice;

use as_slice::AsSlice;

use crate::sealed;

/// `slice[start..end]` but by value
pub trait IntoSlice<I>: AsSlice {
    /// The result of the slicing operation
    type Slice: AsSlice<Element = <Self as AsSlice>::Element>
        + IntoSlice<I>
        + IntoSliceFrom<I>
        + IntoSliceTo<I>
        + Truncate<I>;

    /// `slice[start..end]` but by value
    fn into_slice(self, start: I, length: I) -> Self::Slice;
}

impl<'a, T, I> IntoSlice<I> for &'a [T]
where
    I: sealed::Index,
{
    type Slice = &'a [T];

    fn into_slice(self, start: I, length: I) -> Self::Slice {
        let start = start.into();
        let end = start + length.into();
        &self[start..end]
    }
}

impl<'a, T, I> IntoSlice<I> for &'a mut [T]
where
    I: sealed::Index,
{
    type Slice = &'a mut [T];

    fn into_slice(self, start: I, length: I) -> Self::Slice {
        let start = start.into();
        let end = start + length.into();
        &mut self[start..end]
    }
}

/// `slice[start..]` but by value
pub trait IntoSliceFrom<I>: AsSlice {
    /// The result of the slicing operation
    type SliceFrom: AsSlice<Element = <Self as AsSlice>::Element>
        + IntoSlice<I>
        + IntoSliceFrom<I>
        + IntoSliceTo<I>;

    /// `slice[start..]` but by value
    fn into_slice_from(self, start: I) -> Self::SliceFrom;
}

impl<'a, T, I> IntoSliceFrom<I> for &'a [T]
where
    I: sealed::Index,
{
    type SliceFrom = &'a [T];

    fn into_slice_from(self, start: I) -> Self::SliceFrom {
        &self[start.into()..]
    }
}

impl<'a, T, I> IntoSliceFrom<I> for &'a mut [T]
where
    I: sealed::Index,
{
    type SliceFrom = &'a mut [T];

    fn into_slice_from(self, start: I) -> Self::SliceFrom {
        &mut self[start.into()..]
    }
}

/// `slice[..end]` but by value
pub trait IntoSliceTo<I>: AsSlice {
    /// The result of the slicing operation
    type SliceTo: AsSlice<Element = <Self as AsSlice>::Element>
        + IntoSlice<I>
        + IntoSliceFrom<I>
        + IntoSliceTo<I>
        + Truncate<I>;

    /// `slice[..end]` but by value
    fn into_slice_to(self, end: I) -> Self::SliceTo;
}

impl<'a, T, I> IntoSliceTo<I> for &'a [T]
where
    I: sealed::Index,
{
    type SliceTo = &'a [T];

    fn into_slice_to(self, end: I) -> Self::SliceTo {
        &self[..end.into()]
    }
}

impl<'a, T, I> IntoSliceTo<I> for &'a mut [T]
where
    I: sealed::Index,
{
    type SliceTo = &'a mut [T];

    fn into_slice_to(self, end: I) -> Self::SliceTo {
        &mut self[..end.into()]
    }
}

/// Truncate a slice in place
pub trait Truncate<I> {
    /// Shortens the slice to the requested `length`
    ///
    /// This is a no-operation if `length` > `self.len()`
    fn truncate(&mut self, length: I);
}

impl<'a, T, I> Truncate<I> for &'a [T]
where
    I: sealed::Index,
{
    fn truncate(&mut self, len: I) {
        let end = len.into();

        if end < self.len() {
            *self = &self[..end]
        }
    }
}

impl<'a, T, I> Truncate<I> for &'a mut [T]
where
    I: sealed::Index,
{
    fn truncate(&mut self, len: I) {
        let end = len.into();

        if end < self.len() {
            *self = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), end) };
        }
    }
}
