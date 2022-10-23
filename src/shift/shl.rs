use std::array;

#[derive(Debug)]
pub struct csl_8{

    buffer: [bool; 8],

}

impl csl_8{

    pub fn instantiate(input: u8) -> Self{

        let mut tmp = input;
        let mut buffer:[bool; 8] = [false; 8];

        for i in 0..8{

            match tmp%2 as u8{

                0 => {buffer[i] = false;},
                _ => {buffer[i] = true;},

            }

            tmp = tmp/2 as u8;
        }

        csl_8 { buffer: buffer }

    }

    pub fn shl(&mut self) -> u8{

        let clone = self.buffer.clone();

        let mut output:u8 = 0;

        for i in 0..8{

            match i {

                7 => {self.buffer[0] = clone[7];},
                _ => {self.buffer[i+1] = clone[i]},

            }
        }

        for i in 0..8{

            match self.buffer[i] {

                true => output += (2 as u8).pow(i as u32),
                false => (),
            }

        }
       output

    }


}