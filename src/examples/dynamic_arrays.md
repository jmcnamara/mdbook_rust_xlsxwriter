# Dynamic array formulas: Examples of dynamic arrays and formulas

An example of how to use the `rust_xlsxwriter`library to write formulas and
functions that create dynamic arrays. These functions are new to Excel
365. The examples mirror the examples in the Excel documentation for these
functions.

**Images of the output file:**


![Image 1 of output from app_dynamic_arrays.rs](../../images/dynamic_arrays01.png)

Here is another example:

![Image 2 of output from app_dynamic_arrays.rs](../../images/dynamic_arrays02.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_dynamic_arrays.rs:9:}}
```
