# Extending generic `write()` to handle user data types

Example of how to extend the the `rust_xlsxwriter`[`worksheet.write()`] method using the
[`IntoExcelData`] trait to handle arbitrary user data that can be mapped to one
of the main Excel data types.

For this example we create a simple struct type to represent a [Unix Time]. This
is the number of elapsed seconds since the epoch of January 1970 (UTC). Note,
this is for demonstration purposes only. The [`ExcelDateTime`] struct in
 `rust_xlsxwriter` can handle Unix timestamps.


[Unix Time]: https://en.wikipedia.org/wiki/Unix_time
[`IntoExcelData`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/trait.IntoExcelData.html
[`ExcelDateTime`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ExcelDateTime.html
[`worksheet.write()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write

**Image of the output file:**

![Image of generic writer example](../../images/write_generic.png)


**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_write_generic_data.rs:9:}}
```