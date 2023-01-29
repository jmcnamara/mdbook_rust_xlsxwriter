# Performance

The `rust_xlsxwriter` library has sister libraries written in C
([libxlsxwriter]), Python ([XlsxWriter]) and Perl ([Excel::Writer::XLSX]).

[libxlsxwriter]: https://libxlsxwriter.github.io
[XlsxWriter]: https://xlsxwriter.readthedocs.io/index.html
[Excel::Writer::XLSX]: https://metacpan.org/dist/Excel-Writer-XLSX/view/lib/Excel/Writer/XLSX.pm

A [hyperfine] performance comparison between the C, Rust and Python versions is
shown below. The Perl performance is similar to the Python library.

[hyperfine]: https://lib.rs/crates/hyperfine

```
$ hyperfine target/release/examples/app_perf_test ./c_perf_test "python py_perf_test.py"
Benchmark 1: target/release/examples/app_perf_test
  Time (mean ± σ):     447.2 ms ±   8.3 ms    [User: 402.8 ms, System: 39.4 ms]
  Range (min … max):   431.5 ms … 460.7 ms    10 runs

Benchmark 2: ./c_perf_test
  Time (mean ± σ):     362.5 ms ±   6.4 ms    [User: 305.2 ms, System: 53.0 ms]
  Range (min … max):   353.2 ms … 371.9 ms    10 runs

Benchmark 3: python py_perf_test.py
  Time (mean ± σ):      2.899 s ±  0.023 s    [User: 2.787 s, System: 0.088 s]
  Range (min … max):    2.868 s …  2.934 s    10 runs

Summary
  './c_perf_test' ran
    1.23 ± 0.03 times faster than 'target/release/examples/app_perf_test'
    8.00 ± 0.16 times faster than 'python py_perf_test.py'
```

Or in other words, the C version is the fastest and if we take that as 1 then
the rust version is 1.2x (or 20%) slower and the Python version is 8x slower.

As with any performance test there are a lot of factors that may affect the
results, however these results are indicative of the relative performance.

The programs used to generate these results are shown below.

## Rust

```rust
{{#rustdoc_include ../../rust_xlsxwriter/examples/app_perf_test.rs:8:}}
```

## C

```C
#include "xlsxwriter.h"

int main() {

    int max_row = 4000;
    int max_col = 50;

    lxw_workbook  *workbook  = workbook_new("c_perf_test.xlsx");
    lxw_worksheet *worksheet = workbook_add_worksheet(workbook, NULL);

    for (int row_num = 0; row_num < max_row; row_num++) {
        for (int col_num = 0; col_num < max_col; col_num++) {
            if (col_num % 2)
                worksheet_write_string(worksheet, row_num, col_num, "Foo", NULL);
            else
                worksheet_write_number(worksheet, row_num, col_num, 12345.0, NULL);

        }
    }

    workbook_close(workbook);

    return 0;
}
```

## Python

```python
import xlsxwriter

row_max = 4000
col_max = 50

workbook = xlsxwriter.Workbook('py_perf_test.xlsx')
worksheet = workbook.add_worksheet()

for row in range(0, row_max):
    for col in range(0, col_max):
        if col % 2:
            worksheet.write_string(row, col, "Foo")
        else:
            worksheet.write_number(row, col, 12345)

workbook.close()
```