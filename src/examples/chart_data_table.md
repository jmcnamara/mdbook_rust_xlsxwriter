# Chart: Chart data table

An example of creating Excel Column charts with data tables using the
`rust_xlsxwriter` library.


**Image of the output file:**

Chart 1 in the following code is a Column chart with a default chart data table.
![Image of output chart example](../../images/chart_data_table1.png)

Chart 2 in the following code is a Column chart with a chart data table with legend keys.
![Image of output chart example](../../images/chart_data_table2.png)



**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_data_table.rs:7:}}
```