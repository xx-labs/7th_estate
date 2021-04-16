//! Implementation of Lumbroso's Fast Dice Roller algorithm.
//!
//! The [Fast Dice Roller algorithm](https://arxiv.org/abs/1304.1916) optimally
//! samples the discrete uniform distribution.

#[derive(Debug)]
pub struct FastDiceRoller {
    byte_stream: Vec<u8>,
    byte_counter: usize,
    bit_counter: usize
}

impl FastDiceRoller {
    pub fn from_bytes(stream: &[u8]) -> Self {
        FastDiceRoller {
            byte_stream: stream.to_vec(),
            byte_counter: 0,
            bit_counter: 0
        }
    }

    /*
    pub fn append_bytes(&mut self, stream: &[u8]) {
        self.byte_stream.extend_from_slice(stream);
    }
    */

    pub fn next(&mut self) -> Option<u8> {
        let byte: u8 = *self.byte_stream.get(self.byte_counter)?;
        let bit: u8 = ((byte << self.bit_counter) & 0x80) >> 7;
        self.bit_counter = (self.bit_counter + 1) % 8;
        if self.bit_counter == 0 {
            self.byte_counter += 1;
        }
        Some(bit)
    }

    pub fn random(&mut self, n: u128) -> Option<u128> {
        let mut v: u128 = 1;
        let mut c: u128 = 0;
        loop {
            v = v << 1;
            c = (c << 1) + (self.next()? as u128);
            if n <= v {
                if c < n {
                    break
                } else {
                    v = v - n;
                    c = c - n;
                }
            }
        }
        Some(c)
    }
    /*
    pub fn random_interval(&mut self, min: i128, max: i128) -> Option<i128> {
        assert!(min < max);
        let range_max: u128 = (max - min) as u128;
        let value: u128 = self.random(range_max)?;
        Some((value as i128) - min)
    }
    */
}
