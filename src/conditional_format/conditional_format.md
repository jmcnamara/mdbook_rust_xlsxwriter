# Working with Conditional Formats

Conditional formatting is a feature of Excel which allows you to apply a
format to a cell or a range of cells based on certain criteria. For example
you might apply rules like the following to highlight cells in different
ranges.

![conditional_format_dialog](../../images/conditional_format_dialog.png)

With `rust_xlsxwriter` we could emulate this by creating [`Format`]
instances and applying them to conditional format rules, like this:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_conditional_format_cell1.rs:45:57}}
```

Which would produce an output like the following. Cells with values >= 50
are in light red. Values < 50 are in light green.

![conditional_format_cell1](../../images/conditional_format_cell1.png)




## Replicating an Excel conditional format with `rust_xlsxwriter`

It is important not to try to reverse engineer Excel's conditional
formatting rules from `rust_xlsxwriter`. If you aren't familiar with the
syntax and functionality of conditional formats then a better place to start
is in Excel. Create a conditional format in Excel to meet your needs and
then port it over to `rust_xlsxwriter`.

There are several common features of conditional formats (although some
variants do not have all of these):

- A range: The range that the conditional format applies to. This is usually
  set via the
  [`Worksheet::add_conditional_format()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.add_conditional_format)
  method.
- A rule: This can be an equality like `>=` or a rule like "Top 10".
- A target: This is usually a cell or range that the rule applies to. This
  mainly applies to Cell style conditional formats. For other types of
  conditional format the "range" is the target.
- A format: The cell format with properties such as text or background color
  to high the cell if the rule matches. This only applies to "Classic"
  conditional formats, see below.

Excel has four main categories of conditional format: Classic, Color Scale,
Data Bar and Icon Sets. The Classic variants apply a format based on a rule.
The newer versions apply a visualization such as a color scale, data bar or
icons based on a rule.

The following are the structs that represent the main conditional format
variants in Excel. See each of these sections for more information:

- Classic:
  - [`ConditionalFormatCell`]: The Cell style conditional format. This is
    the most common style of conditional formats which uses simple
    equalities such as "equal to" or "greater than" or "between". See the
    example above.
  - [`ConditionalFormatAverage`]: The Average/Standard Deviation style
    conditional format.
  - [`ConditionalFormatBlank`]: The Blank/Non-blank style conditional
    format.
  - [`ConditionalFormatDate`]: The Dates Occurring style conditional format.
  - [`ConditionalFormatDuplicate`]: The Duplicate/Unique style conditional
    format.
  - [`ConditionalFormatError`]: The Error/Non-error style conditional
    format.
  - [`ConditionalFormatFormula`]: The Formula style conditional format.
  - [`ConditionalFormatText`]: The Text conditional format for rules like
    "contains" or "begins with".
  - [`ConditionalFormatTop`]: The Top/Bottom style conditional format.
- Color Scale:
  - [`ConditionalFormat2ColorScale`]: The 2 color scale style conditional
    format.
  - [`ConditionalFormat3ColorScale`]: The 3 color scale style conditional
    format.
- Data Bar:
  - [`ConditionalFormatDataBar`]: The Data Bar style conditional format.
- Icon Set:
  - [`ConditionalFormatIconSet`]: The Icon style conditional format.




## Excel's limitations on conditional format properties

When using the "Classic" style of conditional format, see above, it is
important to note that not all of Excel's cell format properties can be
modified or set.

For example the view below of the Excel conditional format dialog shows the
limited number of font properties that can be set. The available properties
are highlighted with green.

![conditional_format_limitations](../../images/conditional_format_limitations.png)

Properties that **cannot** be modified in a conditional format are font
name, font size, superscript and subscript, diagonal borders, all alignment
properties and all protection properties.




## Selecting a non-contiguous range

In Excel it is possible to select several non-contiguous cells or ranges
like `"B3:D6 I3:K6 B9:D12 I9:K12"` and apply a conditional format to them.

