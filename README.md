# [Metatheory.jl](https://github.com/JuliaSymbolics/Metatheory.jl) and [egg](https://github.com/egraphs-good/egg) comparison benchmarks

Read more in the [preprint](https://arxiv.org/abs/2404.08751). 

This repository contains some benchmarks for the [egg](https://github.com/egraphs-good/egg) equality saturation Rust library, that should correspond 1-1 with the benchmarks in Metatheory.jl. 
Metatheory.jl benchmarks are defined in the [Metatheory.jl repo](https://github.com/JuliaSymbolics/Metatheory.jl) in the benchmarks folder.

This repository contains also some automation scripts to automatically produce a Markdown table that contains 
the comparison between various versions of MT against egg. 

We report the median execution time of such benchmarks. The benchmarking time includes e-graph creation, 
one or multiple iterated executions of EqSat and extraction. Metatheory.jl's implementation of EqSat is 
closely modeled after egg and the benchmarks include equivalent systems of rewrites, EqSat parameters 
and halting conditions.  For the sake of simplicity, we are always using an e-graph extraction cost 
function that chooses the smallest terms in e-classes (`astsize`). Readers can refer to the 
original [egg paper](https://dl.acm.org/doi/10.1145/3434304) for an accurate description of the e-graph 
extraction process. The first column reports the name of the benchmark, while the second and third columns 
report the median execution time in egg. The _egg-sym_ column refers to benchmarks implemented using `SymbolLang`, 
a generic tree structure, while _egg-cust_ benchmarks were implemented with `define_language!`, generating 
optimized expression data structures. Columns _MT2_ and _MT3_ report the execution times in Metatheory.jl 
version 2.0 and the currently in-development version 3.0. Metatheory.jl benchmarks were implemented using 
the internal Julia expression type `Expr`. The remaining columns report a ratio, comparing how Metatheory.jl
3.0 performs against its previous release and egg. As Metatheory.jl can rewrite on any Julia data type that 
implements [TermInterface.jl](https://github.com/JuliaSymbolics/TermInterface.jl), there is a tradeoff 
between dynamism and speed, as substantial overhead is indeed introduced by repeated construction and 
garbage collection of such a generic data structure.


To run the benchmarks:

```bash
# run Metatheory benchmarks on the two branches that are specified in the Makefile
make mt-bench

# run egg benchmarks
make egg-bench

# create results table
make results-table
```


Preliminary results:

|                                 | egg-sym   | egg-cust | MT@ale/3.0 | MT@master  | egg-sym/MT@ale... | egg-cust/MT@al... | MT@master/MT@a... |
|:--------------------------------|:---------:|:--------:|:----------:|:----------:|:-----------------:|:-----------------:|:-----------------:|
| egraph_addexpr                  | 730 μs    |          | 4.75 ms    | 11.5 ms    | 0.154             |                   | 2.43              |
| basic_maths_simpl2              | 8.19 ms   | 3.49 ms  | 24.4 ms    | 795 ms     | 0.336             | 0.143             | 32.6              |
| prop_logic_freges_theorem       | 1.64 ms   | 1.08 ms  | 2.99 ms    | 31.1 ms    | 0.55              | 0.361             | 10.4              |
| calc_logic_demorgan             | 41.4 μs   | 24.8 μs  | 87.6 μs    | 327 μs     | 0.472             | 0.283             | 3.73              |
| calc_logic_freges_theorem       | 3.11 ms   | 2.13 ms  |            |            |                   |                   |                   |
| basic_maths_simpl1              | 4.06 ms   | 2.09 ms  | 5.97 ms    | 45.8 ms    | 0.679             | 0.351             | 7.67              |
| egraph_constructor              | 0.0561 μs |          | 0.417 μs   | 0.417 μs   | 0.135             |                   | 1                 |
| prop_logic_prove1               | 20.5 ms   | 18.3 ms  | 28.8 ms    | 7.3e+03 ms | 0.712             | 0.634             | 253               |
| prop_logic_demorgan             | 56.7 μs   | 29.6 μs  | 99.2 μs    | 579 μs     | 0.572             | 0.298             | 5.84              |
| while_superinterpreter_while_10 |           |          | 14.1 ms    | 75.8 ms    |                   |                   | 5.37              |
| prop_logic_rewrite              |           |          | 74.8 μs    | 80.8 μs    |                   |                   | 1.08              |
| time_to_load                    |           |          | 41.1 ms    | 65.8 ms    |                   |                   | 1.6               |

