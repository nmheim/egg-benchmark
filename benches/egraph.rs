// rand_letter() = Symbol(rand('a':'z'))
// 
// function nested_expr(level)
//   if level > 0
//     :(($(rand_letter()))($(rand_letter())) + $(rand_letter()) + $(rand(1:100)) * $(nested_expr(level - 1)))
//   else
//     rand_letter()
//   end
// end
// 
// SUITE["egraph"]["addexpr"] = @benchmarkable EGraph($(nested_expr(2000)))

extern crate rand;
use rand::Rng;
use std::char;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use egg::{*};
use egg_benchmark::*;
use log::{warn};

define_language! {
    pub enum BasicMath {
        Num(i32),
        "+" = Add([Id; 2]),
        "*" = Mul([Id; 2]),
        "call" = Call([Id; 2]),
        Symbol(Symbol),
    }
}

// Generates a random lowercase letter
fn rand_letter() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range(b'a'..=b'z') as char
}

// Recursively creates a nested expression based on a level
fn nested_expr(level: u32) -> String {
    if level > 0 {
        format!("(+ (call {} {}) (+ {} (* {} {})))",
            rand_letter(),
            rand_letter(),
            rand_letter(),
            rand::thread_rng().gen_range(1..=100),
            nested_expr(level - 1)
        )
    } else {
        rand_letter().to_string()
    }
}

pub fn egraph_benchmark(c: &mut Criterion) {

    c.bench_function("egraph/constructor",
        |b| b.iter(|| {
            let r: Runner<BasicMath,()> = Runner::default();
            r
        })
    );

    let expr: RecExpr<BasicMath> = nested_expr(2000).parse().unwrap();
    c.bench_function( "egraph/addexpr",
        |b| {
            let mut size = EGraphSize{num_classes:0, num_nodes:0, num_memo:0};
            b.iter(|| {
                let runner: Runner<BasicMath,()> = Runner::default().with_expr(black_box(&expr));
                size.num_classes = runner.egraph.classes().count();
                size.num_memo = runner.egraph.total_size();
                size.num_nodes = runner.egraph.total_number_of_nodes();
                runner
            });
            warn!("egraph/addexpr {}", size);
        }
    );
}

criterion_group!(benches, egraph_benchmark);
criterion_main!(benches);

