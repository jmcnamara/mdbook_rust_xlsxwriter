# Format defaults

The default Excel cell format is a font setting of Calibri size 11 with all
other properties turned off.

It is occasionally useful to use a default format with a method that requires a
format but where you don't actually want to change the formatting.

```rust
{{#include ../../../rust_xlsxwriter/examples/doc_format_default.rs:7:}}
```

Output showing that both strings have the same format:

![Image of output from doc_format_default.rs](../../images/format_default.png)
