# Freeze Panes

An example of setting some "freeze" panes in worksheets to split the worksheet
into scrolling and non-scrolling areas. This is generally used to have one or
more row or column to the top or left of the worksheet area that stays fixed
when a user scrolls.

![Image of output from app_hello_world.rs](../../images/panes.png)

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_panes.rs:7:}}
```