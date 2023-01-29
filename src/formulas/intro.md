# Working with Formulas

In general any Excel formula can be used directly in `rust_xlsxwriter`:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_working_with_formulas_intro.rs:13:16}}
```

The formula will then be displayed as expected in Excel:

![Image of output from doc_working_with_formulas_intro.rs](../../images/working_with_formulas1.png)

However, there are a few potential issues and differences that the user of
`rust_xlsxwriter` should be aware of. These are explained in the following
sections.


[`worksheet.write_formula_only()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_formula_only