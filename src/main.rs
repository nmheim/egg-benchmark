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
    let runner = Runner::default().with_expr(&expr).run(&calc_logic_rules());
    let t: RecExpr<CalcLogic> = "true".parse().unwrap();
    let r = runner.egraph.equivs(&expr, &t);
    r.len() > 0
}


pub fn main() {
    //let s = "(== (!! (|| p q)) (&& (!! p) (!! q)))";
    let s = "(=> (=> p (=> p r)) (=> (=> q p) (=> p r)))";
    println!("{}", prove(&s))
}
