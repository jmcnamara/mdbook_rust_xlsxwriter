# Adding dates and more formatting

Let's extend the program a little bit more to add some dates to the data:

```rust,ignore
{{#include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:12:17}}
```

The corresponding spreadsheet will look like this:

![Image of first tutorial 3](../images/tutorial3.png)

The differences here are that we have added a "Date" column with formatting and
made that column a little wider to accommodate the dates.

To do this we can extend our program as follows:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:7:}}
```

Dates and times in Excel are floating point numbers that have a number format
applied to display them in the correct format. In order to handle dates and
times with `rust_xlsxwriter` we create them as [`chrono::NaiveDateTime`],
[`chrono::NaiveDate`] or [`chrono::NaiveTime`] instances and format them with an
Excel number format.

[`chrono::NaiveDateTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html
[`chrono::NaiveDate`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
[`chrono::NaiveTime`]: https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html

In the example above we create the NaiveDates from the date strings in our
input data and then format it, in Excel, with a number format.

```rust,ignore
{{#include ../../../rust_xlsxwriter/examples/app_tutorial3.rs:29}}
```

The final addition to our program is the make the "Date" column wider for
clarity using the [`worksheet.set_column_width()`] method.

[`worksheet.set_column_width()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_column_width
