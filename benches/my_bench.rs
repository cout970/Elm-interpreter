#[macro_use]
extern crate criterion;
extern crate elm_interpreter;
extern crate nom;

use criterion::Criterion;

use elm_interpreter::interpreter::dynamic_env::DynamicEnv;
use elm_interpreter::interpreter::eval_expression;
use elm_interpreter::interpreter::eval_statement;
use elm_interpreter::parsers::new::parse_full_module;
use elm_interpreter::parsers::new::util::from;
use elm_interpreter::parsers::parse_module;
use elm_interpreter::tokenizer::tokenize;
use elm_interpreter::tokenizer::TokenStream;

fn bench_tokenize_small_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_1.elm");
    c.bench_function("tokenizer_1", move |b| b.iter(|| tokenize(code)));
}

fn bench_tokenize_medium_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_2.elm");
    c.bench_function("tokenizer_2", move |b| b.iter(|| tokenize(code)));
}

fn bench_parser_small_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_1.elm");
    let tokens = tokenize(code).unwrap();
    c.bench_function("parser_1", move |b| b.iter(|| parse_module(TokenStream::new(&tokens))));
}

fn bench_parser_medium_file(c: &mut Criterion) {
    let code: &'static [u8] = include_bytes!("data/tokenizer_2.elm");
    let tokens = tokenize(code).unwrap();
    c.bench_function("parser_2", move |b| b.iter(|| parse_module(TokenStream::new(&tokens))));
}

fn bench_new_parser_small_file(c: &mut Criterion) {
    let input = from(include_str!("data/tokenizer_1.elm"));
    c.bench_function("new_parser_1", move |b| b.iter(|| parse_full_module(input.clone())));
}

fn bench_new_parser_medium_file(c: &mut Criterion) {
    let input = from(include_str!("data/tokenizer_2.elm"));
    c.bench_function("new_parser_2", move |b| b.iter(|| parse_full_module(input.clone())));
}

fn bench_eval_expr_1(c: &mut Criterion) {
    let mut env = DynamicEnv::default_lang_env();
    eval_statement(&mut env, "fib num = case num of \n 0 -> 0 \n 1 -> 1 \n _ -> fib (num - 1) + fib (num - 2)").unwrap();

    c.bench_function("eval_1", move |b| b.iter(|| eval_expression(&mut env, "fib 50")));
}

fn bench_eval_expr_2(c: &mut Criterion) {
    let mut env = DynamicEnv::default_lang_env();
    let code = "1 + 2 * 3 / 4 + 5 * 4 / 5 - 6 + 7 * 123 / 234 - 876 + 938 * 2 / 3";

    c.bench_function("eval_2", move |b| b.iter(|| eval_expression(&mut env, code)));
}

fn check_copy_cost(c: &mut Criterion) {
    c.bench_function("input_copy_cost", move |b| {
        let input = from("a long stream of tokens");
        b.iter(|| input.clone())
    });
    c.bench_function("token_stream_copy_cost", move |b| {
        let tokens = tokenize(b"a long stream of tokens").unwrap();
        let stream = TokenStream::new(&tokens);
        b.iter(|| stream.clone())
    });
}

criterion_group!(tokenizer_benches, bench_tokenize_small_file, bench_tokenize_medium_file);
criterion_group!(parser_benches, bench_parser_small_file, bench_parser_medium_file);
criterion_group!(new_parser_benches, bench_new_parser_small_file, bench_new_parser_medium_file);
criterion_group!(eval_benches, bench_eval_expr_1, bench_eval_expr_2);
criterion_group!(internal_optimizations, check_copy_cost);

criterion_main!(internal_optimizations);