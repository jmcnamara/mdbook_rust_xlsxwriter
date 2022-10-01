# Introduction

`rust_xlsxwriter` is a Rust library that can be used to write text, numbers, dates and formulas to multiple worksheets in an Excel 2007+ XLSX file.

It is a port of the [XlsxWriter] Python module by the same author, who also actively maintains a C version [libxlsxwriter] and a Perl version [Excel::Writer::XLSX]. The Rust version is also intended to try address some limitations and frequently requested features of the previous versions, such as the separation of formatting and data writing.

The overall focus of `rust_xlsxwriter` is on performance, on testing, on documentation, and on fidelity with the file format created by Excel.

[XlsxWriter]: https://xlsxwriter.readthedocs.io/index.html
[Excel::Writer::XLSX]: https://github.com/jmcnamara/excel-writer-xlsx
[libxlsxwriter]: https://github.com/jmcnamara/libxlsxwriter