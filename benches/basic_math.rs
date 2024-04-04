use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::*;
use egg_benchmark::*;


/// parse an expression, simplify it using egg, and pretty print it back out
pub fn simplify(s: &str) -> String {
    // parse the expression, the type annotation tells it which Language to use
    let expr: RecExpr<BasicMath> = s.parse().unwrap();

    // simplify the expression using a Runner, which creates an e-graph with
    // the given expression and runs the given rules over it
    let runner = Runner::default().with_expr(&expr).run(&basic_math_rules());

    // the Runner knows which e-class the expression given with `with_expr` is in
    let root = runner.roots[0];

    // use an Extractor to pick the best element of the root eclass
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best) = extractor.find_best(root);
    best.to_string()
}


pub fn criterion_benchmark(c: &mut Criterion) {
    //assert_eq!(simplify("(+ a (+ b (+ (* 0 c) d)))"), "(+ d (+ b a))");
    c.bench_function(
        "simpl1",
        |b| b.iter(||
            simplify(black_box("(+ a (+ b (+ (* 0 c) d)))"))
        )
    );

    //assert_eq!(simplify("(+ (+ (+ 0 (* (* 1 foo) 0)) (* a 0)) a)"), "a");
    c.bench_function(
        "foo",
        |b| b.iter(||
            simplify(black_box("(+ (+ (+ 0 (* (* 1 foo) 0)) (* a 0)) a)"))
        )
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

