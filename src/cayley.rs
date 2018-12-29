use core::ops::{Add, AddAssign, Neg, Sub, SubAssign, Mul, MulAssign};

#[derive(Clone)]
pub struct CayleyPair<T> {
    a: T,
    b: T,
}

pub trait Conjugate {
    fn conjugate(self) -> Self;
}

impl<T> Conjugate for CayleyPair<T>
where
    T: Conjugate + Neg<Output=T>,
{
    fn conjugate(self) -> Self {
        CayleyPair {
            a: self.a.conjugate(),
            b: -self.b,
        }
    }
}

impl<T> Add<Self> for CayleyPair<T>
where
    T: Add<T, Output=T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        CayleyPair {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl<T> AddAssign<Self> for CayleyPair<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Self) {
        (*self).a += rhs.a;
        (*self).b += rhs.b;
    }
}

impl<T> Neg for CayleyPair<T>
where
    T: Neg<Output=T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        CayleyPair {
            a: -self.a,
            b: -self.b,
        }
    }
}

impl<T> Sub<Self> for CayleyPair<T>
where
    Self: Add<Self, Output=Self> + Neg<Output=Self>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl<T> SubAssign<Self> for CayleyPair<T>
where
    T: SubAssign<T>,
{
    fn sub_assign(&mut self, rhs: Self) {
        (*self).a -= rhs.a;
        (*self).b -= rhs.b;
    }
}

impl<T> Mul<Self> for CayleyPair<T>
where
    T: Clone + Conjugate + Add<T, Output=T> + Mul<T, Output=T> + Sub<T, Output=T>,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_c = self.clone();
        let rhs_c = self.clone();
        CayleyPair {
            a: self.a * rhs.a - rhs.b.conjugate() * self.b,
            b: rhs_c.b * self_c.a + self_c.b * rhs_c.a.conjugate(),
        }
    }
}

impl<T> MulAssign<Self> for CayleyPair<T>
where
    Self: Clone + Mul<Self, Output=Self>,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.clone() * rhs;
    }
}
