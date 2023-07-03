# Merging cells: An example of merging cell ranges

This is an example of creating merged cells ranges in Excel using
[`worksheet.merge_range()`].

[`worksheet.merge_range()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.merge_range

The `merge_range()` method only handles strings but it can be used to merge
other data types, such as number, as shown below.

**Image of the output file:**

![Image of output from app_hello_world.rs](../../images/app_merge_range.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_merge_range.rs:8:}}
```


