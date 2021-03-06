pub struct WindowHaystack {
    haystack: Vec<u8>,
}
impl ::Haystack for WindowHaystack {
    fn new(haystack: Vec<u8>) -> WindowHaystack {
        WindowHaystack { haystack }
    }
    fn contains(&self, needle: &[u8]) -> bool {
        needle.is_empty() || self.haystack.windows(needle.len()).any(|h| h == needle)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn smoke() {
        ::test::smoke::<WindowHaystack>();
    }
}
