# Working with Charts

The sections below explain some of the options and features when working
with the [`Chart`] struct.




## Creating a chart with `rust_xlsxwriter`

The basic steps for creating a chart in `rust_xlsxwriter` are:

- Create a new [`Chart`] object of the chart type you want.
- Add one or more series to the chart via [`Chart::add_series()`] to define
  the data you wish to plot.
- Add any formatting or additional feature you need.
- Add the chart to the worksheet via
  [`Worksheet::insert_chart()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.insert_chart).

These steps are shown in the example below which creates a minimal chart
that plots data in a worksheet. The program creates a new column [`Chart`],
adds a series via [`Chart::add_series()`] and add the value range the series
refers to via [`ChartSeries::set_values()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_tutorial1.rs:7:36}}
```

This produces a file like this:

![chart_tutorial1](../../images/chart_tutorial1.png)

To improve this a little we will add the category data that the value data
refers to. In this case it is the "Day" data in Column A. We extend the
program by adding the category values via [`ChartSeries::set_categories()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_tutorial2.rs:25:34}}
```
(Note: The section of the program where we write the worksheet data is the
same as the previous example and is omitted in this example. You can view it
in the source or the examples folder.)

The updated file looks like this (the chart data range is highlighted):

![chart_tutorial2](../../images/chart_tutorial2.png)

The previous example works fine but it contained some hard-coded cell ranges
like `set_values("Sheet1!$B$2:$B$6")`. This is okay for simple programs but
if our example changed to have a different number of data items then we
would have to manually change the code to adjust for the new ranges.

Fortunately, these hard-coded values are only used for the sake of the
example and `rust_xlsxwriter` provides APIs to handle these more
programmatically.

In general `rust_xlsxwriter` always provides numeric APIs for any ranges in
Excel but when it makes ergonomic sense it also provides **secondary**
string-based APIs. The previous example uses one of these secondary string-
based APIs for demonstration purposes. For real applications you would set
the chart ranges using 5-tuple values like this:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_tutorial3.rs:24:40}}
```

We could use hard coded row and column indexes for the chart ranges but the
variables make it more flexible and future proof. The output from this
example is exactly the same as the previous image above.

Finally we can improve the output a bit more by inserting chart and axes
titles, by hiding the legend since it doesn't provide much information in
this case, and by shifting the chart a few pixels away from the data for
clarity via
[`Worksheet::insert_chart_with_offset()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.insert_chart_with_offset).

Here is the chart section with these changes:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_tutorial4.rs:30:48}}
```

This produces a file like this:

![chart_tutorial4](../../images/chart_tutorial4.png)

For more examples see chart examples in the [`Cookbook`](../examples/intro.md).




## Supported chart types

The `rust_xlsxwriter` library supports the main original Excel chart types
such as:

- Area
- Bar
- Column
- Doughnut
- Line
- Pie
- Radar
- Stock
- Scatter

See [`ChartType`] for the full list and examples.

Support for newer Excel chart types such as Treemap, Sunburst, Box and
Whisker, Statistical Histogram, Waterfall, Funnel, and Maps is not currently
planned since the underlying structure is substantially different from the
original chart types above.



## Chart formatting

Excel uses a standard dialog for any chart elements that support formatting
such as data series, the plot area, the chart area, the legend or individual
points. It looks like this:

![chart_format_dialog](../../images/chart_format_dialog.png)

In `rust_xlsxwriter` the [`ChartFormat`] struct represents many of these
format options and just like Excel it offers a standard formatting interface
for a number of the chart elements.

The [`ChartFormat`] struct is generally passed to the `set_format()` method
of a chart element. `ChartFormat` supports several child formatting structs
such as:

- [`ChartSolidFill`] properties for solid fills.
- [`ChartPatternFill`] for pattern fills.
- [`ChartGradientFill`] for gradient fills.
- [`ChartLine`] for line/border properties.

As a syntactic shortcut you can pass any of the child format structs to
`set_format()`. This allows you to create and use a [`ChartLine`] struct,
for example, without having to construct a parent [`ChartFormat`]. However,
if you mix more than one type of format you will need a parent
[`ChartFormat`] struct to group them.




## Chart Value and Category Axes

When working with charts it is important to understand how Excel
differentiates between a chart axis that is used for data series
"Categories" and a chart axis that is used for data series "Values".

The majority of Excel charts types have a "Category" X-axis axis where each
of the values is evenly spaced and sequential. The category values can be
strings, like in the example below, or they can be numbers. When the
categories are numbers, Excel treats them as if they were strings, which is
why you can't set a minimum or maximum limit in Category axes.

![chart_axes01](../../images/chart_axes01.png)

<p style="text-align: center;"><b>Column chart with Category X-axis and Value Y-axis</b></p>

The Y-axis of Excel charts is generally a "Value" axis where points are
displayed according to their value. This type of axis does support minimum
and maximum limits.

Scatter charts are an exception to the general rule since they have two
value axes to plot `(x, y)` data that doesn't necessarily lie on
fixed X-axis divisions, as shown in the chart below:

![chart_axes02](../../images/chart_axes02.png)

<p style="text-align: center;"><b>Scatter chart with Value X-axis and Value Y-axis</b></p>

Another variation is a category-style chart with date or time values for
the X-axis. This type of "Date" axis shares properties of Category and Value
axes. Date axes are used in Stock charts or if you provide date/time data
and use the [`ChartAxis::set_date_axis()`] method.

![chart_axes03](../../images/chart_axes03.png)

<p style="text-align: center;"><b>Column chart with Date X-axis and Value Y-axis</b></p>

Finally, another variant is a Bar chart where the Category and Value axes
positions are reversed:

![chart_axes04](../../images/chart_axes04.png)

<p style="text-align: center;"><b>Bar chart with Category Y-axis and Value X-axis</b></p>

In Excel, category and value axes expose different properties, as can be seen
in the dialogs for each type of axis, shown below. Note that for Category
axes, there is no minimum and maximum "Bounds" option:

![chart_axes05](../../images/chart_axes05.png)

![chart_axes06](../../images/chart_axes06.png)

Due to Excel's distinction between axis types, some `rust_xlsxwriter`
[`ChartAxis`] properties can be set for a value axis, some can be set for a
category axis, and some properties can be set for both. For example, `reverse`
can be set for either category or value axes, while the `min` and `max`
properties can only be set for value axes (and date axes). The documentation
calls out the type of axis to which properties apply.

## Secondary Axes

It is possible to add a secondary axis of the same type to a chart by
setting the [`ChartSeries::set_secondary_axis()`] property of the series:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_chart_series_set_secondary_axis.rs:17:37}}
```

Output file:

![chart_series_set_secondary_axis](../../images/chart_series_set_secondary_axis.png)

In general secondary axes are used for displaying different Y values for the
same category range. However it is also possible to display a secondary X
axis for series that use a different category range. See the example below.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_chart_series_set_secondary_axis2.rs:24:62}}
```

