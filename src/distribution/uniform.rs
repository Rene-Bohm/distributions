use std::fmt::Display;

use crate::pseudo::{Shiro, Splitmix64};

use super::{Call, Set};

pub struct Uniform<T> {
    rand: Box<Shiro>,
    lower_bound: T,
    upper_bound: T,
}

impl<T> Uniform<T>
where
    T: PartialOrd + /*Eq +*/ Display,
{
    pub fn new(seed: u64, lower: T, upper: T) -> Self {
        if upper < lower
        /*|| upper == lower*/
        {
            panic!(
                "bounds are not correct\n
                low: {} upper:{}",
                lower, upper
            )
        } else {
            let mut splitmix = Splitmix64 { seed: seed };

            let shiro = Shiro::new(splitmix.call_256());

            Uniform {
                rand: Box::new(shiro),
                lower_bound: lower,
                upper_bound: upper,
            }
        }
    }
}

macro_rules! uniform_implementation_int {
    ($ty:ty) => {
        impl Call<$ty> for Uniform<$ty> {
            fn call(&mut self) -> $ty {
                let diff: $ty = self.upper_bound - self.lower_bound /*+ 1*/;
                let result: $ty = (self.rand.call() % diff as u64) as $ty + self.lower_bound;

                result
            }
        }
    };
}

uniform_implementation_int!(u64);
uniform_implementation_int!(u32);
uniform_implementation_int!(u16);
uniform_implementation_int!(u8);

uniform_implementation_int!(i64);
uniform_implementation_int!(i32);
uniform_implementation_int!(i16);
uniform_implementation_int!(i8);

macro_rules! uniform_set_int {
    ($ty:ty) => {

        impl Set<$ty> for Uniform<$ty> {
            fn set(&mut self, samples: usize) -> Vec<$ty> {

                let mut output = Vec::<$ty>::with_capacity(samples);
                let diff: $ty = self.upper_bound - self.lower_bound /*+ 1*/;

                for _ in 0..samples{

                    output.push((self.rand.call() % diff as u64) as $ty + self.lower_bound)

                }

                output
            }
        }
    };
}

uniform_set_int!(u64);
uniform_set_int!(u32);
uniform_set_int!(u16);
uniform_set_int!(u8);
uniform_set_int!(i64);
uniform_set_int!(i32);
uniform_set_int!(i16);
uniform_set_int!(i8);

macro_rules! uniform_implementation_float {
    ($ty:ty) => {
        impl Call<$ty> for Uniform<$ty> {
            fn call(&mut self) -> $ty {
                let result: f64 = ((self.upper_bound - self.lower_bound) as f64
                    * self.rand.call_f64())
                    + self.lower_bound as f64;

                result as $ty
            }
        }
    };
}

uniform_implementation_float!(f64);
uniform_implementation_float!(f32);

macro_rules! uniform_set_float {
    ($ty:ty) => {

        impl Set<$ty> for Uniform<$ty> {
            fn set(&mut self, samples: usize) -> Vec<$ty> {

                let mut output = Vec::<$ty>::with_capacity(samples);
                let diff: $ty = self.upper_bound - self.lower_bound /*+ 1*/;

                for _ in 0..samples{

                    output.push((diff * self.rand.call_f64() as $ty) + self.lower_bound)

                }

                output
            }
        }
    };
}

uniform_set_float!(f64);
uniform_set_float!(f32);

#[cfg(test)]
mod test {
    use crate::distribution::*;

    use super::Uniform;

    #[test]
    fn unsigned() {
        let mut uni = Uniform::<u8>::new(132, 1, 255);

        for _ in 0..20 {
            let res = uni.call();

            println!("{}", res);
        }
    }

    #[test]
    fn float() {
        let mut uni = Uniform::<f32>::new(132, -1.5, 1.5);

        for _ in 0..20 {
            let res = uni.call();

            println!("{}", res);
        }
    }

    #[test]
    fn signed() {
        let mut uni = Uniform::<i64>::new(132, -100, 0);

        for _ in 0..20 {
            let res = uni.call();

            println!("{}", res);
        }
    }

    #[test]
    fn int_set() {
        let mut uni = Uniform::<i64>::new(132, -5, 5);

        println!("{:?}", uni.set(20));
    }

    #[test]
    fn float_set() {
        let mut uni = Uniform::<f64>::new(132, -5.0, 5.0);

        println!("{:?}", uni.set(20));
    }
}
