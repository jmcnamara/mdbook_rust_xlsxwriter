# Chart: Stock: Excel Stock chart example

Example of creating Excel Stock charts.

Note, Volume variants of the Excel stock charts aren't currently supported but
will be in a future release.

**Image of the output file:**

Chart 1 in the following example is an example of a High-Low-Close Stock chart.

To create a chart similar to a default Excel High-Low-Close Stock chart
you need to do the following steps:

1. Create a `Stock` type chart.
2. Add 3 series for High, Low and Close, in that order.
3. Hide the default lines in all 3 series.
4. Hide the default markers for the High and Low series.
5. Set a dash marker for the Close series.
6. Turn on the chart High-Low bars.
7. Format any other chart or axis property you need.


![Image of output chart example](../../images/chart_stock1.png)

Chart 2 in the following example is an example of an Open-High-Low-Close Stock chart.

To create a chart similar to a default Excel Open-High-Low-Close Stock
chart you need to do the following steps:

1. Create a `Stock` type chart.
2. Add 4 series for Open, High, Low and Close, in that order.
3. Hide the default lines in all 4 series.
4. Turn on the chart High-Low bars.
5. Turn on the chart Up-Down bars.
6. Format any other chart or axis property you need.


![Image of output chart example](../../images/chart_stock2.png)


**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_stock.rs:10:}}
```