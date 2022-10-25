pub struct splitmix64_state {
    pub seed: u64,
}

impl splitmix64_state {
    pub fn call(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0x9E3779B97f4A7C15);
        let mut result: u64 = self.seed;
        result = (result ^ (result.rotate_right(30))).wrapping_mul(0xBF58476D1CE4E5B9);
        result = (result ^ (result.rotate_right(27))).wrapping_mul(0x94D049BB133111EB);
        result ^ (result.rotate_right(31))
    }
}

#[cfg(test)]
mod test {
    use super::splitmix64_state;

    #[test]
    fn split() {
        let mut gen = splitmix64_state { seed: 4 };

        println!("{:?}", &gen.call());
    }
}
