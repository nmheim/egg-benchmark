use egg::{*};


pub fn prove<L: Language + FromOp>(
    s: &str,
    rules: &Vec<Rewrite<L, ()>>,
    timeout: usize,
    eclasslimit: usize,
) -> bool {
    // parse input string to logic expression
    let expr: RecExpr<L> = s.parse().unwrap();

    // run rules
    let runner = saturate(&expr, rules, timeout, eclasslimit);

    // check if expr and `true` are in the same eclass
    let t: RecExpr<L> = "true".parse().unwrap();
    let r = runner.egraph.equivs(&expr, &t);
    r.len() > 0
}


pub fn saturate<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    timeout: usize,
    eclasslimit: usize,
) -> Runner<L,()> {
    let scheduler = BackoffScheduler::default()
        .with_initial_match_limit(6000)
        .with_ban_length(5);
    let runner = Runner::default()
        .with_iter_limit(timeout)
        .with_node_limit(eclasslimit)
        .with_expr(&expr)
        .with_scheduler(scheduler)
        .run(rules);
    runner
}


pub fn simplify<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    steps: usize,
    timeout: usize,
    eclasslimit: usize,
) -> RecExpr<L> {

    let out: RecExpr<L> = (0..steps).fold(
        expr.clone(),
        |expr, _| {
            let runner = saturate(&expr, &rules, timeout, eclasslimit);
            let root = runner.roots[0];
            let extractor = Extractor::new(&runner.egraph, AstSize);
            let (_, best) = extractor.find_best(root);
            best
        }
    );
    out
}
