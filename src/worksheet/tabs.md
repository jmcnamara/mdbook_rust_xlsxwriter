# Working with worksheet tabs

Worksheet tabs in Excel allow the user to differentiate between different
worksheets.

Worksheets in a workbook can be highlighted via the tab name, color, position or
the fact that it is active when the user opens the workbook.

![Image of a worksheet with four tabs](../../images/worksheet_tabs.png)

The `rust_xlsxwriter` library provides a number of methods, explained below, to
emulate this functionality.

## Worksheet names

The worksheet name can be set with [`worksheet.set_name()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_name.rs:12:15}}
```

Which gives the following output:

![Image of output from doc_worksheet_set_name.rs](../../images/worksheet_set_name.png)

Excel applies various rules to worksheet names such as:

* The name must be less than 32 characters.
* The name cannot be blank.
* The name cannot contain any of the characters: `[ ] : * ? / \`.
* The name cannot start or end with an apostrophe.
* The name shouldn't be "History" (case-insensitive) since that is reserved by
  Excel.
* The name must not be a duplicate (case-insensitive) of another worksheet name
  used in the workbook.

The rules for worksheet names in Excel are explained in the [Microsoft
Office documentation].

## Setting the active worksheet

In Excel the visible worksheet in a group of worksheets is known as the active
worksheet. Since only one worksheet is in the foreground at any one time there
can only be one active worksheet.

With `rust_xlsxwriter` the [`worksheet.set_active()`] method is used to specify
which worksheet is active. If no worksheet is set as the active worksheet then
the default is to have the first one active, like in Excel.

Here is an example of making the second worksheet active:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_active.rs:16}}
```

Which gives the following output:

![Image of output from doc_worksheet_set_active.rs](../../images/worksheet_set_active.png)

If you have a lot of worksheets then they may not all fit on the screen at the
same time. In cases like that the active worksheet will still be visible but its
tab may not be. In those, rare, cases you can use the
[`worksheet.set_first_tab()`] method to set the first visible tab (not
worksheet) in a group of worksheets.

## Setting worksheet tab colors

Another way of highlighting one or more worksheets in Excel is to set the tab
color. With `rust_xlsxwriter` this is achieved with
[`worksheet.set_tab_color()`] and a [`XlsxColor`] color:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_tab_color.rs:11:21}}
```

Which gives the following output:

![Image of output from doc_worksheet_set_tab_color.rs](../../images/worksheet_set_tab_color.png)

## Hiding worksheets

Sometimes it is desirable to hide worksheets if they contain a lot of
intermediate data or calculations that end user doesn't need to see. With
`rust_xlsxwriter` this is achieved with the [`worksheet.set_hidden()`]
method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_hidden.rs:11:15}}
```

Which gives the following output:

![Image of output from doc_worksheet_set_hidden.rs](../../images/worksheet_set_hidden.png)

In Excel a hidden worksheet can not be activated or selected so
`worksheet.set_hidden()` is mutually exclusive with the `worksheet.set_active()`
and `worksheet.set_selected()` methods. In addition, since the first worksheet
will default to being the active worksheet, you cannot hide the first worksheet
without activating another sheet.


## Selecting worksheets

A selected worksheet has its tab highlighted. Selecting worksheets is a way of
grouping them together so that, for example, several worksheets could be printed
in one go.

The [`worksheet.set_selected()`] method is used to indicate that a
worksheet is selected in a multi-sheet workbook.

A worksheet that has been activated via the `worksheet.set_active()` method will
also appear as selected.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_selected.rs:12:17}}
```

Which gives the following output. Note that Sheet 1 and Sheet2 are selected but
Sheet3 isn't:

![Image of output from doc_worksheet_set_selected.rs](../../images/worksheet_set_selected.png)









[`XlsxColor`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/enum.XlsxColor.html
[`worksheet.set_name()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_name
[`worksheet.set_active()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_active
[`worksheet.set_hidden()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_hidden
[`worksheet.set_selected()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_selected
[`worksheet.set_tab_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_tab_color
[`worksheet.set_first_tab()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_first_tab
[Microsoft Office documentation]: https://support.office.com/en-ie/article/rename-a-worksheet-3f1f7148-ee83-404d-8ef0-9ff99fbad1f9
