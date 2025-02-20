use egg::*;

pub struct EGraphSize {
    pub num_classes: usize,
    pub num_memo: usize,
    pub num_nodes: usize
}

use std::fmt;

impl fmt::Display for EGraphSize {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "n_classes: {}, n_nodes: {}, n_memo: {}", self.num_classes, self.num_nodes, self.num_memo)
    }
}


pub fn simplify<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    timeout: usize,
) -> (RecExpr<L>, EGraphSize) {
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
    let size = EGraphSize { 
        num_classes: runner.egraph.classes().count(), 
        num_memo: runner.egraph.total_size(), 
        num_nodes: runner.egraph.total_number_of_nodes() };
    (best, size)
}

pub fn prove<L: Language>(
    expr: &RecExpr<L>,
    rules: &Vec<Rewrite<L, ()>>,
    steps: usize,
    timeout: usize,
    tru: &RecExpr<L>,
) -> (RecExpr<L>, EGraphSize) {
    let mut n_classes = 0;
    let mut n_memo = 0;
    let mut n_nodes = 0;
    // We start with the provided expr and in the following iterations start with the expr returned by the previous iteration.
    // As soon as expr == true all subsequent calls of .run() will return immediately because of the hook.
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
        n_classes += runner.egraph.classes().count();
        n_memo += runner.egraph.total_size();
        n_nodes += runner.egraph.total_number_of_nodes();
        best
    });
    let size = EGraphSize { num_classes: n_classes, num_memo: n_memo, num_nodes: n_nodes };

    (out, size)
}
