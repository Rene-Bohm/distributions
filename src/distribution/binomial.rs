use crate::pseudo::{Shiro, Splitmix64};

use super::{Call, Set};

pub struct Binomial<T> {
    rand: Box<Shiro>,
    num_of_trials: usize,
    propability: f64,
    output: Option<Vec<T>>,
}

impl<T> Binomial<T> {
    pub fn new(seed: u64, trials: usize, prob: f64) -> Self {
        let mut splitmix = Splitmix64 { seed: seed };

        let shiro = Shiro::new(splitmix.call_256());

        Binomial {
            rand: Box::new(shiro),
            num_of_trials: trials,
            propability: prob,
            output: None,
        }
    }

    pub fn change(&mut self, trials: usize, prob: f64) {
        self.num_of_trials = trials;
        self.propability = prob;
    }
}

macro_rules! binom_call_int {
    ($ty:ty) => {
        impl Call<$ty> for Binomial<$ty> {
            fn call(&mut self) -> $ty {
                let mut buffer: $ty = 0 as $ty;

                for _round in 0..self.num_of_trials {
                    let tmp = self.rand.call_f64();

                    buffer += if tmp <= self.propability {
                        1 as $ty
                    } else {
                        0 as $ty
                    }
                }

                buffer
            }
        }
    };
}

binom_call_int!(u64);
binom_call_int!(u32);
binom_call_int!(u16);
binom_call_int!(u8);
binom_call_int!(i64);
binom_call_int!(i32);
binom_call_int!(i16);
binom_call_int!(i8);

macro_rules! binom_set_int {
    ($ty:ty) => {
        impl Set<$ty> for Binomial<$ty> {
            fn set(&mut self, samples: usize) -> Vec<$ty> {
                let mut output: Vec<$ty> = Vec::with_capacity(samples);

                for _ith in 0..samples {
                    let mut buffer: $ty = 0 as $ty;
                    for _trial in 0..self.num_of_trials {
                        let tmp = self.rand.call_f64();

                        buffer += if tmp <= self.propability {
                            1 as $ty
                        } else {
                            0 as $ty
                        }
                    }
                    output.push(buffer);
                }

                output
            }
        }
    };
}

binom_set_int!(u64);
binom_set_int!(u32);
binom_set_int!(u16);
binom_set_int!(u8);
binom_set_int!(i64);
binom_set_int!(i32);
binom_set_int!(i16);
binom_set_int!(i8);

#[cfg(test)]
mod test {
    use crate::distribution::*;

    #[test]
    fn call() {
        let mut bin = Binomial::<u64>::new(1337, 100, 0.75);

        println!("{}", bin.call());
    }

    #[test]
    fn set() {
        let mut bin = Binomial::<u64>::new(1337, 100, 0.75);

        println!("{:?}", bin.set(15));
    }
}
