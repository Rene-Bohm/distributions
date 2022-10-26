pub struct Splitmix64 {
    pub seed: u64,
}

impl Splitmix64 {
    pub fn call(&mut self) -> u64 {
        self.seed = self.seed.wrapping_add(0x9E3779B97f4A7C15);
        let mut result: u64 = self.seed;
        result = (result ^ (result.rotate_right(30))).wrapping_mul(0xBF58476D1CE4E5B9);
        result = (result ^ (result.rotate_right(27))).wrapping_mul(0x94D049BB133111EB);
        result ^ (result.rotate_right(31))
    }

    pub fn call_256(&mut self) -> [u64; 4] {
        let mut buffer: [u64; 4] = [0, 0, 0, 0];

        for i in 0..4 {
            buffer[i] = self.call();
        }

        buffer
    }
}

#[cfg(test)]
mod test {
    use super::Splitmix64;

    #[test]
    fn split() {
        let mut gen = Splitmix64 { seed: 4 };

        println!("{:?}", &gen.call());
    }
}
