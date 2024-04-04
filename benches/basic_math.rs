use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg_benchmark::*;


pub fn criterion_benchmark(c: &mut Criterion) {

    //assert_eq!(simplify("(+ (+ (+ 0 (* (* 1 foo) 0)) (* a 0)) a)"), "a");
    c.bench_function(
        "simplify foo",
        |b| b.iter(||
            simplify(black_box("(+ (+ (+ 0 (* (* 1 foo) 0)) (* a 0)) a)"))
        )
    );

    //assert_eq!(simplify("(+ a (+ b (+ (* 0 c) d)))"), "(+ d (+ b a))");
    c.bench_function(
        "simplify abc",
        |b| b.iter(||
            simplify(black_box("(+ a (+ b (+ (* 0 c) d)))"))
        )
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

