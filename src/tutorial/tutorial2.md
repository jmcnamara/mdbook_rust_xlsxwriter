# Adding some formatting


The previous program converted the required data into an Excel file but it
looked a little bare. In order to make the information clearer we can add some
simple formatting, like this:

![Image of first tutorial 2](../images/tutorial2.png)


The differences here are that we have added "Item" and "Cost" column headers in
a bold font, we have formatted the currency in the second column and we have
made the "Total" string bold.

To do this programmatically we can extend our code as follows:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_tutorial2.rs:7:}}
```

The main difference between this and the previous program is that we have added
two [Format] objects that we can use to format cells in the spreadsheet.

Format objects represent all of the formatting properties that can be applied to
a cell in Excel such as fonts, number formatting, colors and borders. This is
explained in more detail in the [Format] struct documentation.

[Format]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html


For now we will avoid getting into the details of Format and just use a limited
amount of the its functionality to add some simple formatting:

```rust,ignore
{{#include ../../../rust_xlsxwriter/examples/app_tutorial2.rs:16:20}}
```

We can use these formats in worksheet methods that support formatting such as
the [`worksheet.write_string()`] and [`worksheet.write_number()`] methods which
can write data and formatting together.

[`worksheet.write_string()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_string
[`worksheet.write_number()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_number
