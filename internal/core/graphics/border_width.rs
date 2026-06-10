use core::fmt::{Debug, Formatter};
use euclid::num::Zero;
use euclid::{Length, Scale};
use num_traits::NumCast;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[repr(C)]
pub struct BorderWidth<T, U> {
    /// The top width.
    pub top: T,
    /// The right width.
    pub right: T,
    /// The bottom width.
    pub bottom: T,
    /// The left width.
    pub left: T,
    #[doc(hidden)]
    pub _unit: PhantomData<U>,
}

impl<T, U> Copy for BorderWidth<T, U> where T: Copy {}

impl<T, U> Clone for BorderWidth<T, U>
where
    T: Copy,
{
    fn clone(&self) -> Self {
        BorderWidth {
            top: self.top.clone(),
            right: self.right.clone(),
            bottom: self.bottom.clone(),
            left: self.left.clone(),
            _unit: PhantomData,
        }
    }
}

impl<T, U> Eq for BorderWidth<T, U> where T: Eq {}

impl<T, U> PartialEq for BorderWidth<T, U>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.top == other.top
            && self.right == other.right
            && self.left == other.left
            && self.bottom == other.bottom
    }
}

impl<T, U> Default for BorderWidth<T, U>
where
    T: Default,
{
    fn default() -> Self {
        BorderWidth::new(T::default(), T::default(), T::default(), T::default())
    }
}

impl<T, U> Debug for BorderWidth<T, U>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "BorderWidth({:?},{:?},{:?},{:?})", self.top, self.right, self.bottom, self.left)
    }
}

impl<T, U> Add for BorderWidth<T, U>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        BorderWidth::new(
            self.top + rhs.top,
            self.right + rhs.right,
            self.bottom + rhs.bottom,
            self.left + rhs.left,
        )
    }
}

impl<T, U> AddAssign<Self> for BorderWidth<T, U>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        self.top += rhs.top;
        self.bottom += rhs.bottom;
        self.left += rhs.left;
        self.right += rhs.right;
    }
}

impl<T, U> Sub for BorderWidth<T, U>
where
    T: Sub<T, Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        BorderWidth::new(
            self.top - rhs.top,
            self.right - rhs.right,
            self.bottom - rhs.bottom,
            self.left - rhs.left,
        )
    }
}

impl<T, U> SubAssign<Self> for BorderWidth<T, U>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.top -= rhs.top;
        self.right -= rhs.right;
        self.left -= rhs.left;
        self.bottom -= rhs.bottom;
    }
}

impl<T, U> Neg for BorderWidth<T, U>
where
    T: Neg<Output = T>,
{
    type Output = Self;
    fn neg(self) -> Self {
        BorderWidth {
            top: -self.top,
            bottom: -self.bottom,
            right: -self.right,
            left: -self.left,
            _unit: PhantomData,
        }
    }
}

impl<T, U> Mul<T> for BorderWidth<T, U>
where
    T: Copy + Mul,
{
    type Output = BorderWidth<T::Output, U>;

    #[inline]
    fn mul(self, scale: T) -> Self::Output {
        BorderWidth::new(
            self.top * scale,
            self.right * scale,
            self.bottom * scale,
            self.left * scale,
        )
    }
}

impl<T, U> MulAssign<T> for BorderWidth<T, U>
where
    T: Copy + MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, other: T) {
        self.top *= other;
        self.right *= other;
        self.bottom *= other;
        self.left *= other;
    }
}

impl<T, U1, U2> Mul<Scale<T, U1, U2>> for BorderWidth<T, U1>
where
    T: Copy + Mul,
{
    type Output = BorderWidth<T::Output, U2>;

    #[inline]
    fn mul(self, scale: Scale<T, U1, U2>) -> Self::Output {
        BorderWidth::new(
            self.top * scale.0,
            self.right * scale.0,
            self.bottom * scale.0,
            self.left * scale.0,
        )
    }
}

impl<T, U> MulAssign<Scale<T, U, U>> for BorderWidth<T, U>
where
    T: Copy + MulAssign,
{
    #[inline]
    fn mul_assign(&mut self, other: Scale<T, U, U>) {
        *self *= other.0;
    }
}

