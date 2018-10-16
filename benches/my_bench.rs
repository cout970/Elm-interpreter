#[macro_use]
extern crate criterion;
extern crate mylib;

use criterion::Criterion;
use mylib::tokenizer::tokenize;

fn bench_tokenize_small_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_1.elm");
    c.bench_function("tokenizer_1", move |b| b.iter(|| tokenize(code)));
}

fn bench_tokenize_medium_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_2.elm");
    c.bench_function("tokenizer_2", move |b| b.iter(|| tokenize(code)));
}

criterion_group!(benches, bench_tokenize_small_file, bench_tokenize_medium_file);
criterion_main!(benches);