It is possible to achieve a similar effect with `rust_xlsxwriter` by using
repeated calls to
[`Worksheet::add_conditional_format()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.add_conditional_format).
However this approach results in multiple identical conditional formats
applied to different cell ranges rather than one conditional format applied
to a multiple range selection.

If this distinction is important to you then you can get the Excel multiple
selection effect using the `set_multi_range()` which is provided for all the
`ConditionalFormat` types. See the example below and note that the cells
outside the selected ranges do not have any conditional formatting.


Note, you can use an Excel range like
`"$B$3:$D$6,$I$3:$K$6,$B$9:$D$12,$I$9:$K$12"` or omit the `$` anchors and
replace the commas with spaces to have a clearer range like `"B3:D6 I3:K6
B9:D12 I9:K12"`. The documentation and examples use the latter format for
clarity but it you are copying and pasting from Excel you can use the first
format.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_conditional_format_multi_range.rs:47:61}}
```

This creates conditional format rules like this:

![conditional_format_multi_range_rules](../../images/conditional_format_multi_range_rules.png)

And the following output file:

![conditional_format_multi_range](../../images/conditional_format_multi_range.png)




## Relative and absolute references in conditional formats

When dealing with Excel conditional formats it is important to distinguish
between relative and absolute cell references in Excel.

Relative cell references change when they are copied while absolute
references maintain fixed row and column references. In Excel absolute
references are prefixed by the dollar symbol like `$A$1`. See the Microsoft
Office documentation for more information on [relative and absolute
references].

[relative and absolute references]:
    https://support.microsoft.com/en-us/office/switch-between-relative-absolute-and-mixed-references-dfec08cd-ae65-4f56-839e-5f0d8d0baca9

In conditional formats each of these four variants has a different effect on
how the conditional format is applied:

- `A1`: Column and row are relative. The conditional format rule is applied
  to each cell in the conditional format range.
- `$A1`: Column is absolute and row is relative. The conditional format rule
  is applied to each row based on the first cell in the row.
- `A$1`: Column is relative and row is absolute. The conditional format rule
  is applied to each column based on the first cell in the column.
- `$A$1`: Column and row are absolute. The conditional format rule is
  applied to the entire range based on the first cell in the range.

The effect is shown in the following example that highlights cells with even
numbered values:


```rust
# // This code is available in examples/doc_conditional_format_anchor.rs
#
# use rust_xlsxwriter::{ConditionalFormatFormula, Format, Workbook, XlsxError};
#
# fn main() -> Result<(), XlsxError> {
#     // Create a new Excel file object.
#     let mut workbook = Workbook::new();
#
#     // Add a format. Green fill with dark green text.
#     let format = Format::new()
#         .set_font_color("006100")
#         .set_background_color("C6EFCE");
#
#     // Add some sample data.
#     let data = [
#         [34, 73, 39, 32, 75, 48, 75, 66],
#         [5, 24, 1, 84, 54, 62, 60, 3],
#         [28, 79, 97, 13, 85, 93, 93, 22],
#         [27, 71, 40, 17, 18, 79, 90, 93],
#         [88, 25, 33, 23, 67, 1, 59, 79],
#         [23, 100, 20, 88, 29, 33, 38, 54],
#         [7, 57, 88, 28, 10, 26, 37, 7],
#         [53, 78, 1, 96, 26, 45, 47, 33],
#         [60, 54, 81, 66, 81, 90, 80, 93],
#         [70, 5, 46, 14, 71, 19, 66, 36],
#     ];
#
#     // Add a new worksheet and write the sample data.
#     let worksheet = workbook.add_worksheet();
#     worksheet.write_row_matrix(2, 1, data)?;
#
    // The rule is applied to each cell in the range.
    let conditional_format = ConditionalFormatFormula::new()
        .set_rule("=ISEVEN(B3)")
        .set_format(&format);

#     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
#
#     // Add a new worksheet and write the sample data.
#     let worksheet = workbook.add_worksheet();
#     worksheet.write_row_matrix(2, 1, data)?;
#
    // The rule is applied to each row based on the first row in the column.
    let conditional_format = ConditionalFormatFormula::new()
        .set_rule("=ISEVEN($B3)")
        .set_format(&format);

#     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
#
#     // Add a new worksheet and write the sample data.
#     let worksheet = workbook.add_worksheet();
#     worksheet.write_row_matrix(2, 1, data)?;
#
    // The rule is applied to each column based on the first cell in the column.
    let conditional_format = ConditionalFormatFormula::new()
        .set_rule("=ISEVEN(B$3)")
        .set_format(&format);

#     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
#
#     // Add a new worksheet and write the sample data.
#     let worksheet = workbook.add_worksheet();
#     worksheet.write_row_matrix(2, 1, data)?;
#
    // The rule is applied to the entire range based on the first cell in the range.
    let conditional_format = ConditionalFormatFormula::new()
        .set_rule("=ISEVEN($B$3)")
        .set_format(format);

