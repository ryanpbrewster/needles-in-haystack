#[macro_use]
extern crate criterion;
extern crate needles_in_haystack;

use criterion::Criterion;

use needles_in_haystack::{
    rabin_karp::RabinKarpHaystack, suffix_tree::SuffixTreeHaystack, window::WindowHaystack,
    Haystack,
};

fn needle() -> Vec<u8> {
    let mut buf = vec![0; 99];
    buf.push(1);
    buf
}
fn haystack() -> Vec<u8> {
    let mut buf = vec![0; 999];
    buf.push(1);
    buf
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("window", |b| {
        let h = WindowHaystack::new(haystack());
        let n = needle();
        b.iter(|| h.contains(&n))
    });

    c.bench_function("rabin-karp", |b| {
        let h = RabinKarpHaystack::new(haystack());
        let n = needle();
        b.iter(|| h.contains(&n))
    });

    c.bench_function("suffix-tree", |b| {
        let h = SuffixTreeHaystack::new(haystack());
        let n = needle();
        b.iter(|| h.contains(&n))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
