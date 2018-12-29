use core::iter::{FromIterator, IntoIterator};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};
use core::slice::{Iter, IterMut};

use generic_array::{ArrayLength, GenericArray};

pub trait Magma
where
    Self: Clone + Mul<Self, Output = Self>,
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

impl<Scalar, Base, Size, Kind> Default for Vector<Scalar, Base, Size, Kind>
where
    Scalar: Magma,
    Base: Float<Scalar> + Default,
    Size: ArrayLength<Base>,
    Kind: VariantKind,
{
    fn default() -> Self {
        use core::iter;

        iter::repeat(Base::default()).collect()
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
#[allow(unused_imports)]
mod tests {
    use super::{Column, Contravariant, Covariant, Float, Row, VariantKind, Vector};
    use generic_array::typenum::*;

    pub mod tools {
        use super::super::{Float, Magma, VariantKind, Vector};
        use core::ops::Neg;
        use generic_array::typenum::*;

        pub trait Zero {
            fn zero() -> Self;
        }

        pub trait One {
            fn one() -> Self;
        }

        impl Zero for f32 {
            fn zero() -> Self {
                0.0
            }
        }

        impl One for f32 {
            fn one() -> Self {
                1.0
            }
        }

        // three dimensional Levi-Civita symbol
        pub fn e3<S, V>() -> Vector<S, Vector<S, Vector<S, S, U3, V>, U3, V>, U3, V>
        where
            V: VariantKind,
            S: Float<S> + Magma + Zero + One + Neg<Output = S>,
        {
            (0..3)
                .map(|i| {
                    (0..3)
                        .map(|j| {
                            (0..3)
                                .map(|k| {
                                    if i == j || j == k || k == i {
                                        S::zero()
                                    } else {
                                        if (i + 1) % 3 == j {
                                            S::one()
                                        } else {
                                            -S::one()
                                        }
                                    }
                                })
                                .collect()
                        })
                        .collect()
                })
                .collect()
        }

        pub fn metric3<S, V>() -> Vector<S, Vector<S, S, U3, V>, U3, V>
        where
            V: VariantKind,
            S: Float<S> + Magma + Zero + One,
        {
            (0..3)
                .map(|i| {
                    (0..3)
                        .map(|j| if i == j { S::one() } else { S::zero() })
                        .collect()
                })
                .collect()
        }
    }

    #[test]
    fn test() {
        let r: Row<f32, U3> = (0..3).map(|i| 0.1_f32 * (i as f32)).collect();
        let c: Column<f32, U3> = (0..3).map(|i| 0.4_f32 * (i as f32)).collect();
        let g = r.clone().append::<_, U3>(c.clone());
        assert_eq!(r.inner(c), g.trace())
    }

    #[test]
    fn square() {
        use self::tools;
        use rand::Rng;

        let e = tools::e3::<f32, Covariant>();

        let mut rng = rand::thread_rng();
        let a: Column<f32, U3> = [3.0 + rng.gen::<f32>(), 4.0 + rng.gen::<f32>(), 0.0]
            .iter()
            .map(Clone::clone)
            .collect();
        let b: Column<f32, U3> = [-3.0 + rng.gen::<f32>(), 4.0 + rng.gen::<f32>(), 0.0]
            .iter()
            .map(Clone::clone)
            .collect();

        // cross product
        let cross = e.inner(a.clone()).inner(b.clone());
        let cross_up = tools::metric3::<f32, Contravariant>().inner(cross.clone());
        // square
        let sq = cross_up.inner(cross).abs().sqrt() / 2.0;

        let c = a.clone() - b.clone();

        let la = tools::metric3::<f32, Covariant>()
            .inner(a.clone())
            .inner(a)
            .sqrt();
        let lb = tools::metric3::<f32, Covariant>()
            .inner(b.clone())
            .inner(b)
            .sqrt();
        let lc = tools::metric3::<f32, Covariant>()
            .inner(c.clone())
            .inner(c)
            .sqrt();

        let p = (la + lb + lc) / 2.0;
        let sq_g = (p * (p - la) * (p - lb) * (p - lc)).sqrt();

        let r = (sq - sq_g) / sq;
        assert!(r < 1e-6);
    }
}
