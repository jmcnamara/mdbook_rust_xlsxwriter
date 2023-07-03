# Insert images: Inserting images to fit cell

An example of inserting images into a worksheet using `rust_xlsxwriter`so that
they are scaled to a cell. This approach can be useful if you are building up a
spreadsheet of products with a column of images for each product.

**Image of the output file:**

![Image of output from app_images_fit_to_cel.rs](../../images/app_images_fit_to_cell.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_images_fit_to_cell.rs:10:}}
```