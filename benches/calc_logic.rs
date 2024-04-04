use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::*;

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



pub fn calc_logic_rules() -> Vec<Rewrite<CalcLogic, ()>> {
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
    ].concat()
}

pub fn prove(s: &str) -> bool {
    let expr: RecExpr<CalcLogic> = s.parse().unwrap();
    let scheduler = BackoffScheduler::default()
        .with_initial_match_limit(6000)
        .with_ban_length(5);
    let runner = Runner::default()
        .with_iter_limit(10)
        .with_node_limit(5_000)
        .with_expr(&expr)
        .with_scheduler(scheduler)
        .run(&calc_logic_rules());
    let t: RecExpr<CalcLogic> = "true".parse().unwrap();
    let r = runner.egraph.equivs(&expr, &t);
    r.len() > 0
}


// fold = @theory p q begin
//   (p::Bool == q::Bool) => (p == q)
//   (p::Bool || q::Bool) => (p || q)
//   (p::Bool ⟹ q::Bool)  => ((p || q) == q)
//   (p::Bool && q::Bool) => (p && q)
//   !(p::Bool)           => (!p)
// end
// 
// calc = @theory p q r begin
// end
// 
// calculational_logic_theory = calc ∪ fold


pub fn calc_logic_benchmark(c: &mut Criterion) {
    let demorgan = "(== (!! (|| p q)) (&& (!! p) (!! q)))";
    assert!(prove(&demorgan));
    c.bench_function( "demorgan", |b| b.iter(|| prove(black_box(&demorgan))));

    let frege = "(=> (=> p (=> p r)) (=> (=> q p) (=> p r)))";
    assert!(prove(&frege));
    c.bench_function( "frege", |b| b.iter(|| prove(black_box(&frege))));
}

criterion_group!(benches, calc_logic_benchmark);
criterion_main!(benches);

