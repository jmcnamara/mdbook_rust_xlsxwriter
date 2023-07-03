# Freeze Panes: Example of setting freeze panes in worksheets

An example of setting some "freeze" panes in worksheets to split the worksheet
into scrolling and non-scrolling areas. This is generally used to have one or
more row or column to the top or left of the worksheet area that stays fixed
when a user scrolls.

**Image of the output file:**

![Image of output from app_hello_world.rs](../../images/panes.png)

**Code to generate the output file:**

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_panes.rs:8:}}
```