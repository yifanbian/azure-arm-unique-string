use std::convert::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn unique_string(input: &str) -> String {
    let h = murmur2_64b(input.as_bytes(), 0);
    base32_encode(h)
}

fn base32_encode(input: u64) -> String {
    const CHARSET: &'static [u8; 32] = b"abcdefghijklmnopqrstuvwxyz234567";
    let mut ret = Vec::with_capacity(13);
    let mut c = input;
    for _ in 0..13 {
        ret.push(CHARSET[(c >> 59) as usize]);
        c <<= 5;
    }
    String::from_utf8(ret).unwrap()
}

fn murmur2_64b(data: &[u8], seed: u32) -> u64 {
    const C1: u32 = 0x239b961b;
    const C2: u32 = 0xab0e9789;
    const C3: u32 = 0x561ccd1b;
    const C4: u32 = 0x0bcaa747;
    const C5: u32 = 0x85ebca6b;
    const C6: u32 = 0xc2b2ae35;
    let length = data.len();

    let mut h1: u32 = seed;
    let mut h2: u32 = seed;
    let mut index: usize = 0;
    while index + 7 < length {
        let mut k1: u32 = u8_slice_to_u32(&data[index..(index + 4)]);
        let mut k2: u32 = u8_slice_to_u32(&data[(index + 4)..(index + 8)]);

        k1 *= C1;
        k1 = rotate_left32(k1, 15);
        k1 *= C2;
        h1 ^= k1;
        h1 = rotate_left32(h1, 19);
        h1 += h2;
        h1 = (h1 * 5) + C3;

        k2 *= C2;
        k2 = rotate_left32(k2, 17);
        k2 *= C1;
        h2 ^= k2;
        h2 = rotate_left32(h2, 13);
        h2 += h1;
        h2 = (h2 * 5) + C4;
        
        index += 8;
    }

    let tail = length - index;
    if tail > 0 {
        let mut k1: u32 = u8_slice_to_u32(&data[index..]);
        k1 *= C1;
        k1 = rotate_left32(k1, 15);
        k1 *= C2;
        h1 ^= k1;

        if tail > 4 {
            let mut k2 : u32 = u8_slice_to_u32(&data[(index+4)..]);
            k2 *= C2;
            k2 = rotate_left32(k2, 17);
            k2 *= C1;
            h2 ^= k2;
        }
    }

    h1 ^= length as u32;
    h2 ^= length as u32;

    h1 += h2;
    h2 += h1;

    h1 ^= h1 >> 16;
    h1 *= C5;
    h1 ^= h1 >> 13;
    h1 *= C6;
    h1 ^= h1 >> 16;

    h2 ^= h2 >> 16;
    h2 *= C5;
    h2 ^= h2 >> 13;
    h2 *= C6;
    h2 ^= h2 >> 16;

    h1 += h2;
    h2 += h1;
    
    ((h2 as u64) << 32) | (h1 as u64)
}

fn u8_slice_to_u32(v: &[u8]) -> u32 {
    if v.len() >= 4 {
        let (int_bytes, _) = v.split_at(std::mem::size_of::<u32>());
        u32::from_le_bytes(int_bytes.try_into().unwrap())
    }
    else {
        let mut buf: [u8; 4] = [0, 0, 0, 0];
        buf[..(v.len())].clone_from_slice(v);
        u32::from_le_bytes(buf)
    }
}

fn rotate_left32(v: u32, n: usize) -> u32 {
    (v << n) | (v >> (32 - n))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use crate::unique_string;
        let out = unique_string("world");
        assert_eq!(out, "iw4p5eg6q6hta");
    }
}