# Insert images: Embedding an image in a cell

An example of embedding images into a worksheet cells using `rust_xlsxwriter`.
This image scales to size of the cell and moves with it.

This approach can be useful if you are building up a spreadsheet of products
with a column of images for each product.

This is the equivalent of Excel's menu option to insert an image using the
option to "Place in Cell" which is only available in Excel 365 versions from
2023 onwards. For older versions of Excel a `#VALUE!` error is displayed.


**Image of the output file:**

![Image of output from app_embedded_images.rs](../../images/embedded_images.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_embedded_images.rs:15:}}
```