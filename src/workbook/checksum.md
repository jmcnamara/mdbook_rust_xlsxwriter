# Checksum of a saved file


A common issue that occurs with `rust_xlsxwriter`, but also with Excel, is that
running the same program twice doesn't generate the same file, byte for byte.
This can cause issues with applications that do checksumming for testing
purposes.

For example consider the following simple `rust_xlsxwriter` program:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_properties_checksum1.rs:8:}}
```

If we run this several times, with a small delay, we will get different checksums as shown below:

```bash
$ cargo run --example doc_properties_checksum1

$ sum properties.xlsx
62457 6 properties.xlsx

$ sleep 2

$ cargo run --example doc_properties_checksum1

$ sum properties.xlsx
56692 6 properties.xlsx # Different to previous.
```

This is due to a file creation datetime that is included in the file and which
changes each time a new file is created.

The relevant section of the `docProps/core.xml` sub-file in the xlsx format
looks like this:


```xml
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<cp:coreProperties>
  <dc:creator/>
  <cp:lastModifiedBy/>
  <dcterms:created xsi:type="dcterms:W3CDTF">2023-01-08T00:23:58Z</dcterms:created>
  <dcterms:modified xsi:type="dcterms:W3CDTF">2023-01-08T00:23:58Z</dcterms:modified>
</cp:coreProperties>
```

If required this can be avoided by setting a constant creation date in the
document properties metadata:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_properties_checksum2.rs:8:}}
```

Then we will get the same checksum for the same output every time:

```bash
$ cargo run --example doc_properties_checksum2

$ sum properties.xlsx
8914 6 properties.xlsx

$ sleep 2

$ cargo run --example doc_properties_checksum2

$ sum properties.xlsx
8914 6 properties.xlsx # Same as previous
```

For more details see [`Properties`] and [`workbook::set_properties()`].

[`Properties`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Properties.html
[`workbook::set_properties()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Workbook.html#method.set_properties