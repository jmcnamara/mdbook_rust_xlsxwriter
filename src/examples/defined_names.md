# Defined names: using user defined variable names in worksheets

Example of how to create defined names using the `rust_xlsxwriter` library.

This functionality is used to define user friendly variable names to represent a
value, a single cell,  or a range of cells in a workbook.

**Images of the output file:**

![Image 1 of output from app_defined_name.rs](../../images/app_defined_name1.png)

Here is the output in the Excel Name Manager. Note that there is a
Global/Workbook "Sales" variable name and a Local/Worksheet version.

![Image 2 of output from app_defined_name.rs](../../images/app_defined_name2.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_defined_name.rs:10:}}
```
