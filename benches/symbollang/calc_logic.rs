use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::*;
use egg_benchmark::*;
use log::{warn};

// ## Theory of Calculational Logic
// https://www.cs.cornell.edu/gries/Logic/Axioms.html
// The axioms of calculational propositional logic C are listed in the order in
// which they are usually presented and taught. Note that equivalence comes
// first. Note also that, after the first axiom, we take advantage of
// associativity of equivalence and write sequences of equivalences without
// parentheses. We use == for equivalence, | for disjunction, & for conjunction,
//
// Golden rule: p & q == p == q == p | q
//
// Implication: p ⟹ q == p | q == q
// Consequence: p ⟸q == q ⟹ p
//
// Definition of false: false == !true

define_language! {
    pub enum CalcLogic {
        Num(i32),
        "==" = Eq([Id; 2]),
        "||" = Or([Id; 2]),
        "!!" = Not([Id; 1]),
        "!=" = NotEq([Id; 2]),
        "&&" = And([Id; 2]),
        "=>" = Impl([Id; 2]),
        Symbol(Symbol),
    }
}

pub fn calc_logic_rules() -> Vec<Rewrite<SymbolLang, ()>> {
    vec![
        // ((p == q) == r) == (p == (q == r))      # Associativity of ==:
        rewrite!("==-assoc"; "(== (== ?p ?q) ?r)" <=> "(== ?p (== ?q ?r))"),
        // (p == q) == (q == p)                    # Symmetry of ==:
        rewrite!("==-sym"; "(== ?p ?q)" <=> "(== ?q ?p)"),
        // !(p == q) == (!(p) == q)                # Distributivity of !:
        rewrite!("==-distr"; "(!! (== ?p ?q))" <=> "(== (!! ?p) ?q)"),
        // (p != q) == !(p == q)                   # Definition of !=:
        rewrite!("!="; "(!= ?p ?q)" <=> "(!! (== ?p ?q))"),
        // ((p || q) || r) == (p || (q || r))      # Associativity of ||:
        rewrite!("||-assoc"; "(|| (|| ?p ?q) ?r)" <=> "(|| ?p (|| ?q ?r))"),
        // (p || q) == (q || p)                    # Symmetry of ||:
        rewrite!("||-sym"; "(|| ?p ?q)" <=> "(|| ?q ?p)"),
        // (p || (q == r)) == (p || q == p || r)   # Distributivity of ||:
        rewrite!("||-distr"; "(|| ?p (== ?q ?r))" <=> "(== (|| ?p ?q) (|| ?p ?r))"),
        // !(p || q) == (!p && !q)                 # DeMorgan
        rewrite!("||-demorgan"; "(!! (|| ?p ?q))" <=> "(&& (!! ?p) (!! ?q))"),
        // !(p && q) == (!p || !q)
        rewrite!("!&&"; "(!! (&& ?p ?q))" <=> "(|| (!! ?p) (!! ?q))"),
        // (p && q) == ((p == q) == p || q)
        rewrite!("&&"; "(&& ?p ?q)" <=> "(== (== ?p ?q) (|| ?p ?q))"),
        // (p ⟹  q) == ((p || q) == q)
        rewrite!("=>"; "(=> ?p ?q)" <=> "(== (|| ?p ?q) ?q)"),
        // (q == q) --> true                       # Identity of ==:
        vec![rewrite!("==-id"; "(== ?p ?p)" => "true")],
        // (p || p) --> p                          # Idempotency of ||:
        vec![rewrite!("||-idem"; "(|| ?p ?p)" => "?p")],
        // (p || !(p)) --> true                    # Excluded Middle:
        vec![rewrite!("||-excl"; "(|| ?p (!! ?p))" => "true")],
    ]
    .concat()
}

// fold = @theory p q begin
//   (p::Bool == q::Bool) => (p == q)
//   (p::Bool || q::Bool) => (p || q)
//   (p::Bool ⟹ q::Bool)  => ((p || q) == q)
//   (p::Bool && q::Bool) => (p && q)
//   !(p::Bool)           => (!p)
// end
//
// calculational_logic_theory = calc ∪ fold

pub fn calc_logic_benchmark(c: &mut Criterion) {
    let rules = calc_logic_rules();
    let tru: RecExpr<SymbolLang> = "true".parse().unwrap();

    let demorgan: RecExpr<SymbolLang> = "(== (!! (|| p q)) (&& (!! p) (!! q)))".parse().unwrap();
    c.bench_function("calc_logic/demorgan", |b| {
        let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
        b.iter(|| {
            let (res,itersize) = prove(black_box(&demorgan), black_box(&rules), 1, 10, &tru);
            size = itersize;
            assert!(tru.eq(&res))
        });
        warn!("calc_logic/demorgan {}", size);
    });

    let frege: RecExpr<SymbolLang> = "(=> (=> p (=> q r)) (=> (=> p q) (=> p r)))"
        .parse()
        .unwrap();
    c.bench_function("calc_logic/freges_theorem", |b| {
        let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
        b.iter(|| {
            let (res,itersize) = prove(black_box(&frege), black_box(&rules), 2, 10, &tru);
            size = itersize;
            assert!(tru.eq(&res))
        });
        warn!("calc_logic/freges_theorem {}", size);
    });
}

criterion_group!(benches, calc_logic_benchmark);
criterion_main!(benches);
