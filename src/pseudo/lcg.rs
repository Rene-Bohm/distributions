pub struct Lcg {
    mult: u32,
    rest: u32,
    seed: u32,
    size: usize,
}

impl Lcg {
    pub fn instantiate(mul: u32, seed: u32, size: usize) -> Self {
        Lcg {
            mult: mul,
            rest: u32::MAX,
            seed,
            size,
        }
    }

    pub fn call(&self) -> Vec<f64> {
        let mut output = Vec::<f64>::with_capacity(self.size);

        let mut x = (self.mult.overflowing_mul(self.seed).0 + 1) % self.rest;

        output.push(x as f64 / self.rest as f64);

        for _i in 1..self.size {
            x = (self.mult.overflowing_mul(x).0 + 1) % self.rest;
            output.push(x as f64 / self.rest as f64);
        }
        output
    }
}
