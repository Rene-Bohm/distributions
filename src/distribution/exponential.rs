use super::{Call, Discrete, Set, Uniform, Unsigned};

pub struct Exponential<U, T>
where
    T: Discrete + Unsigned,
    U: Discrete + PartialOrd,
{
    generator: Box<Uniform<f64>>,
    lambda: T,
    upper_bound: U,
    lower_bound: U,
}

impl<U, T> Exponential<U, T>
where
    T: Discrete + Unsigned,
    U: Discrete + PartialOrd,
{
    pub fn new(seed: u64, lambda: T, lower: U, upper: U) -> Self {
        let gen = Uniform::new(seed, 0.0, 1.0);

        Exponential {
            generator: Box::new(gen),
            lambda,
            upper_bound: upper,
            lower_bound: lower,
        }
    }

    pub fn change_lambda(&mut self, lambda: T) {
        self.lambda = lambda
    }

    pub fn change_bound(&mut self, lower: U, upper: U) {
        self.upper_bound = upper;
        self.lower_bound = lower;
    }
}

impl Call<u64> for Exponential<u64, u64> {
    fn call(&mut self) -> u64 {
        let val = self.generator.call();
        (-(1.0 / self.lambda as f64) * f64::ln(1.0 - val)) as u64
    }
}

impl Set<u64> for Exponential<u64, u64> {
    fn set(&mut self, samples: usize) -> Vec<u64> {
        let mut buffer = Vec::<u64>::with_capacity(samples);

        for _sample in 0..samples {
            let val = self.generator.call();

            buffer.push((-(1.0 / self.lambda as f64) * f64::ln(1.0 - val)) as u64);
        }

        buffer
    }
}
