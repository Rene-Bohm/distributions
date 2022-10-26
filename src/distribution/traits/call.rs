pub trait Call<T> {
    fn call(&mut self) -> T;
}

pub trait Set<T> {
    fn set(&mut self, samples: usize) -> Vec<T>;
}
