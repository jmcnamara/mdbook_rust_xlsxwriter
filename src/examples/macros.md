# Macros: Adding macros to a workbook

An example of adding macros to an `rust_xlsxwriter` file using a VBA macros
file extracted from an existing Excel xlsm file.

The [`vba_extract`](https://crates.io/crates/vba_extract) utility can be used to
extract the `vbaProject.bin` file.

**Image of the output file:**

![Image of output from app_macros.rs](../../images/app_macros.png)

```rust
{{#include ../../../rust_xlsxwriter/examples/app_macros.rs:11:}}
```

