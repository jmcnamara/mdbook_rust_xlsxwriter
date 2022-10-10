# Format colors

Format property colors are specified by using the [`XlsxColor`] enum with a Html
style RGB integer value or a limited number of defined colors, for example:

[`XlsxColor`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/enum.XlsxColor.html

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_enum_xlsxcolor.rs:7:}}
```

Output:

![Image of output from doc_enum_xlsxcolor.rs](../../images/enum_xlsxcolor.png)

See also the longer example in [Format colors example](../examples/colors.md).