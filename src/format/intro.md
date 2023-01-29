# The Format struct


The [`Format`] struct is used to define cell formatting for data in a worksheet.

The properties of a cell that can be formatted include: fonts, colors, patterns,
borders, alignment and number formatting.

![Image of output from doc_format_intro.rs](../../images/format_intro.png)

The output file above was created with the following code:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_format_intro.rs:8:}}
```

[`Format`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html