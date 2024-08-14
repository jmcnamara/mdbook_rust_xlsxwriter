# Dealing with formula errors

If there is an error in the syntax of a formula it is usually displayed in Excel
as `#NAME?`. Alternatively you may get a warning from Excel when the file is
loaded. If you encounter an error like this you can debug it using the following
steps:

1. Ensure the formula is valid in Excel by copying and pasting it into a cell.
   Note, this should be done in Excel and **not** other applications such as
   OpenOffice or LibreOffice since they may have slightly different syntax.

2. Ensure the formula is using comma separators instead of semi-colons, see [Non
   US Excel functions and syntax](syntax.md).

3. Ensure the formula is in English, see [Non US Excel functions and
   syntax](syntax.md).

4. Ensure that the formula doesn't contain an Excel 2010+ future function, see
   [Formulas added in Excel 2010 and later](future_functions.md). If it does
   then ensure that the correct prefix is used.

5. If the function loads in Excel but appears with one or more `@` symbols added
   then it is probably an array function and should be written using
   [`Worksheet::write_array_formula()`] or
   [`Worksheet::write_dynamic_array_formula()`] (see also [Dynamic Array
   support](dynamic_arrays.md)).

[`Worksheet::write_array_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_array_formula
[`Worksheet::write_dynamic_array_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_dynamic_array_formula

Finally if you have completed all the previous steps and still get a `#NAME?`
error you can examine a valid Excel file to see what the correct syntax should
be. To do this you should create a valid formula in Excel and save the file. You
can then examine the XML in the unzipped file.

The following shows how to do that using Linux `unzip` and libxml's
[xmllint](http://xmlsoft.org/xmllint.html) to format the XML for clarity:

```bash
    $ unzip myfile.xlsx -d myfile
    $ xmllint --format myfile/xl/worksheets/sheet1.xml | grep '</f>'

            <f>SUM(1, 2, 3)</f>
```