pub struct RabinKarpHaystack {
    haystack: Vec<u8>,
}

const RADIX: u32 = 255;

impl ::Haystack for RabinKarpHaystack {
    fn new(haystack: Vec<u8>) -> RabinKarpHaystack {
        RabinKarpHaystack { haystack }
    }
    fn contains(&self, needle: &[u8]) -> bool {
        let n = needle.len();
        if n > self.haystack.len() {
            return false;
        }

        let mut needle_hash = 0;
        let mut haystack_hash = 0;
        let mut max_radix = 1;
        for i in 0..n {
            needle_hash = RADIX.wrapping_mul(needle_hash) + needle[i] as u32;
            haystack_hash = RADIX.wrapping_mul(haystack_hash) + self.haystack[i] as u32;
            if i > 0 {
                max_radix = RADIX.wrapping_mul(max_radix);
            }
        }

        if needle_hash == haystack_hash && needle == &self.haystack[..n] {
            return true;
        }

        for i in n..self.haystack.len() {
            haystack_hash = haystack_hash
                .wrapping_sub(max_radix * self.haystack[i - n] as u32)
                .wrapping_mul(RADIX) + self.haystack[i] as u32;
            if needle_hash == haystack_hash && needle == &self.haystack[i - n + 1..i + 1] {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        ::test::smoke::<RabinKarpHaystack>();
    }
}
