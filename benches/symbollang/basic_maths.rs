use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::*;
use egg_benchmark::{simplify, EGraphSize};
use log::{warn};

define_language! {
    pub enum BasicMath {
        Num(i32),
        "+" = Add([Id; 2]),
        "-" = Sub([Id; 2]),
        "-" = Neg([Id; 1]),
        "*" = Mul([Id; 2]),
        "^" = Pow([Id; 2]),
        "inv" = Inv([Id; 1]),
        Symbol(Symbol),
    }
}

pub fn basic_maths_rules() -> Vec<Rewrite<SymbolLang, ()>> {
    vec![
        // monoid +
        rewrite!("commute-add"; "(+ ?a ?b)" => "(+ ?b ?a)"),
        rewrite!("assoc-add-1"; "(+ (+ ?a ?b) ?c)" => "(+ ?a (+ ?b ?c))"),
        rewrite!("assoc-add-2"; "(+ ?a (+ ?b ?c))" => "(+ (+ ?a ?b) ?c)"),
        rewrite!("add-0"; "(+ ?a 0)" => "?a"),

        // monoid *
        rewrite!("commute-mul"; "(* ?a ?b)" => "(* ?b ?a)"),
        rewrite!("assoc-mul-1"; "(* (* ?a ?b) ?c)" => "(* ?a (* ?b ?c))"),
        rewrite!("assoc-mul-2"; "(* ?a (* ?b ?c))" => "(* (* ?a ?b) ?c)"),
        rewrite!("mul-a0"; "(* ?a 0)" => "0"),
        rewrite!("mul-0a"; "(* 0 ?a)" => "0"),
        rewrite!("mul-a1"; "(* ?a 1)" => "?a"),
        rewrite!("mul-1a"; "(* 1 ?a)" => "?a"),

        // minus
        rewrite!("minus-0"; "(- ?a ?a)" => "0"),
        rewrite!("minus-1"; "(+ ?a (- ?b))" => "(- ?a ?b)"),

        // distributive
        rewrite!("distr-abc"; "(* ?a (+ ?b ?c))" => "(+ (* ?a ?b) (* ?a ?c))"),
        rewrite!("distr-aba"; "(+ ?a (* ?b ?a))" => "(* (+ ?b 1)  ?a)"),

        // powers
        //(y^n) * y --> y^(n + 1)
        rewrite!("power-yny"; "(* (^ ?y ?n) ?y)" => "(^ ?y (+ ?n 1))"),
        //x^n * x^m == x^(n + m)
        rewrite!("power-ynym-1"; "(* (^ ?y ?n) (^ ?y ?m))" => "(^ ?y (+ ?n ?m))"),
        rewrite!("power-ynym-2"; "(^ ?y (+ ?n ?m))" => "(* (^ ?y ?n) (^ ?y ?m))"),
        //(x * y)^z == x^z * y^z
        rewrite!("power-distr-1"; "(^ (* ?x ?y) ?z)" => "(* (^ ?x ?z) (^ ?y ?z))"),
        rewrite!("power-distr-2"; "(* (^ ?x ?z) (^ ?y ?z))" => "(^ (* ?x ?y) ?z)"),
        //(x^p)^q == x^(p * q)
        rewrite!("power-power-1"; "(^ (^ ?x ?p) ?q)" => "(^ ?x (+ ?p ?q))"),
        rewrite!("power-power-2"; "(^ ?x (+ ?p ?q))" => "(^ (^ ?x ?p) ?q)"),
        //x^0 --> 1
        rewrite!("power-x0"; "(^ ?x 0)" => "1"),
        //0^x --> 0
        rewrite!("power-0x"; "(^ 0 ?x)" => "0"),
        //1^x --> 1
        rewrite!("power-1x"; "(^ 1 ?x)" => "1"),
        //x^1 --> x
        rewrite!("power-x1"; "(^ 1 ?x)" => "?x"),
        //inv(x) == x^(-1)
        rewrite!("power-inv"; "(inv ?x)" => "(^ ?x (- 1))")
    ]
}


pub fn basic_maths_benchmark(c: &mut Criterion) {
    let rules = basic_maths_rules();
    let expr = "(+ a (+ b (+ (* 0 c) d)))".parse().unwrap();
    
    c.bench_function(
        "basic_maths/simpl1",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let (result,itersize) = simplify(black_box(&expr), black_box(&rules), 8);
                size=itersize;
                result
                //assert_eq!(result, "(+ d (+ b a))");
            });
            warn!("basic_maths/simpl1 {}", size);
        }
    );

    let expr = "(+ (+ (+ 0 (* (* 1 foo) 0)) (* a 0)) a)".parse().unwrap();
    c.bench_function(
        "basic_maths/simpl2",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let (result,itersize) = simplify(black_box(&expr), black_box(&rules), 8);
                size=itersize;
                assert_eq!(result, "a".parse().unwrap());
            });
            warn!("basic_maths/simpl2 {}", size);
        }
    );
}

criterion_group!(benches, basic_maths_benchmark);
criterion_main!(benches);
