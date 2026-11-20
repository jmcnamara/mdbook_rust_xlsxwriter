# Pivot Tables: Adding pivot tables

Pivot tables in Excel are a way of summarizing, grouping and analyzing a range
of source data. The unique values of the source data fields can be arranged as
row and column labels, and the numeric fields are summarized, for example as a
sum or an average, for each label. Pivot tables can also have filter fields,
different layouts and formatting styles.

The image below shows a pivot table in Excel with the "Pivot Table Fields" list
used to arrange the fields.

![Image of Excel Pivot Table options](../../images/pivot_table_demo.png)

A pivot table is added to a worksheet via the [`Worksheet::add_pivot_table()`]
method. The source data range and fields of a pivot table are configured via a
[`PivotTable`] struct, but the source data is added using standard
[`Worksheet::write()`] methods.

[`PivotTable`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.PivotTable.html
[`Worksheet::write()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write
[`Worksheet::add_pivot_table()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.add_pivot_table

## Some examples:

**Example 1.** Pivot table with no fields.

![Image of pivot table example](../../images/app_pivot_table1.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:28:33}}
```


**Example 2.** Pivot table with a row field and a value field. Note that the
data source can also be specified as a tuple.

![Image of pivot table example](../../images/app_pivot_table2.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:39:49}}
```


**Example 3.** Pivot table with a row field, a column field and a value field.

![Image of pivot table example](../../images/app_pivot_table3.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:55:65}}
```


**Example 4.** Pivot table with a filter field.

![Image of pivot table example](../../images/app_pivot_table4.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:71:82}}
```


**Example 5.** Pivot table with two row fields.

![Image of pivot table example](../../images/app_pivot_table5.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:88:98}}
```


**Example 6.** Pivot table with several value fields that use different
functions.

![Image of pivot table example](../../images/app_pivot_table6.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:104:119}}
```


**Example 7.** Pivot table with a custom name and number format for the value
field.

![Image of pivot table example](../../images/app_pivot_table7.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:125:138}}
```


**Example 8.** Pivot table with a different style and no grand totals.

![Image of pivot table example](../../images/app_pivot_table8.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:144:157}}
```


**Example 9.** Pivot table with a tabular layout.

![Image of pivot table example](../../images/app_pivot_table9.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:163:175}}
```


**Example 10.** Two pivot tables on the same worksheet.

![Image of pivot table example](../../images/app_pivot_table10.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:181:196}}
```


**Example 11.** Pivot table on the same worksheet as the source data.

![Image of pivot table example](../../images/app_pivot_table11.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_pivot_table.rs:202:214}}
```