Output file:

![chart_series_set_secondary_axis2](../../images/chart_series_set_secondary_axis2.png)


It is also possible to have a secondary, combined, chart either with a
shared or secondary axis, see below.


## Combined Charts

In Excel is also possible to combine two different chart types, for example
a column and line chart to create a Pareto chart. In `rust_xlsxwriter` you
can replicate this by creating a new chart instance as the primary chart and
then create a secondary chart of a different type and combine it with the
primary chart using the [`Chart::combine()`] method.

The combined secondary chart can share the same Y axis as the primary chart,
or it can use a secondary Y2 axis. An example of each is shown below.

Combined Column and Line chart with the same Y axis.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_chart_combine1.rs:22:48}}
```

Output file:

![chart_combine1](../../images/chart_combine1.png)


Combined Column and Line chart with the Column values are on the primary Y
axis and the Line chart values on the secondary Y2 axis.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_chart_combine2.rs:23:53}}
```

Output file:

![chart_combine2](../../images/chart_combine2.png)

The following is a slightly more realistic example of combining chart types
to create a [Pareto Chart](https://en.wikipedia.org/wiki/Pareto_chart).

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_chart_pareto.rs:7:90}}
```

Chart from the output file:

![app_chart_pareto](../../images/app_chart_pareto.png)

The examples above use the concept of a *primary* and *secondary* chart. The
primary chart is the chart that defines the primary X and Y axis. It is also
used for setting all chart properties apart from the secondary data series.
For instance the chart title and axes properties should be set via the
primary chart.

There are some limitations in the `rust_xlsxwriter` implementation of
combined charts:

- Only two charts can be combined.
- Scatter charts cannot currently be used as a primary chart but they can be
  used as a secondary chart.
- Bar charts can only combine secondary charts on a secondary axis. This is
  an Excel limitation.


[`Chart`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.Chart.html
[`Chart::add_series()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.Chart.html#method.add_series
[`ChartType`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/enum.ChartType.html
[`ChartFormat`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartFormat.html
[`ChartSolidFill`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartSolidFill.html
[`ChartPatternFill`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartPatternFill.html
[`ChartGradientFill`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartGradientFill.html
[`ChartLine`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartLine.html
[`ChartAxis::set_date_axis()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartAxis.html#method.set_date_axis
[`ChartAxis`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartAxis.html
[`ChartSeries::set_secondary_axis()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.ChartSeries.html#method.set_secondary_axis
[`Chart::combine()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/chart/struct.Chart.html#method.combine
