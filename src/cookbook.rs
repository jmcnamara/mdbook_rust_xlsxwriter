/*!

A cookbook of example programs using `rust_xlsxwriter`.


The examples below can be run as follows:

```bash
git clone git@github.com:jmcnamara/rust_xlsxwriter.git
cd rust_xlsxwriter/
cargo run --example app_demo  # or any other example
```

# Contents

1. [Hello World: Simple getting started example](#hello-world-simple-getting-started-example)
2. [Feature demo: Demonstrates more features of the library](#feature-demo-demonstrates-more-features-of-the-library)
3. [Cell formatting: Demonstrates various formatting options](#cell-formatting-demonstrates-various-formatting-options)
4. [Format colors: Create a palette of the available colors](#format-colors-create-a-palette-of-the-available-colors)
5. [Merging cells: An example of merging cell ranges](#merging-cells-an-example-of-merging-cell-ranges)
6. [Autofilters: Add an autofilter to a worksheet](#autofilters-add-an-autofilter-to-a-worksheet)
7. [Tables: Adding worksheet tables](#tables-adding-worksheet-tables)
8. [Conditional Formatting: Adding conditional formatting to worksheets](#conditional-formatting-adding-conditional-formatting-to-worksheets)
9. [Data Validation: Add cell validation and dropdowns](#data-validation-add-cell-validation-and-dropdowns)
10. [Notes: Adding notes to worksheet cells](#notes-adding-notes-to-worksheet-cells)
11. [Rich strings: Add multi-font rich strings to a worksheet](#rich-strings-add-multi-font-rich-strings-to-a-worksheet)
12. [Right to left display: Set a worksheet into right to left display mode](#right-to-left-display-set-a-worksheet-into-right-to-left-display-mode)
13. [Autofitting Columns: Example of autofitting column widths](#autofitting-columns-example-of-autofitting-column-widths)
14. [Insert images: Add images to a worksheet](#insert-images-add-images-to-a-worksheet)
15. [Insert images: Embedding an image in a cell](#insert-images-embedding-an-image-in-a-cell)
16. [Insert images: Inserting images to fit a cell](#insert-images-inserting-images-to-fit-a-cell)
17. [Adding a watermark: Adding a watermark to a worksheet by adding an image to the header](#adding-a-watermark-adding-a-watermark-to-a-worksheet-by-adding-an-image-to-the-header)
18. [Adding a watermark: Adding a watermark to a worksheet by adding a background image](#adding-a-watermark-adding-a-watermark-to-a-worksheet-by-adding-a-background-image)
19. [Chart: Simple: Simple getting started chart example](#chart-simple-simple-getting-started-chart-example)
20. [Chart: Area: Excel Area chart example](#chart-area-excel-area-chart-example)
21. [Chart: Bar: Excel Bar chart example](#chart-bar-excel-bar-chart-example)
22. [Chart: Column: Excel Column chart example](#chart-column-excel-column-chart-example)
23. [Chart: Line: Excel Line chart example](#chart-line-excel-line-chart-example)
24. [Chart: Scatter: Excel Scatter chart example](#chart-scatter-excel-scatter-chart-example)
25. [Chart: Pie: Excel Pie chart example](#chart-pie-excel-pie-chart-example)
26. [Chart: Doughnut: Excel Doughnut chart example](#chart-doughnut-excel-doughnut-chart-example)
27. [Chart: Radar: Excel Radar chart example](#chart-radar-excel-radar-chart-example)
28. [Chart: Stock: Excel Stock chart example](#chart-stock-excel-stock-chart-example)
29. [Chart: Using a secondary axis](#chart-using-a-secondary-axis)
30. [Chart: Create a combined chart](#chart-create-a-combined-chart)
31. [Chart: Create a combined pareto chart](#chart-create-a-combined-pareto-chart)
32. [Chart: Pattern Fill: Example of a chart with Pattern Fill](#chart-pattern-fill-example-of-a-chart-with-pattern-fill)
33. [Chart: Gradient Fill: Example of a chart with Gradient Fill](#chart-gradient-fill-example-of-a-chart-with-gradient-fill)
34. [Chart: Styles: Example of setting default chart styles](#chart-styles-example-of-setting-default-chart-styles)
35. [Chart: Chart data table](#chart-chart-data-table)
36. [Chart: Chart data tools](#chart-chart-data-tools)
37. [Chart: Gauge Chart](#chart-gauge-chart)
38. [Chart: Chartsheet](#chart-chartsheet)
39. [Grouped Rows: Create a grouped row outline](#grouped-rows-create-a-grouped-row-outline)
40. [Grouped Columns: Create a grouped column outline](#grouped-columns-create-a-grouped-column-outline)
41. [Textbox: Inserting Checkboxes in worksheets](#textbox-inserting-checkboxes-in-worksheets)
42. [Textbox: Inserting Textboxes in worksheets](#textbox-inserting-textboxes-in-worksheets)
43. [Textbox: Ignore Excel cell errors](#textbox-ignore-excel-cell-errors)
44. [Sparklines: simple example](#sparklines-simple-example)
45. [Sparklines: advanced example](#sparklines-advanced-example)
46. [Traits: Extending generic `write()` to handle user data types](#traits-extending-generic-write-to-handle-user-data-types)
47. [Macros: Adding macros to a workbook](#macros-adding-macros-to-a-workbook)
48. [Defined names: using user defined variable names in worksheets](#defined-names-using-user-defined-variable-names-in-worksheets)
49. [Cell Protection: Setting cell protection in a worksheet](#cell-protection-setting-cell-protection-in-a-worksheet)
50. [Document Properties: Setting document metadata properties for a workbook](#document-properties-setting-document-metadata-properties-for-a-workbook)
51. [Document Properties: Setting the Sensitivity Label](#document-properties-setting-the-sensitivity-label)
52. [Internal links: Creating a Table of Contents](#internal-links-creating-a-table-of-contents)
53. [Headers and Footers: Shows how to set headers and footers](#headers-and-footers-shows-how-to-set-headers-and-footers)
54. [Hyperlinks: Add hyperlinks to a worksheet](#hyperlinks-add-hyperlinks-to-a-worksheet)
55. [Freeze Panes: Example of setting freeze panes in worksheets](#freeze-panes-example-of-setting-freeze-panes-in-worksheets)
56. [Dynamic array formulas: Examples of dynamic arrays and formulas](#dynamic-array-formulas-examples-of-dynamic-arrays-and-formulas)
57. [Excel `LAMBDA()` function: Example of using the Excel 365 `LAMBDA()` function](#excel-lambda-function-example-of-using-the-excel-365-lambda-function)


# Hello World: Simple getting started example

Program to create a simple Hello World style Excel spreadsheet using the
`rust_xlsxwriter` library.

**Image of the output file:**


<img src="https://rustxlsxwriter.github.io/images/hello.png">

**Code to generate the output file:**

