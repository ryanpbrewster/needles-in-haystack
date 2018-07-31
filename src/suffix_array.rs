#[derive(Default)]
pub struct SuffixArrayHaystack {
    haystack: Vec<u8>,
    suffix_positions: Vec<usize>,
}

impl ::Haystack for SuffixArrayHaystack {
    /// This is the naive SuffixArray construction, and it's much slower than more sophisticated
    /// algorithms, but it's a one-time cost.
    fn new(haystack: Vec<u8>) -> SuffixArrayHaystack {
        let mut suffix_positions: Vec<usize> = (0..haystack.len()).collect();
        suffix_positions.sort_by_key(|&n| &haystack[n..]);
        SuffixArrayHaystack {
            haystack,
            suffix_positions,
        }
    }

    fn contains(&self, needle: &[u8]) -> bool {
        let idx = self
            .suffix_positions
            .binary_search_by(|&i| self.haystack[i..].cmp(needle));
        match idx {
            Ok(_) => true,
            Err(i) => self.haystack[self.suffix_positions[i]..].starts_with(needle),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        ::test::smoke::<SuffixArrayHaystack>();
    }
}
