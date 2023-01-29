# Autofitting column widths

Column widths in a `rust_xlsxwriter` worksheet can be adjusted automatically
using the [`worksheet.set_autofit()`] method.


There is no option in the xlsx file format that `rust_xlsxwriter` can be used to
say "autofit columns on loading". Auto-fitting of columns is something that
Excel does at runtime when it has access to all of the worksheet information as
well as the Windows functions for calculating display areas based on fonts and
formatting.

As such [`worksheet.set_autofit()`] simulates this behavior by calculating string
widths using metrics taken from Excel. This isn't perfect but for most cases it
should be sufficient and if not you can set your own widths, see below.

The `worksheet.set_autofit()` method ignores columns that already have an
explicit column width set via [`worksheet.set_column_width()`] or
[`worksheet.set_column_width_pixels()`]. You can use these methods if you wish
to override the `worksheet.set_autofit()` values.

It is intended that the `worksheet.set_autofit()` fitting algorithm will be
improved in upcoming releases.

**Note**, `worksheet.set_autofit()` iterates through all the cells in a
worksheet that have been populated with data and performs a length calculation
on each one, so it can have a performance overhead for bigger worksheets.

See also the [Autofitting Columns](../examples/autofit.md) example.

[`worksheet.set_autofit()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_autofit
[`worksheet.set_column_width()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_column_width
[`worksheet.set_column_width_pixels()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_column_width_pixels
