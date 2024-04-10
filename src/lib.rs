use egg::*;

pub fn simplify<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    timeout: usize,
) -> RecExpr<L> {
    // run rules
    let scheduler = BackoffScheduler::default();
    let runner = Runner::default()
        .with_iter_limit(timeout)
        .with_node_limit(15000)
        .with_expr(&expr)
        .with_scheduler(scheduler)
        .run(rules);

    // extract shortest expression
    let extractor = Extractor::new(&runner.egraph, AstSize);
    let (_, best) = extractor.find_best(runner.roots[0]);
    best
}

pub fn prove<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    steps: usize,
    timeout: usize,
    tru: &RecExpr<L>,
) -> RecExpr<L> {
    let out: RecExpr<L> = (0..steps).fold(expr.clone(), |expr, _| {
        let scheduler = BackoffScheduler::default()
            .with_initial_match_limit(6000)
            .with_ban_length(5);
        let runner = Runner::default()
            .with_iter_limit(timeout)
            .with_node_limit(15000)
            .with_expr(&expr)
            .with_expr(&tru)
            .with_scheduler(scheduler)
            .with_hook(|runner| {
                let istru =
                    runner.egraph.find(runner.roots[0]) == runner.egraph.find(runner.roots[1]);
                // println!("Is true??? {}", istru);
                if istru {
                    Err("PROVED".to_string())
                } else {
                    Ok(())
                }
            })
            .run(rules);
        let root = runner.roots[0];
        let extractor = Extractor::new(&runner.egraph, AstSize);
        let (_, best) = extractor.find_best(root);
        best
    });
    out
}
