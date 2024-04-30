# egg-benchmark

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

|                                 | egg-sym   | egg-cust | MT@ale/3.0 | MT@master   | egg-sym/MT@ale... | egg-cust/MT@al... | MT@master/MT@a... |
|:--------------------------------|:---------:|:--------:|:----------:|:-----------:|:-----------------:|:-----------------:|:-----------------:|
| egraph_addexpr                  | 730 μs    |          | 4.65 ms    | 10.9 ms     | 0.157             |                   | 2.35              |
| basic_maths_simpl2              | 8.19 ms   | 3.49 ms  | 23.8 ms    | 756 ms      | 0.344             | 0.147             | 31.7              |
| prop_logic_freges_theorem       | 1.64 ms   | 1.08 ms  | 2.95 ms    | 31 ms       | 0.558             | 0.366             | 10.5              |
| calc_logic_demorgan             | 41.4 μs   | 24.8 μs  | 87 μs      | 326 μs      | 0.475             | 0.285             | 3.74              |
| calc_logic_freges_theorem       | 3.11 ms   | 2.13 ms  |            |             |                   |                   |                   |
| basic_maths_simpl1              | 4.06 ms   | 2.09 ms  | 6 ms       | 44.8 ms     | 0.676             | 0.349             | 7.46              |
| egraph_constructor              | 0.0561 μs |          | 0.459 μs   | 0.417 μs    | 0.122             |                   | 0.908             |
| prop_logic_prove1               | 20.5 ms   | 18.3 ms  | 28.3 ms    | 7.16e+03 ms | 0.726             | 0.647             | 253               |
| prop_logic_demorgan             | 56.7 μs   | 29.6 μs  | 99.8 μs    | 579 μs      | 0.569             | 0.296             | 5.8               |
| while_superinterpreter_while_10 |           |          | 14.1 ms    | 73.1 ms     |                   |                   | 5.19              |
| prop_logic_rewrite              |           |          | 76 μs      | 81 μs       |                   |                   | 1.07              |
| time_to_load                    |           |          | 42.2 ms    | 62.2 ms     |                   |                   | 1.47              |
