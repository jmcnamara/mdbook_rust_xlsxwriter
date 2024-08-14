# Number Format Categories

The [`set_num_format()`] method is used to set the number format for numbers
used with [`Worksheet::write_number()`]. For example the following formats a
number with the Excel number format `"$#,##0.00"`.

[`set_num_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_num_format
[`Worksheet::write_number()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_number


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_format_currency1.rs:9:}}
```
Which give the following output with the category displayed:

![Image of output from doc_format_currency1.rs](../../images/format_currency1.png)

The number category is shown as "Number" but it is also possible to have it
correspond to one of the other number categories such as "Currency" (which we
might have expected in this case).

The Excel number categories are:

- General
- Number
- Currency
- Accounting
- Date
- Time
- Percentage
- Fraction
- Scientific
- Text
- Custom

If we wanted to have the number format display as a different category, such as
"Currency", then would need to match the number format string used by Excel with
the number format used by in our code. The easiest way to do this is to open the
Number Formatting dialog in Excel and set the required format:

![Image of Excel dialog to set number formats](../../images/format_currency2.png)

Then, while still in the dialog, change to Custom. The format displayed is the
format used by Excel.

![Image of Excel dialog to set number formats](../../images/format_currency3.png)

If we put the format that we found (`"[$$-409]#,##0.00"`) into our previous
example and rerun it we will get a number format in the Currency category:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_format_currency2.rs:18:20}}
```

That give us the following updated output. Note that the number category is
now shown as Currency:

![Image of output from doc_format_currency2.rs](../../images/format_currency4.png)

The same process can be used to find format strings for "Date" or
"Accountancy" formats.
