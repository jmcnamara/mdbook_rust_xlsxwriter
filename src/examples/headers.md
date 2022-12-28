# Headers and Footers


This program shows several examples of how to set up worksheet headers and footers.


Here are some examples from the code and the relevant Excel output.

Some simple text:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:24}}
```
![Image 1 of output from app_headers_footers.rs](../../images/app_header_example1.png)

An example with variables:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:35}}
```
![Image 2 of output from app_headers_footers.rs](../../images/app_header_example2.png)


An example of setting a header image:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:49:50}}
```
![Image 3 of output from app_headers_footers.rs](../../images/app_header_example3.png)


An example of how to use more than one font:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:62}}
```
![Image 4 of output from app_headers_footers.rs](../../images/app_header_example4.png)

An example of line wrapping:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:72}}
```
![Image 5 of output from app_headers_footers.rs](../../images/app_header_example5.png)

An example of inserting a literal ampersand &:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:81}}
```
![Image 6 of output from app_headers_footers.rs](../../images/app_header_example6.png)

And here is the full code for the example:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/app_headers_footers.rs:7:}}
```