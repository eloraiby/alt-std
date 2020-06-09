pub trait Hash {
    fn hash(&self) -> usize;
}

impl Hash for &[u8] {
    fn hash(&self) -> usize {
        murmurHash64A(self, 0xcae4f57) as usize
    }
}

// from: https://github.com/antirez/redis/blob/unstable/src/hyperloglog.c
// Copyright 2014 (c) Salvatore Sanfilippo <antirez at gmail dot com> - 3-Clause BSD license
/* Our hash function is MurmurHash2, 64 bit version.
 * It was modified for Redis in order to provide the same result in
 * big and little endian archs (endian neutral). */
pub fn murmurHash64A (key: &[u8], seed: u64) -> u64 {
    let m = 0xc6a4a7935bd1e995;
    let r = 47;
    let mut h = seed ^ ((key.len() as u64).wrapping_mul(m));
    let mut i = 0;
    let len = key.len() & 7;

    let end = key.len() - len;

    while i < end {
        let mut k = key[i + 0] as u64;
        k |= (key[i + 1] as u64) << 8;
        k |= (key[i + 2] as u64) << 16;
        k |= (key[i + 3] as u64) << 24;
        k |= (key[i + 4] as u64) << 32;
        k |= (key[i + 5] as u64) << 40;
        k |= (key[i + 6] as u64) << 48;
        k |= (key[i + 7] as u64) << 56;

        k = k.wrapping_mul(m);
        k ^= k >> r;
        k = k.wrapping_mul(m);
        h ^= k;
        h = h.wrapping_mul(m);
        i += 8;
    }

    let shifts  = [0, 8, 16, 24, 32, 40, 48];
    let offsets = [0, 1, 2, 3, 4, 5, 6];
    if len != 0 {
        for i in 0..len {
            let idx = len - i - 1;
            h ^= (key[end + offsets[idx]] as u64) << shifts[idx];
        }
        h = h.wrapping_mul(m);
    }

    h ^= h >> r;
    h = h.wrapping_mul(m);
    h ^= h >> r;
    h
}
