use std::array;

pub struct State{

    state: [u64; 4],
    old_state: [u64; 4],
    recent_rand: Option<u64>,

}

impl State{

    pub fn new(buffer: [u64; 4]) -> Self{

        State { state: buffer, old_state: buffer, recent_rand: None }

    }

    pub fn call(&mut self) -> u64{

        let result: u64 = rol64(self.state[1].overflowing_mul(5).0 as u32 , 7).overflowing_mul(9).0;
        let t: u64 = self.state[1].rotate_left(17);

        self.state[2] ^= self.state[0];
        self.state[3] ^= self.state[1];
        self.state[1] ^= self.state[2];
        self.state[0] ^= self.state[3];

        self.state[2] = t;
        self.state[3] = rol64(self.state[3] as u32, 45);

        self.recent_rand = Some(result);
        result

    }

    pub fn reset(&mut self){

        self.state = self.old_state;
        self.recent_rand = None;

    }

    pub fn recent(&self) -> Option<u64>{

        self.recent_rand
    }



}

pub fn rol64(shift: u32, buffer: u64 ) -> u64{

    if shift<64 {

        //println!("{}", buffer.rotate_left(shift) | buffer.rotate_right(64 - shift));
        buffer.rotate_left(shift) | buffer.rotate_right(64 - shift)

    }else{

        //println!("{}", buffer.rotate_left(shift) |  buffer.rotate_left(shift - 64));
        buffer.rotate_left(shift) | buffer.rotate_left(shift - 64)

    }

    

    


}