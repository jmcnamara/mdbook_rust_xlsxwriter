# Adding dates and more formatting

Let's extend the program a little bit more to add some dates to the data:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:13:18}}
```

The corresponding spreadsheet will look like this:

![Image of first tutorial 3](../images/tutorial3.png)

The differences here are that we have added a "Date" column with formatting and
made that column a little wider to accommodate the dates.

To do this we can extend our program as follows:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:8:}}
```

Dates and times in Excel are floating point numbers that have a number format
applied to display them in the correct format. In order to handle dates and
times with `rust_xlsxwriter` we create them using a [`ExcelDateTime`] instance
and format them with an Excel number format.

Alternatively, if you have enable the `chrono` feature in `rust_xlsxwriter`  you
can use [`chrono::NaiveDateTime`], [`chrono::NaiveDate`] or
[`chrono::NaiveTime`] instances instead.

[`ExcelDateTime`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ExcelDateTime.html
[`chrono::NaiveDate`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
[`chrono::NaiveTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html
[`chrono::NaiveDateTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html

In the example above we create the `ExcelDateTime` instance from the date strings in our
input data and then format it, in Excel, with a number format.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:48:49}}
```

The final addition to our program is the make the "Date" column wider for
clarity using the [`worksheet.set_column_width()`] method.

[`worksheet.set_column_width()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_column_width

