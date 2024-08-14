# Number Formats in different Locales

As shown in the previous section the `Format::set_num_format()` method is used to
set the number format for `rust_xlsxwriter` formats. A common use case is to set
a number format with a "grouping/thousands" separator and a "decimal" point:

[`Format::set_num_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_num_format


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_format_locale.rs:8:}}
```

In the US locale (and some others) where the number "grouping/thousands"
separator is `","` and the "decimal" point is `"."` which would be shown in
Excel as:

![Image of output from doc_format_locale.rs](../../images/format_currency5.png)

In other locales these values may be reversed or different. They are generally
set in the "Region" settings of Windows or Mac OS.  Excel handles this by
storing the number format in the file format in the US locale, in this case
`"#,##0.00"`, but renders it according to the regional settings of the host OS.
For example, here is the same, unmodified, output file shown above in a German
locale:

![Image of output from doc_format_locale.rs](../../images/format_currency6.png)

And here is the same file in a Russian locale. Note the use of a space as the
"grouping/thousands" separator:

![Image of output from doc_format_locale.rs](../../images/format_currency7.png)


In order to replicate Excel's behavior all XlsxWriter programs should use US
locale formatting which will then be rendered in the settings of your host OS.
