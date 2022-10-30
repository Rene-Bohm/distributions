use std::fmt::Display;
use super::super::{Call,Continuous, Set, Uniform};

pub struct Exponential<U>
where
    U: Continuous + PartialOrd + Display,
{
    generator: Box<Uniform<U>>,
    lambda: f64,
    upper_bound: U,
    lower_bound: U,
}

impl<U> Exponential<U>
where
    U: Continuous + PartialOrd + Display,
{
    pub fn new(seed: u64, lambda: f64, lower: U, upper: U) -> Self {
        let gen = Uniform::<U>::new(seed, U::zero(), U::one());

        Exponential {
            generator: Box::new(gen),
            lambda,
            upper_bound: upper,
            lower_bound: lower,
        }
    }

    pub fn change_lambda(&mut self, lambda: f64) {
        self.lambda = lambda
    }

    pub fn change_bound(&mut self, lower: U, upper: U) {
        self.upper_bound = upper;
        self.lower_bound = lower;
    }
}

macro_rules! impl_call_float {
    ($ty: ident) => {

        impl Call<$ty> for Exponential<$ty> {
            fn call(&mut self) -> $ty {
                let val = self.generator.call();
                -1.0 / self.lambda as $ty * $ty::ln(1.0 - val)
            }
        }
        
    };
}

impl_call_float!(f32);
impl_call_float!(f64);

macro_rules! impl_set_float {
    ($ty: ident) => {
        impl Set<$ty> for Exponential<$ty> {
            fn set(&mut self, samples: usize) -> Vec<$ty> {
                let mut buffer = Vec::<$ty>::with_capacity(samples);
                for _sample in 0..samples {
                    let val = self.generator.call();
                    buffer.push(-1.0 / self.lambda as $ty * $ty::ln(1.0 - val));
                }
                buffer
            }
        }
    };
}

impl_set_float!(f64);
impl_set_float!(f32);

#[cfg(test)]
mod test{

    use crate::distribution::*;

    #[test]
    fn call() {
        let mut bin = Exponential::<f64>::new(1337, 0.75, 0.0, 1.0);

        println!("{}", bin.call());
    }

    #[test]
    fn set() {
        let mut bin = Exponential::<f64>::new(1337, 0.75, 0.0, 1.0);

        println!("{:?}", bin.set(15));
    }


}