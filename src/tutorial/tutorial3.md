# Adding dates and more formatting

Let's extend the program a little bit more to add some dates to the data:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:13:18}}
```

The corresponding spreadsheet will look like this:

![Image of tutorial 3](../images/tutorial3.png)

The differences here are that we have added a "Date" column with formatting and
made that column a little wider to accommodate the dates.

To do this we can extend our program as follows:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:8:}}
```

Dates and times in Excel are floating point numbers that have a format applied
to display them in the desired way. In order to handle dates and times with
`rust_xlsxwriter` we create them using a [`ExcelDateTime`] instance and format
them with an Excel number format.

> **Reading ahead**:
>
> If you enable the `chrono` feature in `rust_xlsxwriter`  you can also use
  [`chrono::NaiveDateTime`], [`chrono::NaiveDate`] or [`chrono::NaiveTime`]
  instances.


[`ExcelDateTime`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ExcelDateTime.html
[`chrono::NaiveDate`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
[`chrono::NaiveTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html
[`chrono::NaiveDateTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html

In the example above we create the `ExcelDateTime` instance from the date
strings in our input data and then add a number format it so that it appears
correctly in Excel:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:48:49}}
```

Another addition to our program is the make the "Date" column wider for clarity
using the [`Worksheet::set_column_width()`] method.

[`Worksheet::set_column_width()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_column_width

