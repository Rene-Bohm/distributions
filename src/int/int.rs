pub struct bound{

    start: f64,
    end: f64,
    count: u32,
    mid: f64,

}

impl bound{

    pub fn instantiate(start: f64, end:f64) -> Self{

        bound { start: start, end: end, count: 0, mid: (start + end)/2.0}

    }

    pub fn call(&mut self, input: f64) -> Option<f64>{

        if input < self.end && input >= self.start{

            self.count += 1;
            Some(self.mid)

        }else{

            None

        }

    }

    pub fn get(&self) -> u32{

        self.count
    }
    

}