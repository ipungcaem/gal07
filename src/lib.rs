#![no_std]
#![forbid(unsafe_code)]
#![allow(non_shorthand_field_patterns)]

#[cfg(test)]
#[macro_use]
extern crate std;

pub mod cayley;
pub mod tensor;
pub mod galois;

#[cfg(test)]
mod tests {
    use super::cayley::CayleyPair;
    use super::galois::{Galois, GF7};

    #[test]
    fn octonion() {
        let a = CayleyPair::real(CayleyPair::imagine(CayleyPair::real(<GF7 as Galois>::from(3))));
        let b = CayleyPair::real(CayleyPair::real(CayleyPair::imagine(<GF7 as Galois>::from(4))));
        let c = CayleyPair::real(CayleyPair::imagine(CayleyPair::imagine(<GF7 as Galois>::from(2))));
        assert_eq!(c, a * b);
    }
}
