# Dynamic array formulas

An example of how to use the rust_xlsxwriter library to write formulas and
functions that create dynamic arrays. These functions are new to Excel
365. The examples mirror the examples in the Excel documentation for these
functions.

Sample output:

![Image 1 of output from app_dynamic_arrays.rs](../../images/dynamic_arrays01.png)

And:

![Image 2 of output from app_dynamic_arrays.rs](../../images/dynamic_arrays02.png)

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_dynamic_arrays.rs:8:}}
```
