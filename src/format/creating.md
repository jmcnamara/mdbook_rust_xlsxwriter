# Creating and using a Format object

Formats are created by calling the [`Format::new()`] method and properties as
set using the various methods shown is this section of the document. Once
the Format has been created it can be passed to one of the worksheet
`write_*()` methods. Multiple properties can be set by chaining them
together, for example:


[`Format::new()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.new

```rust
{{#include ../../../rust_xlsxwriter/examples/doc_format_create.rs:7:}}
```

Output:

![Image of output from doc_format_create.rs](../../images/format_create.png)

Formats can be cloned in the usual way:

```rust
{{#include ../../../rust_xlsxwriter/examples/doc_format_clone.rs:7:}}
```

Output:

![Image of output from doc_format_clone.rs](../../images/format_clone.png)


