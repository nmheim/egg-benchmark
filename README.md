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

