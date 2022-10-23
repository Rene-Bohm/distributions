pub struct splitmix64_state {
	pub seed: u64,
}

impl splitmix64_state{

    pub fn call(& mut self) -> u64 {
        self.seed = self.seed.overflowing_add(0x9E3779B97f4A7C15).0;
        let mut result: u64 =  self.seed;
        result = (result ^ (result.rotate_right(30))).overflowing_mul(0xBF58476D1CE4E5B9).0;
        result = (result ^ (result.rotate_right(27))).overflowing_mul(0x94D049BB133111EB).0;
        result ^ (result.rotate_right(31))
    }
}