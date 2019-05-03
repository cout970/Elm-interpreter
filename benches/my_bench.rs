#[macro_use]
extern crate criterion;
extern crate elm_interpreter;
extern crate nom;

use criterion::Criterion;

use elm_interpreter::parsers::Parser;
use elm_interpreter::Runtime;
use elm_interpreter::source::SourceCode;
use elm_interpreter::tokenizer::Tokenizer;

fn bench_tokenize_small_file(c: &mut Criterion) {
    let code = SourceCode::from_slice(include_bytes!("../resources/benches/tokenizer_1.elm"));

    c.bench_function("tokenizer_1", move |b| b.iter(|| {
        Tokenizer::new(&code).tokenize().expect("Test error")
    }));
}

fn bench_tokenize_medium_file(c: &mut Criterion) {
    let code = SourceCode::from_slice(include_bytes!("../resources/benches/tokenizer_2.elm"));
    c.bench_function("tokenizer_2", move |b| b.iter(|| {
        Tokenizer::new(&code).tokenize().expect("Test error")
    }));
}

fn bench_parser_small_file(c: &mut Criterion) {
    let code = SourceCode::from_slice(include_bytes!("../resources/benches/tokenizer_1.elm"));

    c.bench_function("parser_1", move |b| b.iter(|| {
        Parser::new(Tokenizer::new(&code)).parse_module().expect("Test error")
    }));
}

fn bench_parser_medium_file(c: &mut Criterion) {
    let code = SourceCode::from_slice(include_bytes!("../resources/benches/tokenizer_2.elm"));

    c.bench_function("parser_2", move |b| b.iter(|| {
        Parser::new(Tokenizer::new(&code)).parse_module().expect("Test error")
    }));
}

fn bench_eval_expr_1(c: &mut Criterion) {
    let mut runtime = Runtime::new();
    let function = "fib num = case num of \n 0 -> 0 \n 1 -> 1 \n _ -> fib (num - 1) + fib (num - 2)";

    runtime.eval_statement(function).unwrap();

    c.bench_function("eval_1", move |b| b.iter(|| {
        runtime.eval_expr("fib 50").expect("Test error")
    }));
}

fn bench_eval_expr_2(c: &mut Criterion) {
    let mut runtime = Runtime::new();
    let code = "1 + 2 * 3 / 4 + 5 * 4 / 5 - 6 + 7 * 123 / 234 - 876 + 938 * 2 / 3";

    c.bench_function("eval_2", move |b| b.iter(|| {
        runtime.eval_expr(code).expect("Test error")
    }));
}

fn bench_runtime_init(c: &mut Criterion) {
    c.bench_function("runtime_init_1", move |b| b.iter(|| {
        Runtime::new()
    }));
}

criterion_group!(tokenizer_benches, bench_tokenize_small_file, bench_tokenize_medium_file);
criterion_group!(parser_benches, bench_parser_small_file, bench_parser_medium_file);
criterion_group!(eval_benches, bench_eval_expr_1, bench_eval_expr_2); // fails
criterion_group!(init_benches, bench_runtime_init); // takes 500 seg to run

criterion_main!(tokenizer_benches, parser_benches, init_benches);