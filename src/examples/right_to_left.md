# Right to left display: Set a worksheet into right to left display mode

This is an example of using `rust_xlsxwriter`to create a workbook with the
default worksheet and cell text direction changed from left-to-right to
right-to-left, as required by some middle eastern versions of Excel.

**Image of the output file:**

![Image of output from app_right_to_left.rs](../../images/worksheet_set_right_to_left.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_right_to_left.rs:9:}}
```