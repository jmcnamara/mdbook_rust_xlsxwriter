# Working with Macros

This section explains how to add a VBA file containing functions or macros to a
rust_xlsxwriter file.

![Image of output from app_macros.rs](../../images/app_macros.png)

## The Excel XLSM File Format

An Excel `xlsm` file is structurally the same as an `xlsx` file, except that it
contains an additional `vbaProject.bin` binary file containing VBA functions
and/or macros.

Excel `xlsm` files are subject to additional security checks and warnings when
loaded:

![Excel macro security warning](../../images/doc_macros_warning.png)


## How VBA macros are included in rust_xlsxwriter

The `vbaProject.bin` in a `xlsm` file is a binary OLE COM container. This was
the format used in older `xls` versions of Excel prior to Excel 2007. Unlike
other components of an `xlsx`/`xlsm` file, the data isn't stored in XML format.
Instead, the functions and macros are stored in a pre-parsed binary format. As
such, it wouldn't be feasible to programmatically define macros and create a
`vbaProject.bin` file from scratch (at least not in the remaining lifespan and
interest levels of the author).

Instead, as a workaround, a utility is used to extract `vbaProject.bin` files
from existing `xlsm` files, which you can then add to rust_xlsxwriter files.


## The `vba_extract` Utility

The Rust [`vba_extract`](https://crates.io/crates/vba_extract) utility is used
to extract the `vbaProject.bin` binary from an Excel `xlsm` file. The utility
can be installed via `cargo`:

```bash
$ cargo install vba_extract
```

Once `vba_extract` is installed, it can be used as follows:

```bash
$ vba_extract macro_file.xlsm

Extracted: vbaProject.bin
```

If the VBA project is signed, `vba_extract` also extracts the
`vbaProjectSignature.bin` file from the `xlsm` file (see below).

The syntax and options for `vba_extract` are:

```text
$ vba_extract --help

Utility to extract a `vbaProject.bin` binary from an Excel `xlsm` macro file
for insertion into a `rust_xlsxwriter` file. If the macros are digitally
signed, it also extracts a `vbaProjectSignature.bin` file.

Usage: vba_extract [OPTIONS] <FILENAME_XLSM>

Arguments:
  <FILENAME_XLSM>
          Input Excel xlsm filename

Options:
  -o, --output-macro-filename <OUTPUT_MACRO_FILENAME>
          Output vba macro filename

          [default: vbaProject.bin]

  -s, --output-sig-filename <OUTPUT_SIG_FILENAME>
          Output vba signature filename (if present in the parent file)

          [default: vbaProjectSignature.bin]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


## Adding VBA Macros to a rust_xlsxwriter File

Once the `vbaProject.bin` file has been extracted, it can be added to the
rust_xlsxwriter workbook using the [`Workbook::add_vba_project()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_macros_add.rs:13:13}}
```

Here is a complete example which adds a macro file with a dialog. It also uses a
button, via [`Worksheet::insert_button()`], to trigger the macro:

```rust
{{#include ../../../rust_xlsxwriter/examples/app_macros.rs:11:}}
```

The macro in this example is the following VBA code:

```basic
Sub say_hello()
    MsgBox ("Hello from Rust!")
End Sub
```

Output file after running macro:

![Image of output from app_macros.rs](../../images/app_macros.png)

If the VBA file contains functions you can then refer to them in calculations
using [`Worksheet::write_formula()`]:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_macros_calc.rs:16:16}}
```

**Note**: Excel files that contain functions and macros must use an `.xlsm`
extension or else Excel will complain and possibly not open the file.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_macros_save.rs:17:18}}
```

Here is the dialog that appears when a valid `xlsm` file is incorrectly given a
`xlsx` extension:

![Excel wrong extension warning](../../images/doc_macros_wrong_extension.png)


## Setting the VBA Object Names

VBA macros generally refer to workbook and worksheet objects via names such as
`ThisWorkbook` and `Sheet1`, `Sheet2` etc.

If the imported macro uses other names, you can set them using the
[`Workbook::set_vba_name()`] and [`Worksheet::set_vba_name()`] methods as
follows.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_macros_name.rs:12:16}}
```

**Note**: If you are using a non-English version of Excel you need to pay
particular attention to the workbook/worksheet naming that your version of Excel
uses and add the correct VBA names. You can find the names that are used in the
VBA editor:

![Excel VBA editor](../../images/doc_macros_editor.png)

You can also find them by unzipping the `xlsm` file and grepping the component
XML files. The following shows how to do that using system `unzip` and libxml's
[xmllint](http://xmlsoft.org/xmllint.html) to format the XML for clarity:

```bash
$ unzip myfile.xlsm -d myfile
$ xmllint --format `find myfile -name "*.xml" | xargs` | grep "Pr.*codeName"

    <workbookPr codeName="MyWorkbook" defaultThemeVersion="124226"/>
    <sheetPr codeName="MySheet"/>
```


## Adding a VBA Macro Signature File to a rust_xlsxwriter File

VBA macros can be signed in Excel to allow for further control over execution.
The signature part is added to the `xlsm` file in another binary called
`vbaProjectSignature.bin`.

The `vba_extract` utility will extract the `vbaProject.bin` and
`vbaProjectSignature.bin` files from an `xlsm` file with signed macros.

These files can be added to a rust_xlsxwriter file using the
[`Workbook::add_vba_project_with_signature()`] method:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_macros_signed.rs:13:16}}
```


## What to do if it doesn't work

The rust_xlsxwriter test suite contains several tests to ensure that this
feature works and there is a working example shown above. However, there is no
guarantee that it will work in all cases. Some trial and error may be required
and some knowledge of VBA will certainly help. If things don't work out here are
some things to try:

1. Start with a simple macro file, ensure that it works, and then add
   complexity.

2. Check the code names that macros use to refer to the workbook and worksheets
   (see above). In general VBA uses a code name of `ThisWorkbook` to refer to
   the current workbook and the sheet name (such as `Sheet1`) to refer to the
   worksheets. These are the defaults used by rust_xlsxwriter. If the macro uses
   other names, or the macro was extracted from an non-English language version
   of Excel, then you can specify these using the workbook and worksheet
   `set_vba_name()` methods.




[`Workbook::add_vba_project()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.add_vba_project
[`Workbook::add_vba_project_with_signature()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.add_vba_project_with_signature
[`Workbook::set_vba_name()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/workbook/struct.Workbook.html#method.set_vba_name
[`Worksheet::set_vba_name()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_vba_name
[`Worksheet::insert_button()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.insert_button
[`Worksheet::write_formula()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_formula
