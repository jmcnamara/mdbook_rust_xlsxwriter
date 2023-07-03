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
$ hyperfine ./app_perf_test ./c_perf_test "python py_perf_test.py" --warmup 3
Benchmark 1: ./app_perf_test
  Time (mean ± σ):     233.6 ms ±   1.6 ms    [User: 216.0 ms, System: 15.7 ms]
  Range (min … max):   230.7 ms … 235.7 ms    12 runs

Benchmark 2: ./c_perf_test
  Time (mean ± σ):     197.2 ms ±   2.0 ms    [User: 163.1 ms, System: 28.4 ms]
  Range (min … max):   194.5 ms … 201.0 ms    14 runs

Benchmark 3: python py_perf_test.py
  Time (mean ± σ):      1.099 s ±  0.011 s    [User: 1.051 s, System: 0.041 s]
  Range (min … max):    1.085 s …  1.125 s    10 runs

Summary
  './c_perf_test' ran
    1.18 ± 0.01 times faster than './app_perf_test'
    5.57 ± 0.08 times faster than 'python py_perf_test.py'
```

Or in other words, the C version is the fastest and if we take that as 1 then
the rust version is 1.18x (or 18%) slower and the Python version is 5.5x slower.


The `rust_xlsxwriter` library also has an optional compilation "feature" called
`zlib` which will allow it (via ZipWriter) to use compression from a native C
library. This improves the performance on large files significantly and even
beats the C/libxlsxwriter version:

```
$ hyperfine ./app_perf_test ./app_perf_test_zlib ./c_perf_test --warmup 3
Benchmark 1: ./app_perf_test
  Time (mean ± σ):     244.3 ms ±  10.5 ms    [User: 221.5 ms, System: 18.1 ms]
  Range (min … max):   231.8 ms … 259.8 ms    11 runs

Benchmark 2: ./app_perf_test_zlib
  Time (mean ± σ):     149.4 ms ±  10.5 ms    [User: 129.5 ms, System: 16.1 ms]
  Range (min … max):   140.7 ms … 178.5 ms    18 runs

Benchmark 3: ./c_perf_test
  Time (mean ± σ):     197.7 ms ±   2.3 ms    [User: 163.4 ms, System: 28.4 ms]
  Range (min … max):   195.4 ms … 204.5 ms    14 runs

Summary
  './app_perf_test_zlib' ran
    1.32 ± 0.09 times faster than './c_perf_test'
    1.63 ± 0.13 times faster than './app_perf_test'
```

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