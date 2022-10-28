use std::mem;

pub struct Shiro {
    shiro: [u64; 4],
    old_shiro: [u64; 4],
    recent_rand: Option<u64>,
}

impl Shiro {
    pub fn new(buffer: [u64; 4]) -> Self {
        Shiro {
            shiro: buffer,
            old_shiro: buffer,
            recent_rand: None,
        }
    }

    pub fn call(&mut self) -> u64 {
        let shiro = &mut self.shiro;
        let result = rol64(7, shiro[1].wrapping_mul(5)).wrapping_mul(9);
        let t = shiro[1] << 17;

        shiro[2] ^= shiro[0];
        shiro[3] ^= shiro[1];
        shiro[1] ^= shiro[2];
        shiro[0] ^= shiro[3];
        shiro[2] ^= t;
        shiro[3] = rol64(45, shiro[3]);

        result
    }

    pub fn reset(&mut self) {
        self.shiro = self.old_shiro;
        self.recent_rand = None;
    }

    pub fn recent(&self) -> Option<u64> {
        self.recent_rand
    }

    pub fn call_f64(&mut self) -> f64 {
        let shiro = &mut self.shiro;
        let value = rol64(7, shiro[1].wrapping_mul(5)).wrapping_mul(9);
        let t = shiro[1] << 17;

        shiro[2] ^= shiro[0];
        shiro[3] ^= shiro[1];
        shiro[1] ^= shiro[2];
        shiro[0] ^= shiro[3];
        shiro[2] ^= t;
        shiro[3] = rol64(45, shiro[3]);

        let float_size = mem::size_of::<f64>() as u32 * 8;
        //Fraction bits = 53  + 1
        let scale = 1.0 / ((((1 as u64) << 54) as f64) - 1.0);

        let value = value >> (float_size - 54);
        scale * value as f64

        /*

        $ty:ident, $uty:ident, $f_scalar:ident, $u_scalar:ty,
         $fraction_bits:expr, $exponent_bias:expr)

         float_impls! { f64, u64, f64, u64, 52, 1023 }

        */
    }
}

pub fn rol64(shift: i16, buffer: u64) -> u64 {
    (buffer << shift) | (buffer >> (64 - shift))

    /*
    if shift < 64 {
        //println!("{}", buffer.rotate_left(shift) | buffer.rotate_right(64 - shift));
        buffer.rotate_left(shift) | buffer.rotate_right(64 - shift)
    } else {
        //println!("{}", buffer.rotate_left(shift) |  buffer.rotate_left(shift - 64));
        buffer.rotate_left(shift) | buffer.rotate_left(shift - 64)
    }
    */
}
