use egg::{*, rewrite as rw};
use egg_benchmark::{*};
use std::time::Instant;

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


pub fn propositional_logic_rules() -> Vec<Rewrite<PropositionalLogic, ()>> {
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


pub fn main() {
    let rules = propositional_logic_rules();
    let tru: RecExpr<PropositionalLogic> = "true".parse().unwrap();

    // ===========================================

    let apply_time: std::time::Instant = Instant::now();
    
    // demorgan
    let ex_demorgan: RecExpr<PropositionalLogic> = "(== (!! (|| p q)) (&& (!! p) (!! q)))"
        .parse().unwrap();
    println!("demorgan: {}", prove(&ex_demorgan, &rules, 1, 10, &tru));

    println!("simplification time {}", apply_time.elapsed().as_secs_f64());

    // ===========================================

    let apply_time: std::time::Instant = Instant::now();

    // frege
    let ex_frege: RecExpr<PropositionalLogic> = "(=> (=> p (=> q r)) (=> (=> p q) (=> p r)))"
        .parse().unwrap();
    println!("frege:    {}", prove(&ex_frege, &rules, 1, 10, &tru));

    println!("simplification time {}", apply_time.elapsed().as_secs_f64());


    // ===========================================

    let apply_time: std::time::Instant = Instant::now();

    // let ex_logic = "(=> (&& (&& (=> p q) (=> r s)) (|| p r)) (|| q s))";
    let s = "(|| (!! (&& (|| (!! p) q) (&& (|| (!! r) s) (|| p r)))) (|| q s))";
    // let ex_logic = "(|| (|| q s) (!! (&& q (&& p (!! r)))))";
    // let ex_logic = "(== p p)";
    
    let ex_logic: RecExpr<PropositionalLogic> = s.parse().unwrap();
    let expr = prove(&ex_logic, &rules, 2, 6, &tru);
    println!("logic:    {}", tru.eq(&expr));

    println!("simplification time {}", apply_time.elapsed().as_secs_f64());
}

