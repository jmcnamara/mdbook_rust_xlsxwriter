# Custom UI: Adding a custom ribbon tab

An example of adding a custom ribbon tab to a `rust_xlsxwriter` file using a
Custom UI XML file.

The custom tab contains built-in Excel commands and a button that runs a macro
from a VBA project file. See [Working with Custom UI
elements/Ribbons](../custom_ui/custom_ui.md) for more details.

**Image of the output file:**

![Image of output from app_custom_ui.rs](../../images/app_custom_ui.png)

```rust
{{#include ../../../rust_xlsxwriter/examples/app_custom_ui.rs:11:}}
```
