use egg::{*};


pub fn simplify<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    timeout: usize,
    eclasslimit: usize,
) -> RecExpr<L> {
    // run rules
    let runner = saturate(&expr, rules, timeout, eclasslimit);

    // extract shortest expression
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best) = extractor.find_best(runner.roots[0]);
    best
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


pub fn prove<L: Language>(
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
