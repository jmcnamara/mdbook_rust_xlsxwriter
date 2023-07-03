# Adding a watermark: Adding a watermark to a worksheet by adding an image to the header


An example of adding a worksheet watermark image. This is based on the method of
putting an image in the worksheet header as suggested in the [Microsoft
documentation].

[Microsoft documentation]: https://support.microsoft.com/en-us/office/add-a-watermark-in-excel-a372182a-d733-484e-825c-18ddf3edf009

**Image of the output file:**

![Image of output from app_images.rs](../../images/app_watermark.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_watermark.rs:9:}}
```
