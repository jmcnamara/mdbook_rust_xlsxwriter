# Non US Excel functions and syntax

Excel stores formulas in the format of the US English version, regardless of the
language or locale of the end-user's version of Excel. Therefore all formula
function names written using `rust_xlsxwriter` must be in English. In addition,
formulas must be written with the US style separator/range operator which is a
comma (not semi-colon).

Some examples of how formulas should and shouldn't be written are shown below:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_working_with_formulas_syntax.rs:13:20}}
```
If you have a non-English version of Excel you can use the following
multi-lingual [Formula Translator](http://en.excel-translator.de/language/) to
help you convert the formula. It can also replace semi-colons with commas.

