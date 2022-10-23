# Page Setup

The sections below look at the sections of the Excel Page Setup dialog and the
equivalent rust_xlsxwriter methods.

For more, general, information about the page setup options in Excel see the Microsoft documentation on [Page Setup].

[Page Setup]: https://support.microsoft.com/en-us/office/page-setup-71c20d94-b13e-48fd-9800-cedd1fec6da3

## Page Setup - Page

The page Setup "Page" dialog looks like this:

![Image of Page Setup Dialog Page section](../images/page_setup01.png)

The equivalent rust_xlsxwriter methods are:

1. [`worksheet.set_portrait()`]
2. [`worksheet.set_landscape()`]
3. [`worksheet.set_print_scale()`]
4. [`worksheet.set_print_fit_to_pages()`]
5. [`worksheet.set_print_first_page_number()`]

[`worksheet.set_portrait()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_portrait
[`worksheet.set_landscape()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_landscape
[`worksheet.set_print_scale()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_scale
[`worksheet.set_print_fit_to_pages()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_fit_to_pages
[`worksheet.set_print_first_page_number()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_first_page_number

Note, for [`worksheet.set_print_fit_to_pages()`] a common requirement is to fit
the printed output to `n` pages wide but have the height be as long as
necessary. To achieve this set the `height` to 0:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_set_print_fit_to_pages.rs:17:18}}
```


## Page Setup - Margins

The page Setup "Margins" dialog looks like this:

![Image of Page Setup Dialog Margins section](../images/page_setup02.png)

The equivalent rust_xlsxwriter methods are:

1. [`worksheet.set_margins()`]
2. [`worksheet.set_print_center_horizontally()`]
3. [`worksheet.set_print_center_vertically()`]

[`worksheet.set_margins()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_margins
[`worksheet.set_print_center_horizontally()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_center_horizontally
[`worksheet.set_print_center_vertically()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_center_vertically


## Page Setup - Header/Footer

The page Setup "Header/Footer" dialog looks like this:

![Image of Page Setup Dialog Header/Footer section](../images/page_setup03.png)

The equivalent rust_xlsxwriter methods are:

1. [`worksheet.set_header()`]
2. [`worksheet.set_footer()`]
3. [`worksheet.set_header_footer_scale_with_doc()`]
4. [`worksheet.set_header_footer_align_with_page()`]

[`worksheet.set_header()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_header
[`worksheet.set_footer()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_footer
[`worksheet.set_header_footer_scale_with_doc()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_header_footer_scale_with_doc
[`worksheet.set_header_footer_align_with_page()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_header_footer_align_with_page

Headers and footers are explained in more detail in the next section on [Adding
Headers and Footers](headers.md).

Note, the options for different first, odd and even pages are not supported in
rust_xlsxwriter.

# Page Setup - Sheet

The page Setup "Sheet" dialog looks like this:

![Image of Page Setup Dialog Sheet section](../images/page_setup04.png)

The equivalent rust_xlsxwriter methods are:

1. [`worksheet.set_print_area()`]
2. [`worksheet.set_repeat_rows()`]
3. [`worksheet.set_repeat_columns()`]
4. [`worksheet.set_print_gridlines()`]
5. [`worksheet.set_print_black_and_white()`]
6. [`worksheet.set_print_draft()`]
7. [`worksheet.set_print_headings()`]
8. [`worksheet.set_page_order()`]

[`worksheet.set_print_area()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_area
[`worksheet.set_repeat_rows()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_repeat_rows
[`worksheet.set_repeat_columns()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_repeat_columns
[`worksheet.set_print_gridlines()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_gridlines
[`worksheet.set_print_black_and_white()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_black_and_white
[`worksheet.set_print_draft()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_draft
[`worksheet.set_print_headings()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_print_headings
[`worksheet.set_page_order()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Worksheet.html#method.set_page_order
