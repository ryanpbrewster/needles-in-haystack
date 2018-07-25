pub trait Haystack {
    fn new(haystack: Vec<u8>) -> Self;
    fn contains(&self, needle: &[u8]) -> bool;
}

pub mod rabin_karp;
pub mod suffix_tree;
pub mod window;

#[cfg(test)]
pub mod test {
    use Haystack;

    fn contains<H: Haystack>(haystack: &str, needle: &str) -> bool {
        H::new(haystack.as_bytes().to_vec()).contains(needle.as_bytes())
    }
    fn assert_contains<H: Haystack>(haystack: &str, needle: &str) {
        assert!(
            contains::<H>(haystack, needle),
            "{} was supposed to contain {}",
            haystack,
            needle
        );
    }
    fn assert_not_contains<H: Haystack>(haystack: &str, needle: &str) {
        assert!(
            !contains::<H>(haystack, needle),
            "{} was not supposed to contain {}",
            haystack,
            needle
        );
    }

    pub fn smoke<H: Haystack>() {
        assert_contains::<H>("asdf", "as");
        assert_contains::<H>("asdf", "sd");
        assert_contains::<H>("asdf", "df");
        assert_contains::<H>("asdf", "asdf");
        assert_contains::<H>("asdf", "");

        assert_not_contains::<H>("asdf", "pqrs");
        assert_not_contains::<H>("asdf", "asdd");
        assert_not_contains::<H>("asdf", "asdfpqrs");
    }
}
