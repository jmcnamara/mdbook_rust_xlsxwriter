# Dynamic Array support


In Office 365 Excel introduced the concept of "Dynamic Arrays" and new functions
that use them. The new functions are:

- `FILTER`
- `RANDARRAY`
- `SEQUENCE`
- `SORTBY`
- `SORT`
- `UNIQUE`
- `XLOOKUP`
- `XMATCH`

The following special case functions were also added with Dynamic Arrays:

- `SINGLE`: Explained below in **The Implicit Intersection Operator "@"**
- `ANCHORARRAY`:  Explained below in **The Spilled Range Operator "#"**


## Dynamic Arrays - An introduction

Dynamic arrays in Excel are ranges of return values that can change size based
on the results. For example, a function such as `FILTER()` returns an array of
values that can vary in size depending on the the filter results:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_dynamic_arrays.rs:29}}
```

This formula gives the results shown in the image below. The dynamic range
here is "F2:I5" but it can vary based on the filter criteria.

![Image 2 of output from app_dynamic_arrays.rs](../../images/dynamic_arrays02.png)

It is also possible to get dynamic array behavior with older Excel functions.
For example, the Excel function `"=LEN(A1)"` applies to a single cell and
returns a single value but it can also apply to a range of cells and return a
range of values using an array formula like `"{=LEN(A1:A3)}"`. This type of
"static" array behavior is referred to as a CSE (Ctrl+Shift+Enter) formula and
has existed in Excel since early versions. In Office 365 Excel updated and
extended this behavior to create the concept of dynamic arrays. In Excel 365 you
can now write the previous LEN function as `"=LEN(A1:A3)"` and get a dynamic
range of return values:

![Image of output from doc_working_with_formulas_dynamic_len.rs](../../images/intersection03.png)

The difference between the two types of array functions is explained in the
Microsoft documentation on [Dynamic array formulas vs. legacy CSE array
formulas].

 In `rust_xlsxwriter` you can use the [`worksheet.write_array_formula()`]
function to get a static/CSE range and
[`worksheet.write_dynamic_array_formula()`] or
[`worksheet.write_dynamic_formula()`] to get a dynamic range.

[`worksheet.write_array_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_array_formula
[`worksheet.write_dynamic_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_dynamic_formula
[`worksheet.write_dynamic_array_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.write_dynamic_array_formula

[Dynamic array formulas in Excel]: https://exceljet.net/dynamic-array-formulas-in-excel
[Dynamic array formulas vs. legacy CSE array formulas]: https://support.microsoft.com/en-us/office/dynamic-array-formulas-vs-legacy-cse-array-formulas-ca421f1b-fbb2-4c99-9924-df571bd4f1b4

The `worksheet.write_dynamic_array_formula()` function takes a `(first_row,
first_col, last_row, last_col)` cell range to define the area that the formula
applies to. However, since the range is dynamic this generally won't be known in
advance in which case you can specify the range with the same start and end
cell. The following range is "F2:F2":

```rust
    worksheet1.write_dynamic_array_formula(1, 5, 1, 5, "=FILTER(A1:D17,C1:C17=K2)")?;
```
As a syntactic shortcut you can use the `worksheet.write_dynamic_formula()`
function which only requires the start cell:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_dynamic_arrays.rs:29}}
```

For a wider and more general introduction to dynamic arrays see the following:
[Dynamic array formulas in Excel].


## The Implicit Intersection Operator "@"

The Implicit Intersection Operator, "@", is used by Excel 365 to indicate a
position in a formula that is implicitly returning a single value when a range
or an array could be returned.

We can see how this operator works in practice by considering the formula we
used in the last section: `=LEN(A1:A3)`. In Excel versions without support for
dynamic arrays, i.e. prior to Excel 365, this formula would operate on a single
value from the input range and return a single value, like the following in
Excel 2011:

![Image of output from doc_working_with_formulas_static_len.rs in older Excel versions](../../images/intersection01.png)


There is an implicit conversion here of the range of input values, "A1:A3", to a
single value "A1". Since this was the default behavior of older versions of
Excel this conversion isn't highlighted in any way. But if you open the same
file in Excel 365 it will appear as follows:

![Image of output from doc_working_with_formulas_static_len.rs](../../images/intersection02.png)


The result of the formula is the same (this is important to note) and it still
operates on, and returns, a single value. However the formula now contains a "@"
operator to show that it is implicitly using a single value from the given
range.

In Excel 365, and with `worksheet.write_dynamic_formula()` in `rust_xlsxwriter`,
it would operate on the entire range and return an array of values:

![Image of output from doc_working_with_formulas_dynamic_len.rs](../../images/intersection03.png)


If you are encountering the Implicit Intersection Operator "@" for the first
time then it is probably from a point of view of "why is Excel/rust_xlsxwriter
putting @s in my formulas". In practical terms if you encounter this operator,
and you don't intend it to be there, then you should probably write the formula
as a CSE or dynamic array function using [`worksheet.write_array_formula()`] or
[`worksheet.write_dynamic_array_formula()`]


A full explanation of this operator is given in the Microsoft documentation on
the [Implicit intersection operator: @].

[Implicit intersection operator: @]: https://support.microsoft.com/en-us/office/implicit-intersection-operator-ce3be07b-0101-4450-a24e-c1c999be2b34?ui=en-us&rs=en-us&ad=us>

One important thing to note is that the "@" operator isn't stored with the
formula. It is just displayed by Excel 365 when reading "legacy" formulas.
However, it is possible to write it to a formula, if necessary, using
`SINGLE()`. The rare cases where this may be necessary are shown in the linked
document in the previous paragraph.


## The Spilled Range Operator "#"

In the sections above we saw that dynamic array formulas can return variable
sized ranges of results. The Excel documentation refers to this as a "Spilled"
range/array from the idea that the results spill into the required number of
cells. This is explained in the Microsoft documentation on [Dynamic array
formulas and spilled array behavior].

[Dynamic array formulas and spilled array behavior]: https://support.microsoft.com/en-us/office/dynamic-array-formulas-and-spilled-array-behavior-205c6b06-03ba-4151-89a1-87a7eb36e531


Since a spilled range is variable in size a new operator is required to refer to
the range. This operator is the [Spilled range operator] and it is represented
by "#". For example, the range `F2#` in the image below is used to refer to a
dynamic array returned by `UNIQUE()` in the cell `F2`:

