# Performance

The `rust_xlsxwriter` library has sister libraries written in C
([libxlsxwriter]), Python ([XlsxWriter]) and Perl ([Excel::Writer::XLSX]).

[libxlsxwriter]: https://libxlsxwriter.github.io
[XlsxWriter]: https://xlsxwriter.readthedocs.io/index.html
[Excel::Writer::XLSX]: https://metacpan.org/dist/Excel-Writer-XLSX/view/lib/Excel/Writer/XLSX.pm

A [hyperfine] performance comparison between the C, Rust and Python versions is
shown below. The Perl performance is similar to the Python library so it has been omitted.

[hyperfine]: https://lib.rs/crates/hyperfine

The `rust_xlsxwriter` library also has an optional compilation "feature" called
`zlib` which will allow it (via ZipWriter) to use compression from a native C
library. This improves the performance on large files significantly and even
beats the C/libxlsxwriter version:


```
$ hyperfine ./rust_perf_test_with_zlib \
            ./c_perf_test              \
            ./rust_perf_test           \
            "python py_perf_test.py"   \
            --warmup 3 --sort command

Benchmark 1: ./rust_perf_test_with_zlib
  Time (mean ± σ):     152.6 ms ±   3.5 ms    [User: 134.3 ms, System: 16.4 ms]
  Range (min … max):   147.0 ms … 158.9 ms    17 runs

Benchmark 2: ./c_perf_test
  Time (mean ± σ):     210.9 ms ±   4.2 ms    [User: 171.9 ms, System: 34.1 ms]
  Range (min … max):   204.1 ms … 219.2 ms    13 runs

Benchmark 3: ./rust_perf_test
  Time (mean ± σ):     240.8 ms ±   4.5 ms    [User: 222.4 ms, System: 16.6 ms]
  Range (min … max):   233.8 ms … 250.9 ms    12 runs

Benchmark 4: python py_perf_test.py
  Time (mean ± σ):     919.1 ms ±  17.0 ms    [User: 870.2 ms, System: 43.4 ms]
  Range (min … max):   885.6 ms … 938.2 ms    10 runs

Relative speed comparison
        1.00          ./rust_perf_test_with_zlib
        1.38 ±  0.04  ./c_perf_test
        1.58 ±  0.05  ./rust_perf_test
        6.02 ±  0.18  python py_perf_test.py
```

This shows that the `rust_xlsxwriter` with `zlib` version is the fastest version
and that it is 1.38 times faster than the C version, 1.58 times faster than the
standard `rust_xlsxwriter` version and 6 times faster than the Python version.

Here is the relative performance of the C, standard `rust_xlsxwriter` and Python versions:

```
$ hyperfine ./c_perf_test             \
            ./rust_perf_test          \
            "python py_perf_test.py"  \
            --warmup 3 --sort command
...

Relative speed comparison
        1.00          ./c_perf_test
        1.14 ±  0.03  ./rust_perf_test
        4.44 ±  0.13  python py_perf_test.py
```

And the relative performance of the `rust_xlsxwriter` and Python versions:

```
$ hyperfine ./rust_perf_test "python py_perf_test.py" --warmup 3 --sort command

...

Relative speed comparison
        1.00          ./rust_perf_test
        3.88 ±  0.10  python py_perf_test.py
```

As with any performance test there are a lot of factors that may affect the
results, however these results are indicative of the relative performance.

The programs used to generate these results are shown below.

## Rust

```rust
use rust_xlsxwriter::{Workbook, XlsxError};

fn main() -> Result<(), XlsxError> {

    let col_max = 50;
    let row_max = 4_000;

    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    for row in 0..row_max {
        for col in 0..col_max {
            if col % 2 == 1 {
                worksheet.write_string(row, col, "Foo")?;
            } else {
                worksheet.write_number(row, col, 12345.0)?;
            }
        }
    }
    workbook.save("rust_perf_test.xlsx")?;

    Ok(())
}
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