impl<T, U> Div<T> for BorderWidth<T, U>
where
    T: Copy + Div,
{
    type Output = BorderWidth<T::Output, U>;

    #[inline]
    fn div(self, scale: T) -> Self::Output {
        BorderWidth::new(
            self.top / scale,
            self.right / scale,
            self.bottom / scale,
            self.left / scale,
        )
    }
}

impl<T, U> DivAssign<T> for BorderWidth<T, U>
where
    T: Copy + DivAssign,
{
    #[inline]
    fn div_assign(&mut self, other: T) {
        self.top /= other;
        self.right /= other;
        self.bottom /= other;
        self.left /= other;
    }
}

impl<T, U1, U2> Div<Scale<T, U1, U2>> for BorderWidth<T, U2>
where
    T: Copy + Div,
{
    type Output = BorderWidth<T::Output, U1>;

    #[inline]
    fn div(self, scale: Scale<T, U1, U2>) -> Self::Output {
        BorderWidth::new(
            self.top / scale.0,
            self.right / scale.0,
            self.bottom / scale.0,
            self.left / scale.0,
        )
    }
}

impl<T, U> DivAssign<Scale<T, U, U>> for BorderWidth<T, U>
where
    T: Copy + DivAssign,
{
    fn div_assign(&mut self, other: Scale<T, U, U>) {
        *self /= other.0;
    }
}

impl<T, U> Zero for BorderWidth<T, U>
where
    T: Zero,
{
    fn zero() -> Self {
        BorderWidth::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

impl<T, U> BorderWidth<T, U> {
    /// Constructor taking a scalar for each width.
    ///
    /// Values are specified in top, right, bottom, left order following
    /// CSS conventions.
    pub const fn new(top: T, right: T, bottom: T, left: T) -> Self {
        BorderWidth { top, right, bottom, left, _unit: PhantomData }
    }

    /// Constructor taking a typed length for each radius.
    pub fn from_lengths(
        top: Length<T, U>,
        right: Length<T, U>,
        bottom: Length<T, U>,
        left: Length<T, U>,
    ) -> Self {
        BorderWidth::new(top.0, right.0, bottom.0, left.0)
    }

    pub fn new_uniform(all: T) -> Self
    where
        T: Copy,
    {
        BorderWidth::new(all, all, all, all)
    }

    // DOC: TODO: might need to implement approx_ew
    /// Returns `true` if all width values are zero.
    pub fn is_zero(&self) -> bool
    where
        T: Zero + PartialEq,
    {
        let zero = T::zero();
        self.top == zero && self.right == zero && self.bottom == zero && self.left == zero
    }

    /// Constructor taking the same typed Length for all widths.
    pub fn from_length(all: Length<T, U>) -> Self
    where
        T: Copy,
    {
        BorderWidth::new_uniform(all.0)
    }
}

impl<T, U> BorderWidth<T, U>
where
    T: NumCast + Copy,
{
    /// Cast from one numeric representation to another, preserving the units.
    #[inline]
    pub fn cast<NewT: NumCast>(self) -> BorderWidth<NewT, U> {
        self.try_cast().unwrap()
    }

    /// Fallible cast from one numeric representation to another, preserving the units.
    pub fn try_cast<NewT: NumCast>(self) -> Option<BorderWidth<NewT, U>> {
        match (
            NumCast::from(self.top),
            NumCast::from(self.right),
            NumCast::from(self.bottom),
            NumCast::from(self.left),
        ) {
            (Some(top), Some(right), Some(bottom), Some(left)) => {
                Some(BorderWidth::new(top, right, bottom, left))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use euclid::UnknownUnit;

    type BorderWidth = super::BorderWidth<f32, UnknownUnit>;

    #[test]
    fn test_eq() {
        let a = BorderWidth::new(1.0, 2.0, 3.0, 4.0);
        let b = BorderWidth::new(1.0, 2.0, 3.0, 4.0);
        let c = BorderWidth::new(0.5, 12.0, 3.0, 4.0);
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn test_zero() {
        assert!(BorderWidth::new_uniform(0.0).is_zero());
    }
}
