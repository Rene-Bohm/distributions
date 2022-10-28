use super::super::{Call, Discrete, Set, Uniform, Unsigned};

pub struct Geometric<U>
where
    U: Discrete + PartialOrd,
{
    generator: Box<Uniform<f64>>,
    probability: f64,
    upper_bound: U,
    lower_bound: U,
}

impl<U> Geometric<U>
where
    U: Discrete + PartialOrd,
{
    pub fn new(seed: u64, probability: f64, lower: U, upper: U) -> Self {
        let gen = Uniform::new(seed, 0.0, 1.0);

        Geometric {
            generator: Box::new(gen),
            probability,
            upper_bound: upper,
            lower_bound: lower,
        }
    }

    pub fn change_probability(&mut self, probability: f64) {
        self.probability = probability
    }

    pub fn change_bound(&mut self, lower: U, upper: U) {
        self.upper_bound = upper;
        self.lower_bound = lower;
    }
}

impl Call<u64> for Geometric<u64> {
    fn call(&mut self) -> u64 {
        let val = self.generator.call();
        (-(1.0 / self.probability) * f64::ln(1.0 - val)) as u64
    }
}

impl Set<u64> for Geometric<u64> {
    fn set(&mut self, samples: usize) -> Vec<u64> {

        let mut buffer = Vec::<u64>::with_capacity(samples);

        for _sample in 0..samples {
            let val = self.generator.call();
            if val < self.probability{

                buffer.push(0);
                println!("0");

            }else{

                buffer.push(((f64::ln(1.0 - val) / f64::ln(1.0 - self.probability)).ceil() - 1.0) as u64);
                println!("{}", ((f64::ln(1.0 - val) / f64::ln(1.0 - self.probability)).ceil() - 1.0));

            }
        }
        buffer
    }
}
