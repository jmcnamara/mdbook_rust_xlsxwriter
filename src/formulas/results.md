# Formula Results

The `rust_xlsxwriter` library doesn't calculate the result of a formula and
instead stores the value "0" as the formula result. It then sets a global flag
in the XLSX file to say that all formulas and functions should be recalculated
when the file is opened.

This works fine with the majority of spreadsheet applications. However,
applications that don't have a facility to calculate formulas will only display
the 0 results. Examples of such applications are Excel viewers, PDF converters,
and some mobile device applications.

If required, it is also possible to specify the calculated result of the
formula using the [`worksheet.set_formula_result()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_formula_result.rs:18:20}}
```

One common spreadsheet application where the formula recalculation doesn't work
is LibreOffice (see the following [issue report]). If you wish to force
recalculation in LibreOffice you can use the
[`worksheet.set_formula_result_default()`] method to set the default result to
an empty string:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_formula_result_default.rs:17}}
```

[`worksheet.set_formula_result()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_formula_result
[`worksheet.set_formula_result_default()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_formula_result_default
[issue report]: https://bugs.documentfoundation.org/show_bug.cgi?id=144819