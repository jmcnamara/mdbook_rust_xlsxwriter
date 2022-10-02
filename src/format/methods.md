# Format methods

The following table shows the Excel format categories, in the order shown in the
Excel “Format Cell” dialog, and the equivalent `rust_xlsxwriter` Format method.
The links will take you to the API docs:

| Category        | Description           |  Method Name                     |
| :-------------- | :-------------------- |  :------------------------------ |
| **Number**      | Numeric format        |  [`set_num_format()`]            |
| **Alignment**   | Horizontal align      |  [`set_align()`]                 |
|                 | Vertical align        |  [`set_align()`]                 |
|                 | Rotation              |  [`set_rotation()`]              |
|                 | Text wrap             |  [`set_text_wrap()`]             |
|                 | Indentation           |  [`set_indent()`]                |
|                 | Reading direction     |  [`set_reading_direction()`]     |
|                 | Shrink to fit         |  [`set_shrink()`]                |
| **Font**        | Font type             |  [`set_font_name()`]             |
|                 | Font size             |  [`set_font_size()`]             |
|                 | Font color            |  [`set_font_color()`]            |
|                 | Bold                  |  [`set_bold()`]                  |
|                 | Italic                |  [`set_italic()`]                |
|                 | Underline             |  [`set_underline()`]             |
|                 | Strikethrough         |  [`set_font_strikethrough()`]    |
|                 | Super/Subscript       |  [`set_font_script()`]           |
| **Border**      | Cell border           |  [`set_border()`]                |
|                 | Bottom border         |  [`set_border_bottom()`]         |
|                 | Top border            |  [`set_border_top()`]            |
|                 | Left border           |  [`set_border_left()`]           |
|                 | Super/Subscript       |  [`set_font_script()`]           |
| **Border**      | Cell border           |  [`set_border()`]                |
|                 | Bottom border         |  [`set_border_bottom()`]         |
|                 | Top border            |  [`set_border_top()`]            |
|                 | Left border           |  [`set_border_left()`]           |
|                 | Right border          |  [`set_border_right()`]          |
|                 | Border color          |  [`set_border_color()`]          |
|                 | Bottom color          |  [`set_border_bottom_color()`]   |
|                 | Top color             |  [`set_border_top_color()`]      |
|                 | Left color            |  [`set_border_left_color()`]     |
|                 | Right color           |  [`set_border_right_color()`]    |
|                 | Diagonal border       |  [`set_border_diagonal()`]       |
|                 | Diagonal border color |  [`set_border_diagonal_color()`] |
|                 | Diagonal border type  |  [`set_border_diagonal_type()`]  |
| **Fill**        | Cell pattern          |  [`set_pattern()`]               |
|                 | Background color      |  [`set_background_color()`]      |
|                 | Foreground color      |  [`set_foreground_color()`]      |
| **Protection**  | Unlock cells          |  [`set_unlocked()`]              |
|                 | Hide formulas         |  [`set_hidden()`]                |

[`set_num_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_num_format
[`set_align()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_align
[`set_align()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_align
[`set_rotation()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_rotation
[`set_text_wrap()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_text_wrap
[`set_indent()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_indent
[`set_reading_direction()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_reading_direction
[`set_shrink()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_shrink
[`set_font_name()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_name
[`set_font_size()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_size
[`set_font_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_color
[`set_bold()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_bold
[`set_italic()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_italic
[`set_underline()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_underline
[`set_font_strikethrough()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_strikethrough
[`set_font_script()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_script
[`set_border()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border
[`set_border_bottom()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_bottom
[`set_border_top()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_top
[`set_border_left()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_left
[`set_font_script()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_font_script
[`set_border()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border
[`set_border_bottom()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_bottom
[`set_border_top()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_top
[`set_border_left()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_left
[`set_border_right()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_right
[`set_border_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_color
[`set_border_bottom_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_bottom_color
[`set_border_top_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_top_color
[`set_border_left_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_left_color
[`set_border_right_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_right_color
[`set_border_diagonal()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_diagonal
[`set_border_diagonal_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_diagonal_color
[`set_border_diagonal_type()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_border_diagonal_type
[`set_pattern()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_pattern
[`set_background_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_background_color
[`set_foreground_color()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_foreground_color
[`set_unlocked()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_unlocked
[`set_hidden()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_hidden
