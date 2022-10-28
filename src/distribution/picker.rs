use crate::pseudo::{Shiro, Splitmix64};

pub struct Picker {
    rand: Box<Shiro>,
}

impl Picker {
    pub fn new(seed: u64) -> Self {
        let mut splitmix = Splitmix64 { seed: seed };

        let shiro = Shiro::new(splitmix.call_256());

        Picker {
            rand: Box::new(shiro),
        }
    }

    pub fn pick<T>(&mut self, set: &Vec<T>) -> T
    where
        T: Clone,
    {
        if set.len() == 0 {
            panic!("Set Length is zero!");
        } else {
            let index = self.rand.call() % set.len() as u64;

            set[index as usize].clone()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Picker;

    #[test]
    fn pick() {
        let mut picker = Picker::new(3);

        let buffer = vec![1, 2, 3, 4, 5, 6];
        for _i in 0..20 {
            println!("{:?}", picker.pick(&buffer))
        }
    }
}
