# Chart: Create a combined pareto chart

Example of creating a Pareto chart with a secondary chart and axis.

A Pareto chart is a type of chart that combines a Column/Histogram chart and a
Chart. Individual values are represented in descending order by the columns and
the cumulative total is represented by the line approaching 100% on a second
axis.

**Image of the output file:**

![Image of Excel chart generated by sample code](../../images/app_chart_pareto.png)


**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_pareto.rs:7:}}
```
