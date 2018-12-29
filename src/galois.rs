use core::ops::Rem;
use super::tensor::{Magma, Float};

pub trait Field
where
    Self: Magma + Float<Self>,
{
}

impl Magma for i8 {}

impl Float<i8> for i8 {}

impl Field for i8 {}

pub trait Galois
where
    Self: Sized,
{
    type Inner: Field + Rem<Self::Inner, Output=Self::Inner>;
    const M: Self::Inner;

    fn from(v: Self::Inner) -> Self;
    fn into(self) -> Self::Inner;

    fn add(self, rhs: Self) -> Self {
        Self::from((self.into() + rhs.into()) % Self::M)
    }

    fn sub(self, rhs: Self) -> Self {
        Self::from((self.into() - rhs.into()) % Self::M)
    }

    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn inv(self) -> Self;
}

pub struct GF7(i8);

impl Galois for GF7 {
    type Inner = i8;
    const M: Self::Inner = 7;

    fn from(v: Self::Inner) -> Self {
        GF7(v % Self::M)
    }

    fn into(self) -> Self::Inner {
        match self { GF7(v) => v }
    }

    fn mul(self, rhs: Self) -> Self {
        let r = (Galois::into(self) as i16) * (Galois::into(rhs) as i16);
        Galois::from((r % (Self::M as i16)) as Self::Inner)
    }

    fn div(self, rhs: Self) -> Self {
        self.mul(rhs.inv())
    }

    fn inv(self) -> Self {
        match self {
            GF7(0) => panic!(),
            GF7(1) => GF7(1),
            GF7(2) => GF7(4),
            GF7(3) => GF7(5),
            GF7(4) => GF7(2),
            GF7(5) => GF7(3),
            GF7(6) => GF7(6),
            t @ _ => t,
        }
    }
}
