use criterion::{criterion_main, criterion_group, Criterion};
use log::{LevelFilter};
use env_logger;

mod symbollang;
mod customlang;
mod egraph;


fn init_logger() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Warn) // use warning level to skip all infos logged by egg
        .init();
}

// Function to execute all benchmarks
fn init_benchmarks(_c: &mut Criterion) {
    init_logger(); // Initialize the logger
}

criterion_group!(benches, init_benchmarks);

criterion_main! {
    benches,
    symbollang::basic_maths::benches,
    symbollang::calc_logic::benches,
    symbollang::prop_logic::benches,
    customlang::basic_maths::benches,
    customlang::calc_logic::benches,
    customlang::prop_logic::benches,
    egraph::benches,
}
