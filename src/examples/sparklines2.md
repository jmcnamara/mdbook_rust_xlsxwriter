# Sparklines: advanced example

Example of adding sparklines to an Excel spreadsheet using the
`rust_xlsxwriter` library.

Sparklines are small charts that fit in a single cell and are used to show
trends in data. This example shows the majority of options that can be applied
to sparklines.


**Image of the output file:**

![Image of sparkline example](../../images/sparklines2.png)


**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_sparklines2.rs:12:}}
```
