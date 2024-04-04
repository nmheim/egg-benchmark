use egg::{*};


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


pub fn main() {
    //let s = "(== (!! (|| p q)) (&& (!! p) (!! q)))";
    let s = "(=> (=> p (=> p r)) (=> (=> q p) (=> p r)))";
    let apply_time: std::time::Instant = std::time::Instant::now();
    // assert_eq!(simplify("(+ 0 (* 1 foo))"), "foo");
    assert!(prove(&s));
    let apply_time = apply_time.elapsed().as_secs_f64();
    println!("{}", apply_time)
}
