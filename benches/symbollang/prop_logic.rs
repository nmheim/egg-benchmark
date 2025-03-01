use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::{*, rewrite as rw};
use egg_benchmark::{*};
use log::{warn};

define_language! {
    pub enum PropositionalLogic {
        Num(i32),
        "==" = Eq([Id; 2]),
        "||" = Or([Id; 2]),
        "!!" = Not([Id; 1]),
        "&&" = And([Id; 2]),
        "=>" = Impl([Id; 2]),
        Symbol(Symbol),
    }
}


pub fn propositional_logic_rules() -> Vec<Rewrite<SymbolLang, ()>> {
    vec![
        // or algebra
        // ((p || q) || r) == (p || (q || r))
        rw!("or-1"; "(|| (|| ?p ?q) ?r)" <=> "(|| ?p (|| ?q ?r))"),
        // (p || q) == (q || p)
        rw!("or-2"; "(|| ?p ?q)" <=> "(|| ?q ?p)"),
        // (p || p) --> p
        vec![rw!("or-3"; "(|| ?p ?p)" => "?p")],
        // (p || true) --> true
        vec![rw!("or-4"; "(|| ?p true)" => "true")],
        // (p || false) --> p
        vec![rw!("or-5"; "(|| ?p false)" => "?p")],

        // and algebra
        // ((p && q) && r) == (p && (q && r))
        rw!("and-1"; "(&& (&& ?p ?q) ?r)" <=> "(&& ?p (&& ?q ?r))"),
        // (p && q) == (q && p)
        rw!("and-2"; "(&& ?p ?q)" <=> "(&& ?q ?p)"),
        // (p && p) --> p
        vec![rw!("and-3"; "(&& ?p ?p)" => "?p")],
        // (p && true) --> p
        vec![rw!("and-4"; "(&& ?p true)" => "?p")],
        // (p && false) --> false
        vec![rw!("and-5"; "(&& ?p false)" => "false")],

        // negation algebra
        // (p && !p) --> false
        vec![rw!("neg-1"; "(&& ?p (!! ?p))" => "false")],
        // (p || !(p)) --> true
        vec![rw!("neg-2"; "(|| ?p (!! ?p))" => "true")],
        // !(!p) == p
        rw!("neg-3"; "(!! (!! ?p))" <=> "?p"),

        // comb
        // !(p || q) == (!p && !q)                   # DeMorgan
        rw!("comb-1"; "(!! (|| ?p ?q))" <=> "(&& (!! ?p) (!! ?q))"),
        // !(p && q) == (!p || !q)
        rw!("comb-2"; "(!! (&& ?p ?q))" <=> "(|| (!! ?p) (!! ?q))"),
        // (p && (q || r)) == ((p && q) || (p && r)) # Distributivity
        rw!("comb-3"; "(&& ?p (|| ?q ?r))" <=> "(|| (&& ?p ?q) (&& ?p ?r))"),
        // (p || (q && r)) == ((p || q) && (p || r))
        rw!("comb-4"; "(|| ?p (&& ?q ?r))" <=> "(&& (|| ?p ?q) (|| ?p ?r))"),
        // (p && (p || q)) --> p                     # Absorb
        vec![rw!("comb-5"; "(&& ?p (|| ?p ?q))" => "?p")],
        // (p || (p && q)) --> p
        vec![rw!("comb-6"; "(|| ?p (&& ?p ?q))" => "?p")],
        // (p && (!p || q)) --> p && q               # Complement
        vec![rw!("comb-7"; "(&& ?p (|| (!! ?p) ?q))" => "(&& ?p ?q)")],
        // (p || (!p && q)) --> p || q
        vec![rw!("comb-8"; "(|| ?p (&& (!! ?p) ?q))" => "(|| ?p ?q)")],

        // impl
        //   (p == !p) --> false
        vec![rw!("impl-1"; "(== ?p (!! ?p))" => "false")],
        //   (p == p) --> true
        vec![rw!("impl-2"; "(== ?p ?p)" => "true")],
        //   (p == q) --> (!p || q) && (!q || p)
        vec![rw!("impl-3"; "(== ?p ?q)" => "(&& (|| (!! ?p) ?q) (|| (!! ?q) ?p))")],
        //   (p ⟹  q) --> (!p || q)
        vec![rw!("impl-4"; "(=> ?p ?q)" => "(|| (!! ?p) ?q)")],
    ].concat()
}


pub fn propositional_logic_benchmark(c: &mut Criterion) {
    let rules = propositional_logic_rules();
    let tru: RecExpr<SymbolLang> = "true".parse().unwrap();

    // let ex_orig = "(=> (&& (&& (=> p q) (=> r s)) (|| p r)) (|| q s)))";
    let ex_logic: RecExpr<SymbolLang>
        = "(|| (!! (&& (|| (!! p) q) (&& (|| (!! r) s) (|| p r)))) (|| q s))"
        .parse().unwrap();
    c.bench_function( "prop_logic/prove1",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let (result,itersize) = prove(black_box(&ex_logic), black_box(&rules), 2, 6, &tru);
                assert_eq!(result, tru);
                size = itersize;
            });
            warn!("prop_logic/prove1 {}", size);
        }
    );

    let demorgan: RecExpr<SymbolLang>
        = "(== (!! (|| p q)) (&& (!! p) (!! q)))"
        .parse().unwrap();
    c.bench_function( "prop_logic/demorgan",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let (result,itersize) = prove(black_box(&demorgan), black_box(&rules), 1, 10, &tru);
                size = itersize;
                assert_eq!(result, tru)
            });
            warn!("prop_logic/demorgan {}", size);
        }
    );

    let frege: RecExpr<SymbolLang>
        = "(=> (=> p (=> q r)) (=> (=> p q) (=> p r)))"
        .parse().unwrap();
    c.bench_function(
        "prop_logic/freges_theorem",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let (result,itersize) = prove(black_box(&frege), black_box(&rules), 1, 10, &tru);
                size = itersize;
                assert_eq!(result, tru)
            });
            warn!("prop_logic/freges_theorem {}", size);
        }
    );
}

criterion_group!(benches, propositional_logic_benchmark);
criterion_main!(benches);

