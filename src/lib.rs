pub trait Haystack {
    fn contains(&self, needle: &[u8]) -> bool;
}

pub mod window {
    pub struct WindowHaystack {
        haystack: Vec<u8>,
    }
    impl WindowHaystack {
        pub fn new(haystack: Vec<u8>) -> WindowHaystack {
            WindowHaystack { haystack }
        }
    }
    impl ::Haystack for WindowHaystack {
        fn contains(&self, needle: &[u8]) -> bool {
            self.haystack.windows(needle.len()).any(|h| h == needle)
        }
    }
}

pub mod rabin_karp {
    pub struct RabinKarpHaystack {
        haystack: Vec<u8>,
    }
    impl RabinKarpHaystack {
        pub fn new(haystack: Vec<u8>) -> RabinKarpHaystack {
            RabinKarpHaystack { haystack }
        }
    }

    const RADIX: u32 = 255;

    impl ::Haystack for RabinKarpHaystack {
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
        use Haystack;

        fn contains(haystack: &str, needle: &str) -> bool {
            RabinKarpHaystack::new(haystack.as_bytes().to_vec()).contains(needle.as_bytes())
        }
        fn assert_contains(haystack: &str, needle: &str) {
            assert!(
                contains(haystack, needle),
                "{} was supposed to contain {}",
                haystack,
                needle
            );
        }
        fn assert_not_contains(haystack: &str, needle: &str) {
            assert!(
                !contains(haystack, needle),
                "{} was not supposed to contain {}",
                haystack,
                needle
            );
        }

        #[test]
        fn smoke() {
            assert_contains("asdf", "as");
            assert_contains("asdf", "sd");
            assert_contains("asdf", "df");
            assert_contains("asdf", "asdf");
            assert_contains("asdf", "");

            assert_not_contains("asdf", "pqrs");
            assert_not_contains("asdf", "asdd");
            assert_not_contains("asdf", "asdfpqrs");
        }
    }
}
