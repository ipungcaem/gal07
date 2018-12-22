#![no_std]
#![forbid(unsafe_code)]
#![allow(non_shorthand_field_patterns)]

use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use core::marker::PhantomData;
use core::iter::{FromIterator, IntoIterator};
use core::slice::{Iter, IterMut};

use generic_array::{ArrayLength, GenericArray};

pub trait Magma
where
    Self: Clone + Mul<Self, Output=Self>,
{
}

pub trait Float<Scalar>
where
    Self: Sized
        + Clone
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + SubAssign<Self>
        + Neg<Output = Self>
        + Mul<Scalar, Output = Self>
        + MulAssign<Scalar>,
    Scalar: Magma,
{
}

impl Magma for f32 {}

impl Float<f32> for f32 {}

impl Magma for f64 {}

impl Float<f64> for f64 {}

pub trait VariantKind {}

#[derive(Debug)]
pub struct Covariant;

#[derive(Debug)]
pub struct Contravariant;

impl VariantKind for Covariant {}

impl VariantKind for Contravariant {}

#[derive(Debug)]
pub struct Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    data: GenericArray<Base, Size>,
    phantom_data: PhantomData<(Scalar, Kind)>,
}

impl<Scalar, Base, Size, Kind> Clone for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn clone(&self) -> Self {
        match &self {
            &Vector {
                data: ref data,
                phantom_data: phantom_data,
            } => Vector {
                data: data.clone(),
                phantom_data: phantom_data.clone(),
            },
        }
    }
}

impl<Scalar, Base, Size, Kind> Add<Self> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a.clone() + b.clone())
            .collect()
    }
}

impl<Scalar, Base, Size, Kind> AddAssign<Self> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

impl<Scalar, Base, Size, Kind> Sub<Self> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(a, b)| a.clone() - b.clone())
            .collect()
    }
}

impl<Scalar, Base, Size, Kind> SubAssign<Self> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl<Scalar, Base, Size, Kind> Neg for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.into_iter().map(|a| -a.clone()).collect()
    }
}

impl<Scalar, Base, Size, Kind> Mul<Scalar> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Output = Self;

    fn mul(self, v: Scalar) -> Self::Output {
        self.into_iter().map(|a| a.clone() * v.clone()).collect()
    }
}

impl<Scalar, Base, Size, Kind> MulAssign<Scalar> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn mul_assign(&mut self, rhs: Scalar) {
        *self = self.clone() * rhs;
    }
}

impl<Scalar, Base, Size, Kind> Float<Scalar> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
}

impl<Scalar, Ephemeral, Kind> Vector<Scalar, Scalar, Ephemeral, Kind>
where
    Scalar: Magma + Float<Scalar>,
    Ephemeral: ArrayLength<Scalar>,
    Kind: VariantKind,
{
    pub fn append<Base, Size>(self, base: Base) -> Vector<Scalar, Base, Size, Kind>
    where
        Base: Float<Scalar>,
        Size: ArrayLength<Base>,
    {
        self.into_iter().map(|a| base.clone() * a.clone()).collect()
    }
}

impl<Scalar, Base, Size> Vector<Scalar, Vector<Scalar, Base, Size, Contravariant>, Size, Covariant>
where
    Scalar: Magma,
    Base: Float<Scalar> + Default,
    Size: ArrayLength<Base> + ArrayLength<Vector<Scalar, Base, Size, Contravariant>>,
{
    pub fn trace(self) -> Base {
        self.into_iter()
            .enumerate()
            .fold(Base::default(), |a, (index, data)| {
                a + data.data[index].clone()
            })
    }
}

impl<Scalar, Base, Size> Vector<Scalar, Vector<Scalar, Base, Size, Covariant>, Size, Contravariant>
where
    Scalar: Magma,
    Base: Float<Scalar> + Default,
    Size: ArrayLength<Base> + ArrayLength<Vector<Scalar, Base, Size, Covariant>>,
{
    pub fn trace(self) -> Base {
        self.into_iter()
            .enumerate()
            .fold(Base::default(), |a, (index, data)| {
                a + data.data[index].clone()
            })
    }
}

impl<Scalar, Base, Size> Vector<Scalar, Base, Size, Covariant>
where
    Scalar: Magma + Float<Scalar>,
    Base: Float<Scalar> + Default,
    Size: ArrayLength<Base> + ArrayLength<Scalar>,
{
    pub fn inner(self, other: Vector<Scalar, Scalar, Size, Contravariant>) -> Base {
        self.into_iter()
            .zip(other.into_iter())
            .fold(Base::default(), |a, (left, right)| {
                a + left.clone() * right.clone()
            })
    }
}

impl<Scalar, Base, Size> Vector<Scalar, Base, Size, Contravariant>
where
    Scalar: Magma + Float<Scalar>,
    Base: Float<Scalar> + Default,
    Size: ArrayLength<Base> + ArrayLength<Scalar>,
{
    pub fn inner(self, other: Vector<Scalar, Scalar, Size, Covariant>) -> Base {
        self.into_iter()
            .zip(other.into_iter())
            .fold(Base::default(), |a, (left, right)| {
                a + left.clone() * right.clone()
            })
    }
}

impl<'a, Scalar, Base, Size, Kind> IntoIterator for &'a Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: 'a + Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Item = &'a Base;
    type IntoIter = Iter<'a, Base>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_slice().iter()
    }
}

impl<'a, Scalar, Base, Size, Kind> IntoIterator for &'a mut Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: 'a + Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    type Item = &'a mut Base;
    type IntoIter = IterMut<'a, Base>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.as_mut_slice().iter_mut()
    }
}

impl<Scalar, Base, Size, Kind> FromIterator<Base> for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar>,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Base>,
    {
        Vector {
            data: FromIterator::from_iter(iter),
            phantom_data: PhantomData,
        }
    }
}

pub type Row<Scalar, Size> = Vector<Scalar, Scalar, Size, Covariant>;
pub type Column<Scalar, Size> = Vector<Scalar, Scalar, Size, Contravariant>;

#[cfg(test)]
#[macro_use]
extern crate std;

#[cfg(test)]
#[allow(unused_imports)]
mod test {
    use super::{Column, Contravariant, Covariant, Float, Row, VariantKind, Vector};
    use generic_array::typenum::*;

    #[test]
    fn test() {
        let r: Row<f32, U3> = (0..3).map(|i| 0.1_f32 * (i as f32)).collect();
        let c: Column<f32, U3> = (0..3).map(|i| 0.4_f32 * (i as f32)).collect();
        let g = r.clone().append::<_, U3>(c.clone());
        assert_eq!(r.inner(c), g.trace())
    }
}
