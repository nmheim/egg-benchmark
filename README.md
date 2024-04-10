# egg-benchmark

To run the benchmarks:

```bash
cargo bench
```


Preliminary results:


|                                 | egg                 | Metatheory        | egg/Metatheory |
|:--------------------------------|:-------------------:|:-----------------:|:--------------:|
| basic_maths_foo                 | 0.0557 ± 0.002 s    |                   |                |
| basic_maths_simpl1              | 2.4 ± 0.017 ms      | 17.3 ± 1.8 ms     | 0.139          |
| basic_maths_simpl2              | 4.53 ± 0.031 ms     | 0.0374 ± 0.0021 s | 0.121          |
| calc_logic_demorgan             | 30.8 ± 0.95 μs      | 0.173 ± 0.0061 ms | 0.178          |
| calc_logic_freges_theorem       | 2.69 ± 0.013 ms     | broken            |                |
| egraph_addexpr                  | 1.26 ± 0.047 ms     | 6.37 ± 0.96 ms    | 0.197          |
| egraph_constructor              | 0.0694 ± 0.0002 μs  | 0.768 ± 0.43 μs   | 0.0904         |
| prop_logic_demorgan             | 0.0397 ± 0.00058 ms | 0.199 ± 0.024 ms  | 0.199          |
| prop_logic_freges_theorem       | 1.51 ± 0.011 ms     | 3.57 ± 0.19 ms    | 0.422          |
| prop_logic_prove1               | 12.5 ± 0.086 ms     | 0.0973 ± 0.009 s  | 0.128          |
| simpl1                          | 0.0338 ± 0.00027 s  |                   |                |
| while_superinterpreter_while_10 |                     | 0.0425 ± 0.0037 s |                |
| prop_logic_rewrite              |                     | 0.0932 ± 0.011 ms |                |
| time_to_load                    |                     | 0.0634 ± 0.0029 s |                |
