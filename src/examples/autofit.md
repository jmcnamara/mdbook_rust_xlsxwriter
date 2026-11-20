# Autofitting Columns: Example of autofitting column widths

This is an example of using the simulated autofit option to automatically set
worksheet column widths based on the data in the column.

To get better results for autofitting numbers and dates you can enable the
`enhanced_autofit` feature in your `Cargo.toml` file. This uses the `ssfmt`
crate to more accurately simulate Excel's number formatting.

**Image of the output file:**

![Image of output from autofit.rs](../../images/autofit.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofit.rs:12:}}
```