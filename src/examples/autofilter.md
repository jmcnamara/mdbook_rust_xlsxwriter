# Adding Autofilters

An example of how to create autofilters with the rust_xlsxwriter library..

An autofilter is a way of adding drop down lists to the headers of a 2D range of
worksheet data. This allows users to filter the data based on simple criteria so
that some data is shown and some is hidden.

Note, adding filter criteria isn't currently supported. That will be added in an
upcoming version.

**Image of the output file:**

![Image of output from app_autofilter.rs](../../images/app_autofilter1.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:15:34}}
```