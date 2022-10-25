use std::array;

pub struct Shiro {
    Shiro: [u64; 4],
    old_shiro: [u64; 4],
    recent_rand: Option<u64>,
}

impl Shiro {
    pub fn new(buffer: [u64; 4]) -> Self {
        Shiro {
            Shiro: buffer,
            old_shiro: buffer,
            recent_rand: None,
        }
    }

    pub fn call(&mut self) -> u64 {
        let Shiro = &mut self.Shiro;
        let result = rol64(7, Shiro[1].wrapping_mul(5)).wrapping_mul(9);
        let t = Shiro[1] << 17;

        Shiro[2] ^= Shiro[0];
        Shiro[3] ^= Shiro[1];
        Shiro[1] ^= Shiro[2];
        Shiro[0] ^= Shiro[3];
        Shiro[2] ^= t;
        Shiro[3] = rol64(45, Shiro[3]);

        result
    }

    pub fn reset(&mut self) {
        self.Shiro = self.old_shiro;
        self.recent_rand = None;
    }

    pub fn recent(&self) -> Option<u64> {
        self.recent_rand
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

/*

const std::uint64_t result = detail::RotL(m_shiro[1] * 5, 7) * 9;
        const std::uint64_t t = m_shiro[1] << 17;
        m_shiro[2] ^= m_shiro[0];
        m_shiro[3] ^= m_shiro[1];
        m_shiro[1] ^= m_shiro[2];
        m_shiro[0] ^= m_shiro[3];
        m_shiro[2] ^= t;
        m_shiro[3] = detail::RotL(m_shiro[3], 45);
        return result;

        let result: u64 = rol64(self.Shiro[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t: u64 = self.Shiro[1].rotate_left(17);

        self.Shiro[2] ^= self.Shiro[0];
        self.Shiro[3] ^= self.Shiro[1];
        self.Shiro[1] ^= self.Shiro[2];
        self.Shiro[0] ^= self.Shiro[3];

        self.Shiro[2] = t;
        self.Shiro[3] = rol64(self.Shiro[3] as u32, 45);

        self.recent_rand = Some(result);
        result


*/
