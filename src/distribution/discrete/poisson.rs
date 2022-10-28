use super::super::{Call, Discrete, Set, Uniform, Unsigned};

pub struct Poisson<U, T>
where
    T: Discrete + Unsigned,
    U: Discrete + PartialOrd,
{
    generator: Box<Uniform<f64>>,
    lambda: T,
    upper_bound: U,
    lower_bound: U,
}

impl<U, T> Poisson<U, T>
where
    T: Discrete + Unsigned,
    U: Discrete + PartialOrd,
{
    pub fn new(seed: u64, lambda: T, lower: U, upper: U) -> Self {
        let gen = Uniform::new(seed, 0.0, 1.0);

        Poisson {
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

impl Call<u64> for Poisson<u64, u64> {
    fn call(&mut self) -> u64 {
        let diff = self.upper_bound - self.lower_bound;

        let vec = self.generator.set((5 * self.lambda) as usize);
        let mut x = 0 as u64;
        let mut i = 0 as usize;
        let mut p = 1 as f64;

        while p >= f64::exp(-(self.lambda as f64)) {
            p = vec[i] * p;
            i += 1;
            x += 1;
        }

        x
    }
}

impl Set<u64> for Poisson<u64, u64> {
    fn set(&mut self, samples: usize) -> Vec<u64> {
        let mut buffer = Vec::<u64>::with_capacity(samples);

        for _sample in 0..samples {
            let rand = self.generator.call();
            //println!("{:?}", &vec);
            let mut x = 0 as u64;
            //println!("{:?}", x);

            //println!("{:?}", i);
            let mut p = f64::exp(-(self.lambda as f64));

            let mut s = p;

            while rand > s {
                x += 1;
                p = self.lambda as f64 * p / x as f64;
                s += p;
            }

            buffer.push(x);
        }

        buffer
    }
}

#[cfg(test)]
mod test {
    use crate::distribution::Set;

    use super::Poisson;

    #[test]
    fn set() {
        let mut exp = Poisson::<u64, u64>::new(1337, 5, u64::MIN, u64::MAX);

        println!("{:?}", exp.set(40))
    }
}
