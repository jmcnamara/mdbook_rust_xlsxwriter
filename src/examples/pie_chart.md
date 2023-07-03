# Chart: Pie: Excel Pie chart example

Example of creating Excel Pie charts.


**Image of the output file:**

Chart 1 in the following example is a default pie chart:
![Image of output chart example](../../images/chart_pie1.png)

Chart 2 shows how to set segment colors.

It is possible to define chart colors for most types of `rust_xlsxwriter`charts via
the `add_series()` method. However, Pie charts are a special case since each
segment is represented as a point and as such it is necessary to assign
formatting to each point in the series.
![Image of output chart example](../../images/chart_pie2.png)

Chart 3 shows how to rotate the segments of the chart:
![Image of output chart example](../../images/chart_pie3.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_pie.rs:6:}}
```