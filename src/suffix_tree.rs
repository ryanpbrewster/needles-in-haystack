use std::collections::BTreeMap;

/// BTreeMap is ~2x faster than HashMap for this purpose
#[derive(Default)]
pub struct SuffixTreeHaystack {
    terminal: bool,
    children: BTreeMap<u8, SuffixTreeHaystack>,
}

impl ::Haystack for SuffixTreeHaystack {
    /// This is the naive SuffixTree construction, and it's much slower than more sophisticated
    /// algorithms, but it's a one-time cost. Once the suffix tree has been constructed,
    /// we can perform many matches with it.
    fn new(haystack: Vec<u8>) -> SuffixTreeHaystack {
        let mut root = SuffixTreeHaystack::default();
        for i in 0..haystack.len() {
            let mut leaf = (0..i).fold(&mut root, |cur, j| {
                cur.children.entry(haystack[j]).or_default()
            });
            leaf.terminal = true;
        }
        root
    }

    fn contains(&self, needle: &[u8]) -> bool {
        let mut cur = self;
        for ch in needle {
            match cur.children.get(ch) {
                None => return false,
                Some(child) => cur = child,
            };
        }
        cur.terminal
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        ::test::smoke::<SuffixTreeHaystack>();
    }
}
