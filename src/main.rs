use std::ops::{Div, Add};

use ark_ff::{Field, PrimeField, FpParameters, BigInteger};
use ark_bls12_381::Fq as F; // Prime Field
use ark_std::{One, Zero, UniformRand};


fn main() {
    let mut rng = ark_std::rand::thread_rng();
    // select a random value from the field
    let a = F::rand(&mut rng);
   
    let modulus = <F as PrimeField>::Params::MODULUS;
    

    // show that 1 + 1 = 2
    let b = a.div(a);
    let two = F::one().add(F::one());

    assert_eq!(b.add(b), two);

    // show that the multiplicative inverse of a number multipled by itself equals one. 
    assert_eq!(a.inverse().unwrap() * a, F::one());

    // show that a value raised to the power of the modulus is equal to itself
    assert_eq!(a.pow(modulus), a);
    // use the `pow` function to raise to a power
}