#     worksheet.add_conditional_format(2, 1, 11, 8, &conditional_format)?;
#
#     // Save the file.
#     workbook.save("conditional_format.xlsx")?;
#
#     Ok(())
# }
```

Output for the formula `=ISEVEN(B3)`, note that each cell in the range is
evaluated individually:

![conditional_format_anchor1](../../images/conditional_format_anchor1.png)

Output for the formula `=ISEVEN($B3)`, note that the first cell in the row
is evaluated and the row is formatted appropriately:

![conditional_format_anchor2](../../images/conditional_format_anchor2.png)

Output for the formula `=ISEVEN(B$3)`, note that the first cell in the
column is evaluated and the column is formatted appropriately:

![conditional_format_anchor3](../../images/conditional_format_anchor3.png)

Output for the formula `=ISEVEN($B$3)`, note that the first cell in the
range is evaluated and the entire range is formatted appropriately:

![conditional_format_anchor4](../../images/conditional_format_anchor4.png)



## Examples

Conditional formatting is a feature of Excel which allows you to apply a
format to a cell or a range of cells based on user defined rules. For
example you might apply rules like the following to highlight cells in
different ranges.

![conditional_format_dialog](../../images/conditional_format_dialog.png)

The examples below show how to use the various types of conditional
formatting with `rust_xlsxwriter`.

### Some examples:

**Example 1.** Cell conditional formatting. Cells with values >= 50 are in
light red. Values < 50 are in light green.

See [`ConditionalFormatCell`] for more details.

[`ConditionalFormatCell`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatCell.html

![conditional_formats1](../../images/conditional_formats1.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:71:83}}
```


**Example 2.** Cell conditional formatting with between ranges. Values
between 30 and 70 are in light red. Values outside that range are in light
green.

See [`ConditionalFormatCell`] for more details.

[`ConditionalFormatCell`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatCell.html

![conditional_formats2](../../images/conditional_formats2.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:103:115}}
```


**Example 3.** Duplicate and Unique conditional formats. Duplicate values
are in light red. Unique values are in light green.

See [`ConditionalFormatDuplicate`] for more details.

[`ConditionalFormatDuplicate`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatDuplicate.html

![conditional_formats3](../../images/conditional_formats3.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:134:145}}
```


**Example 4.** Above and Below Average conditional formats. Above average
values are in light red. Below average values are in light green.

See [`ConditionalFormatAverage`] for more details.

[`ConditionalFormatAverage`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatAverage.html

![conditional_formats4](../../images/conditional_formats4.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:164:174}}
```


**Example 5.** Top and Bottom range conditional formats. Top 10 values are
in light red. Bottom 10 values are in light green.

See [`ConditionalFormatTop`] for more details.

[`ConditionalFormatTop`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatTop.html

![conditional_formats5](../../images/conditional_formats5.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:193:205}}
```


**Example 6.** Cell conditional formatting in non-contiguous range. Cells
with values >= 50 are in light red. Values < 50 are in light green.
Non-contiguous ranges.

![conditional_formats6](../../images/conditional_formats6.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:224:238}}
```


**Example 7.** Formula conditional formatting. Even numbered cells are in
light green. Odd numbered cells are in light red.

See [`ConditionalFormatFormula`] for more details.

[`ConditionalFormatFormula`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatFormula.html

![conditional_formats7](../../images/conditional_formats7.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:257:269}}
```


**Example 8.** Text style conditional formats. Column A shows words that
contain the sub-word 'rust'. Column C shows words that start/end with 't'

See [`ConditionalFormatText`] for more details.

[`ConditionalFormatText`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatText.html

![conditional_formats8](../../images/conditional_formats8.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:306:334}}
```


**Example 9.** Examples of 2 color scale conditional formats.

See [`ConditionalFormat2ColorScale`] for more details.

[`ConditionalFormat2ColorScale`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormat2ColorScale.html

![conditional_formats9](../../images/conditional_formats9.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:359:394}}
```


**Example 10.** Examples of 3 color scale conditional formats.

See [`ConditionalFormat3ColorScale`] for more details.

[`ConditionalFormat3ColorScale`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormat3ColorScale.html

![conditional_formats10](../../images/conditional_formats10.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:419:460}}
```


**Example 11.** Examples of data bars.

See [`ConditionalFormatDataBar`] for more details.

[`ConditionalFormatDataBar`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatDataBar.html

![conditional_formats11](../../images/conditional_formats11.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:485:504}}
```


**Example 12.** Examples of icon style conditional formats.


See [`ConditionalFormatIconSet`] for more details.

[`ConditionalFormatIconSet`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/conditional_format/struct.ConditionalFormatIconSet.html

![conditional_formats12](../../images/conditional_formats12.png)

Code to generate the above example:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:553:625}}
```


[`Format`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html
