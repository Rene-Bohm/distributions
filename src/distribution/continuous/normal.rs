use std::fmt::Display;
use super::super::{Call,Continuous, Set, Uniform};

pub struct Normal<U>
where
    U: Continuous + PartialOrd + Display,
{
    generator: Box<Uniform<U>>,
    var: f64,
    avg: f64,
    upper_bound: U,
    lower_bound: U,
}

impl<U> Normal<U>
where
    U: Continuous + PartialOrd + Display,
{
    pub fn new(seed: u64, var:f64, avg: f64, lower: U, upper: U) -> Self {
        let gen = Uniform::<U>::new(seed, U::zero(), U::one());

        Normal {
            generator: Box::new(gen),
            var,
            avg,
            upper_bound: upper,
            lower_bound: lower,
        }
    }

    pub fn change_parameter(&mut self, avg: f64, var:f64) {
        self.avg = avg;
        self.var = var;
    }

    pub fn change_bound(&mut self, lower: U, upper: U) {
        self.upper_bound = upper;
        self.lower_bound = lower;
    }
}

macro_rules! impl_call_float {
    ($ty: ident) => {

        impl Call<$ty> for Normal<$ty> {
            fn call(&mut self) -> $ty {
                let u1 = self.generator.call();
                let u2 = self.generator.call();

                let z0 = $ty::sqrt(-2.0 * $ty::ln(u1)) * $ty::cos(6.28 * u2);
                //let z1 = $ty::sqrt(-2.0 * $ty::ln(u1)) * $ty::sin(6.28 * u2);

                z0*self.var as $ty + self.avg as $ty
               
            }
        }
        
    };
}

impl_call_float!(f32);
impl_call_float!(f64);

macro_rules! impl_set_float {
    ($ty: ident) => {
        impl Set<$ty> for Normal<$ty> {
            fn set(&mut self, samples: usize) -> Vec<$ty> {
                let mut buffer = Vec::<$ty>::with_capacity(samples);
                for _sample in 0..samples {
                    
                    let u1 = self.generator.call();
                    let u2 = self.generator.call();

                    let z0 = $ty::sqrt(-2.0 * $ty::ln(u1)) * $ty::cos(6.28 * u2);
                    //let z1 = $ty::sqrt(-2.0 * $ty::ln(u1)) * $ty::sin(6.28 * u2);

                    buffer.push(z0*self.var as $ty + self.avg as $ty)
               
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
        let mut bin = Normal::<f64>::new(1337, 0.75, 10.0, 0.0, 1.0);

        println!("{}", bin.call());
    }

    #[test]
    fn set() {
        let mut bin = Normal::<f64>::new(1337, 0.75, 10.0, 0.0, 1.0);

        println!("{:?}", bin.set(15));
    }


}