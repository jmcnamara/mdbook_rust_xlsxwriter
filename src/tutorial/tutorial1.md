# Adding data to a worksheet

To add some sample expense data to a worksheet we could start with a simple
program like the following:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:8:}}
```

If we run this program we should get a spreadsheet that looks like this:

![Image of tutorial 1](../images/tutorial1.png)

This is a simple program but it demonstrates some of the steps that would
apply to any rust_xlsxwriter program.

The first step is to create a new workbook object using the
[`Workbook`] constructor [`Workbook::new()`]:

[`Workbook`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html
[`workbook::new()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.new


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:15}}
```

**Note**, `rust_xlsxwriter` can only create new files. It cannot read or modify
existing files.

The workbook object is then used to add a new worksheet via the
[`add_worksheet()`] method:

[`add_worksheet()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.add_worksheet



```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:18}}
```
The worksheet will have a standard Excel name, in this case "Sheet1". You can
specify the worksheet name using the [`Worksheet::set_name()`] method.

[`Worksheet::set_name()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_name

We then iterate over the data and use the
[`Worksheet::write()`](crate::Worksheet::write) method which converts common Rust
types to the equivalent Excel types and writes them to the specified `row, col`
location in the worksheet:

[`Worksheet::write()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write



```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:22:26}}
```


> **Reading ahead**:
>
> There are other type specific write methods such as
> [`Worksheet::write_string()`] and [`Worksheet::write_number()`]. However, these
> aren't generally required and thanks to Rust's monomorphization the
> performance of the generic `write()` method is just as fast.
>
> There are also worksheet methods for writing arrays of data or arrays of
> arrays of data that can be useful in cases where you don't need to add
> specific formatting:
>
> - [`Worksheet::write_row()`]
> - [`Worksheet::write_column()`]
> - [`Worksheet::write_row_matrix()`]
> - [`Worksheet::write_column_matrix()`]



[`Worksheet::write_string()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_string
[`Worksheet::write_number()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_number

[`Worksheet::write_row()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_row
[`Worksheet::write_column()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_column
[`Worksheet::write_row_matrix()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_row_matrix
[`Worksheet::write_column_matrix()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_column_matrix



Throughout rust_xlsxwriter rows and columns are zero indexed. So, for example,
the first cell in a worksheet, `A1`, is `(0, 0)`.

To calculate the total of the items in the second column we add a
[`Formula`] like this:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:30}}
```

[`Formula`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Formula.html


Finally, we save and close the Excel file via the [`Workbook::save()`] method
which will generate the spreadsheet shown in the image above.:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial1.rs:33}}
```

> **Reading ahead**:
>
> The [`Workbook::save()`] method takes a [`std::path`] argument which can be a
> `Path`, `PathBuf` or a filename string. It is also possible to save to a byte
> vector using [`Workbook::save_to_buffer()`].


[`std::path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`Workbook::save()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.save
[`Workbook::save_to_buffer()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.save_to_buffer
