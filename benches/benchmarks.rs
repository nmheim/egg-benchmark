use criterion::criterion_main;

mod symbollang;
mod customlang;
mod egraph;

criterion_main! {
    symbollang::basic_maths::benches,
    symbollang::calc_logic::benches,
    symbollang::prop_logic::benches,
    customlang::basic_maths::benches,
    customlang::calc_logic::benches,
    customlang::prop_logic::benches,
    egraph::benches,
}
