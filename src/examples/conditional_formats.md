# Conditional Formatting: Adding conditional formatting to worksheets

Conditional formatting is a feature of Excel which allows you to apply a format
to a cell or a range of cells based on user defined rules. For example you might
apply rules like the following to highlight cells in different ranges.

![Image of Excel conditional format rules](../../images/conditional_format_dialog.png)

The examples below show how to use the various types of conditional formatting
with `rust_xlsxwriter`.

## Some examples:

**Example 1.** Cell conditional formatting. Cells with values >= 50 are in
light red. Values < 50 are in light green.

See [`ConditionalFormatCell`] for more details.

[`ConditionalFormatCell`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatCell.html

![Image of conditional formatting example output](../../images/conditional_formats1.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:73:85}}
```


**Example 2.** Cell conditional formatting with between ranges. Values between
30 and 70 are in light red. Values outside that range are in light green.

See [`ConditionalFormatCell`] for more details.

[`ConditionalFormatCell`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatCell.html

![Image of conditional formatting example output](../../images/conditional_formats2.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:107:119}}
```


**Example 3.** Duplicate and Unique conditional formats. Duplicate values are
in light red. Unique values are in light green.

See [`ConditionalFormatDuplicate`] for more details.

[`ConditionalFormatDuplicate`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatDuplicate.html

![Image of conditional formatting example output](../../images/conditional_formats3.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:140:151}}
```


**Example 4.** Above and Below Average conditional formats. Above average
values are in light red. Below average values are in light green.

See [`ConditionalFormatAverage`] for more details.

[`ConditionalFormatAverage`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatAverage.html

![Image of conditional formatting example output](../../images/conditional_formats4.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:172:182}}
```


**Example 5.** Top and Bottom range conditional formats. Top 10 values are in
light red. Bottom 10 values are in light green.

See [`ConditionalFormatTop`] for more details.

[`ConditionalFormatTop`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatTop.html

![Image of conditional formatting example output](../../images/conditional_formats5.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:203:215}}
```


**Example 6.** Cell conditional formatting in non-contiguous range. Cells with
values >= 50 are in light red. Values < 50 are in light green. Non-contiguous
ranges.

![Image of conditional formatting example output](../../images/conditional_formats6.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:236:250}}
```


**Example 7.** Formula conditional formatting. Even numbered cells are in
light green. Odd numbered cells are in light red.

See [`ConditionalFormatFormula`] for more details.

[`ConditionalFormatFormula`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatFormula.html

![Image of conditional formatting example output](../../images/conditional_formats7.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:271:283}}
```


**Example 8.** Text style conditional formats. Column A shows words that
contain the sub-word 'rust'. Column C shows words that start/end with 't'

See [`ConditionalFormatText`] for more details.

[`ConditionalFormatText`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatText.html

![Image of conditional formatting example output](../../images/conditional_formats8.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:320:348}}
```


**Example 9.** Examples of 2 color scale conditional formats.

See [`ConditionalFormat2ColorScale`] for more details.

[`ConditionalFormat2ColorScale`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormat2ColorScale.html

![Image of conditional formatting example output](../../images/conditional_formats9.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:375:410}}
```


**Example 10.** Examples of 3 color scale conditional formats.

See [`ConditionalFormat3ColorScale`] for more details.

[`ConditionalFormat3ColorScale`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormat3ColorScale.html

![Image of conditional formatting example output](../../images/conditional_formats10.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:437:478}}
```


**Example 11.** Examples of data bars.

See [`ConditionalFormatDataBar`] for more details.

[`ConditionalFormatDataBar`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatDataBar.html

![Image of conditional formatting example output](../../images/conditional_formats11.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:503:522}}
```


**Example 12.** Examples of icon style conditional formats.


See [`ConditionalFormatIconSet`] for more details.

[`ConditionalFormatIconSet`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ConditionalFormatIconSet.html

![Image of conditional formatting example output](../../images/conditional_formats12.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_conditional_formatting.rs:571:643}}
```