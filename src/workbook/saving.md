# Creating and saving an xlsx file

Creating a  [`Workbook`] struct instance to represent an Excel xlsx file is done
via the [`Workbook::new()`] method:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_workbook_new.rs:11}}
```

Once you are finished writing data via a worksheet you can save it with the [`workbook.save()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_workbook_new.rs:8:}}
```

This will you a simple output file like the following.

![Image of output from doc_workbook_new.rs](../../images/workbook_new.png)

The  `save()` method takes a [`std::path`] or path/filename string. You can also
save the xlsx file data to a `Vec<u8>` buffer via the
[`workbook.save_to_buffer()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_workbook_save_to_buffer.rs:16}}
```

This can be useful if you intend to stream the data.

[`Workbook`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Workbook.html
[`std::path`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`workbook::new()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Workbook.html#method.new
[`workbook.save()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Workbook.html#method.save
[`workbook.save_to_buffer()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Workbook.html#method.save_to_buffer
