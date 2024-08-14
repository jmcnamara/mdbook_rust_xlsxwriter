# Tables: Adding worksheet tables

Tables in Excel are a way of grouping a range of cells into a single entity
that has common formatting or that can be referenced from formulas. Tables
can have column headers, autofilters, total rows, column formulas and
different formatting styles.

The image below shows a default table in Excel with the default properties
shown in the ribbon bar.

![Image of Excel Table options](../../images/table_intro.png)

A table is added to a worksheet via the [`Worksheet::add_table()`]method. The
headers and total row of a table should be configured via a [`Table`] struct but
the table data can be added via standard [`Worksheet::write()`]methods.

[`Table`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Table.html
[`Worksheet::write()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write
[`Worksheet::add_table()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.add_table

## Some examples:

**Example 1.** Default table with no data.

![Image of table example](../../images/app_tables1.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:30:47}}
```


**Example 2.** Default table with data.

![Image of table example](../../images/app_tables2.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:53:74}}
```


**Example 3.** Table without default autofilter.

![Image of table example](../../images/app_tables3.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:80:101}}
```


**Example 4.** Table without default header row.

![Image of table example](../../images/app_tables4.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:107:128}}
```


**Example 5.** Default table with "First Column" and "Last Column" options.

![Image of table example](../../images/app_tables5.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:134:155}}
```


**Example 6.** Table with banded columns but without default banded rows.

![Image of table example](../../images/app_tables6.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:161:182}}
```


**Example 7.** Table with user defined column headers.

![Image of table example](../../images/app_tables7.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:188:217}}
```


**Example 8.** Table with user defined column headers, and formulas.

![Image of table example](../../images/app_tables8.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:223:255}}
```


**Example 9.** Table with totals row (but no caption or totals).

![Image of table example](../../images/app_tables9.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:261:293}}
```


**Example 10.** Table with totals row with user captions and functions.

![Image of table example](../../images/app_tables10.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:299:342}}
```


**Example 11.** Table with alternative Excel style.

![Image of table example](../../images/app_tables11.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:348:394}}
```


**Example 12.** Table with Excel style removed.

![Image of table example](../../images/app_tables12.png)

Code to generate the above example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_tables.rs:400:449}}
```