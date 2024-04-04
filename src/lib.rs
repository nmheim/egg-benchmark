use egg::{*};


pub fn prove<T: Language + FromOp>(s: &str, rules: &Vec<Rewrite<T, ()>>) -> bool {
    // parse input string to logic expression
    let expr: RecExpr<T> = s.parse().unwrap();

    // define scheduler/runner with saturation parameters
    let scheduler = BackoffScheduler::default()
        .with_initial_match_limit(6000)
        .with_ban_length(5);
    let runner = Runner::default()
        .with_iter_limit(10)
        .with_node_limit(5_000)
        .with_expr(&expr)
        .with_scheduler(scheduler)
        .run(rules);

    // check if expr and `true` are in the same eclass
    let t: RecExpr<T> = "true".parse().unwrap();
    let r = runner.egraph.equivs(&expr, &t);
    r.len() > 0
}
