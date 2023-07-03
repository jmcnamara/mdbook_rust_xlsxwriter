# Working with Colors


The [`Color`] enum defines Excel colors the can be used throughout the
`rust_xlsxwriter` library.

[`Color`]: https://docs.rs/rust_xlsxwriter/0.11.0/rust_xlsxwriter/enum.Color.html

There are 3 types of colors within the enum:

1. Predefined named RGB colors like `Color::Green`.
2. User defined RGB colors like `Color::RGB(0x4F026A)`.
3. Theme colors like `Color::Theme(9, 4)`.

These are explained in more detail in the sections below.

## Named colors

For convenience there are a number of predefined named colors which are
translated internally to RGB colors. These are shown in the table below.


| Name               | Value      |
| ------------------ | ---------- |
| `Color::Black`     | `0x000000` |
| `Color::Blue`      | `0x0000FF` |
| `Color::Brown`     | `0x800000` |
| `Color::Cyan`      | `0x00FFFF` |
| `Color::Gray`      | `0x808080` |
| `Color::Green`     | `0x008000` |
| `Color::Lime`      | `0x00FF00` |
| `Color::Magenta`   | `0xFF00FF` |
| `Color::Navy`      | `0x000080` |
| `Color::Orange`    | `0xFF6600` |
| `Color::Pink`      | `0xFF00FF` |
| `Color::Purple`    | `0x800080` |
| `Color::Red`       | `0xFF0000` |
| `Color::Silver`    | `0xC0C0C0` |
| `Color::White`     | `0xFFFFFF` |
| `Color::Yellow`    | `0xFFFF00` |
| `Color::Default  ` | None       |
| `Color::Automatic` | None       |

The `Color::Default` value translates to the default color in whatever context
it is used. The `Color::Automatic` value is usually the same but can be
overridden by system settings.

## RGB colors

These are the most common colors used throughout Excel. In `rust_xlsxwriter`
they can be generated using the `Color::RGB()` member of the enum like this:
`Color::RGB(0x4F026A)`.

These are similar to [HTML style colors] which most people will be familiar with.

[HTML style colors]: https://htmlcolors.com

## Theme colors

Theme colors are colors  from the standard palette of 60 colors which is shown
in the image below.

![Image of the Excel theme color palette](../../images/theme_color_palette.png)

The syntax for theme colors in Color is `Theme(color, shade)` where `color` is
one of the 0-9 values on the top row and `shade` is the variant in the
associated column from 0-5. For example "White, background 1" in the top left is
`Theme(0, 0)` and "Orange, Accent 6, Darker 50%" in the bottom right is
`Theme(9, 5)`.

Note, there are no plans to support anything other than the default Excel
"Office" theme.



