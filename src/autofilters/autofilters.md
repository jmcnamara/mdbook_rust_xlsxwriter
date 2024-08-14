# Working with Autofilters

An autofilter is a way of adding drop down lists to the headers of a 2D range of
worksheet data. This allows users to filter the data based on simple criteria so
that some data is shown and some is hidden.

![Image of output from app_autofilter.rs](../../images/app_autofilter1.png)

In rust_xlsxwriter this is set using the [`Worksheet::autofilter()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:27:27}}
```

## Filter types

Excel supports two main types of filter conditions. The first, and most common,
is a list filter where the user selects the items to filter from a list of all
the values in the the column range:

![Excel autofilter dialog](../../images/autofilter_list.png)

The other main type of filter is a custom filter where the user can specify 1 or
2 conditions like ">= 4000" and "<= 6000":

![Excel autofilter dialog](../../images/autofilter_custom.png)

In Excel these are mutually exclusive and you will need to choose one or the
other depending on your needs.

In rust_xlsxwriter you can set these filters using the [`FilterCondition`] struct and
the [`FilterCondition.add_list_filter()`] and
[`FilterCondition.add_custom_filter()`] methods. Some examples of these are shown in the next section.


## Filter examples

Using the autofilter data and range shown above we will look at some examples of
setting filters to highlight certain rows.

The first example is a list filter to show only rows that are in the "East"
region in the first column:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:42:43}}
```

Example output. Note that the filtered column shows a funnel symbol as part of
the dropdown arrow and the non matching rows are filtered out:

![Image of output from app_autofilter.rs](../../images/app_autofilter2.png)


Multiple list conditions can be set by repeating [`FilterCondition.add_list_filter()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:58:62}}
```

Example output:

![Image of output from app_autofilter.rs](../../images/app_autofilter3.png)


If the data contains blanks you can filter those cells as part of a list filter
or on their own using [`FilterCondition.add_list_blanks_filter()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:76:77}}
```

Example output:

![Image of output from app_autofilter.rs](../../images/app_autofilter4.png)


Filters can be added to more than one autofilter column to show specific data
only:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:91:95}}
```

Example output, note the filters on columns A and D:

![Image of output from app_autofilter.rs](../../images/app_autofilter5.png)


For finer grained control of the filtered data Excel allows 1 or 2 "custom"
filters for conditions like `>=` or `begins with` (for strings):

![Excel autofilter dialog](../../images/autofilter_custom_menu.png)

In rust_xlsxwriter you can set custom filters using the [`FilterCondition`]
struct and the [`FilterCondition.add_custom_filter()`] method. The
criteria/operators are set using the [`FilterCriteria`] struct:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:109:111}}
```

Example output:

![Image of output from app_autofilter.rs](../../images/app_autofilter6.png)


Using both custom filter conditions you can create conditions like "between":

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:125:128}}
```

Example output:

![Image of output from app_autofilter.rs](../../images/app_autofilter7.png)

Note, Excel defaults to using an "And" operator for 2 custom filters but you can
set an "Or" operator using the [`FilterCondition.add_custom_boolean_or()`]
method.

## Filtering Non-blanks

Excel supports, and uses, two different way to filter on non-blanks. The first
is to filter all the unique non-blank strings/numbers in the column.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:143:148}}
```

However, if this is programmatically difficult to set up  you can add a simpler
custom filter to get the same result:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_autofilter.rs:153:155}}
```

Excel uses both these methods depending on context. Example output from either
method:

![Image of output from app_autofilter.rs](../../images/app_autofilter8.png)


## Auto-hiding filtered rows

When you add a filter condition to an autofilter in Excel it automatically hides
all the rows that don't match the filter. This is something that happens at
runtime and isn't part of the file format.

In order to simulate this behavior the rust_xlsxwriter library iterates through
the worksheet data in the autofilter range and hides any rows that don't match.
This is an additional feature that isn't available in the other language ports
of "xlsxwriter". In those versions the programmer has to iterate through the
input data and hide the rows manually.

In general the auto-hiding in rust_xlsxwriter works as expected, as can be seen
in the examples above. However, there are some limitations such as:

- Only String, Number and Blank cells are currently handled.
- The return values from formulas are generally unknown and unhandled.
- Excel supports some simple regex matching of strings with `?` and `*` when
  used with `FilterCriteria::Contains` and `DoesNotContain`. These are not
  currently supported.

If you have some filter criteria that isn't handled correctly you can add to the
filtered rows by using [`Worksheet::set_row_hidden()`].

If the auto-hiding is incorrect you can also turn it off and handle it manually
using [`Worksheet::filter_automatic_off()`] or [`Worksheet::set_row_unhidden()`].




[`FilterCriteria`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCriteria.html
[`FilterCondition`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCondition.html
[`Worksheet::autofilter()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.autofilter
[`Worksheet::set_row_hidden()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_row_hidden
[`Worksheet::set_row_unhidden()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_row_unhidden
[`Worksheet::filter_automatic_off()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.filter_automatic_off
[`FilterCondition.add_list_filter()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCondition.html#method.add_list_filter
[`FilterCondition.add_custom_filter()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCondition.html#method.add_custom_filter
[`FilterCondition.add_custom_boolean_or()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCondition.html#method.add_custom_boolean_or
[`FilterCondition.add_list_blanks_filter()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.FilterCondition.html#method.add_list_blanks_filter