[Spilled range operator]: https://support.microsoft.com/en-us/office/spilled-range-operator-3dd5899f-bca2-4b9d-a172-3eae9ac22efd

![Image of spill output from app_dynamic_arrays.rs](../../images/spill01.png)

Unfortunately, Excel doesn't store the operator in the formula like this and in
`rust_xlsxwriter` you need to use the explicit function `ANCHORARRAY()` to refer
to a spilled range. The example in the image above was generated using the
following formula:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_dynamic_arrays.rs:205}}
```

## The Excel 365 LAMBDA() function

Recent versions of Excel 365 have introduced a powerful new function/feature
called `LAMBDA()`. This is similar to closure expressions in Rust or [lambda
expressions]
(https://docs.microsoft.com/en-us/cpp/cpp/lambda-expressions-in-cpp?view=msvc-160)
in C++ (and other languages).

Consider the following Excel example which converts the variable `temp` from
Fahrenheit to Celsius:

```
    LAMBDA(temp, (5/9) * (temp-32))
```

This could be called in Excel with an argument:

```
    =LAMBDA(temp, (5/9) * (temp-32))(212)
```

Or assigned to a defined name and called as a user defined function:

```
    =ToCelsius(212)
```

A rust xlsxwriter example that replicates the described Excel functionality is
shown below:


```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_lambda.rs:8:}}
```

Note, that the formula name must have a "_xlfn." prefix and the parameters in
the `LAMBDA()` function must have a "_xlpm."  prefix for compatibility with how
the formulas are stored in Excel. These prefixes won't show up in the formula,
as shown in the image.

Image of the output file:

![Image of output from app_lambda.rs](../../images/app_lambda.png)

The `LET()` function is often used in conjunction with `LAMBDA()` to assign
names to calculation results.
