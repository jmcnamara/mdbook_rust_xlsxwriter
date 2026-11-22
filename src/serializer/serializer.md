# Working with Serde

Serialization is the process of converting data from one format to another
using rules shared between the data and the serializer. The
[Serde](https://serde.rs) crate allows you to attach these types of rules to
data, and it also provides traits that can be implemented by serializers to
convert that data into different formats. The `rust_xlsxwriter` crate
implements the Serde [`serde::Serializer`] trait for [`Worksheet`] structs
which allows you to serialize data directly to a worksheet.

The following sections explain how to serialize Serde-enabled data to an
Excel worksheet using `rust_xlsxwriter`.


Contents:

- [How serialization works in
  `rust_xlsxwriter`](#how-serialization-works-in-rust_xlsxwriter)
- [Setting serialization headers](#setting-serialization-headers)
- [Renaming fields when serializing](#renaming-fields-when-serializing)
- [Skipping fields when serializing](#skipping-fields-when-serializing)
- [Setting serialization formatting](#setting-serialization-formatting)
- [Serializing dates and times](#serializing-dates-and-times)
- [Controlling Excel output via `XlsxSerialize` and struct
  attributes](#controlling-excel-output-via-xlsxserialize-and-struct-attributes)
  - [Container `xlsx` attributes](#container-xlsx-attributes)
  - [Field `xlsx` attributes](#field-xlsx-attributes)
- [Limitations of serializing to
  Excel](#limitations-of-serializing-to-excel)

**Note**: This functionality requires the use of the `serde` feature flag
with `rust_xlsxwriter`:

```bash
cargo add rust_xlsxwriter -F serde
```




## How serialization works in `rust_xlsxwriter`

Serialization with `rust_xlsxwriter` needs to consider that the target
output is a 2D grid of cells into which the data can be serialized. As such,
the focus is on serializing data types that map to this 2D grid, such as
structs or compound collections of structs like vectors or tuples, and it
(currently) ignores compound types like maps.

The image below shows the basic scheme for mapping a struct to a worksheet:
fields are mapped to a header and values are mapped to sequential cells
below the header.

![serialize_intro1](../../images/serialize_intro1.png)

This scheme needs an initial (row, col) location from which to start
serializing to allow the data to be positioned anywhere on the worksheet.
Subsequent serializations will be in the same columns (for the target struct
type) but will be one row lower in the worksheet.

The type name and fields of the struct being serialized is also required
information. We will look at that in more detail in the next section.

Here is an example program that demonstrates the basic steps for serializing
data to an Excel worksheet:

```rust
# // This code is available in examples/doc_worksheet_serialize_intro2.rs
#
use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};
use serde::{Serialize, Deserialize};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Add some formats to use with the serialization data.
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6E0B4");

    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    struct Student<'a> {
        name: &'a str,
        age: u8,
        id: u32,
    }

    let students = [
        Student {
            name: "Aoife",
            age: 25,
            id: 564351,
        },
        Student {
            name: "Caoimhe",
            age: 21,
            id: 443287,
        },
    ];

    // Set up the start location and headers of the data to be serialized.
    worksheet.deserialize_headers_with_format::<Student>(1, 3, &header_format)?;

    // Serialize the data.
    worksheet.serialize(&students)?;

    // Save the file.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
```

Output file:

![serialize_intro2](../../images/serialize_intro2.png)




## Setting serialization headers

When serializing structs `rust_xlsxwriter` needs to know the location where
the serialization starts and also the type and field names of the struct
being serialized. The field names are used as headers, and the type name
allows for several distinct structs to be serialized to the same worksheet.

The worksheet methods that perform this function fall into three types:

1. For structs that only derive [`Serialize`] `rust_xlsxwriter` will use the
   same serialization it uses to write data to figure out the required
   fields and headers. This has the minor disadvantage that it needs an
   instance of the type (and not just the type itself) in order to work.

   The following methods are available for this approach:

   - [`Worksheet::serialize_headers()`]: Requires a concrete instance of the
     type of struct that you wish to serialize. The library uses this to
     infer the struct name and fields.

   - [`Worksheet::serialize_headers_with_format()`]: This is similar to the
     previous method but it allows you to add a cell format for the headers.

   - [`Worksheet::serialize_headers_with_options()`]: Similar to the
     previous methods but also allows configuration of the headers and
     fields via [`SerializeFieldOptions`].

2. For structs that derive [`Serialize`] and [`Deserialize`]
   `rust_xlsxwriter` can use deserialization to figure out the required
   fields and headers. This has the minor advantage that it only needs the
   type and doesn't require an instance of the type.

   The following methods are available for this approach:

   - [`Worksheet::deserialize_headers()`]: This only requires the type of
     struct that you wish to serialize and that it derives the
     [`Deserialize`] and [`Serialize`] traits. The library uses this to
     infer the struct name and fields (via deserialization).

   - [`Worksheet::deserialize_headers_with_format()`]: This is similar to
     the previous method but it allows you to add a cell format for the
     headers.

   - [`Worksheet::deserialize_headers_with_options()`]: Similar to the
     previous methods but also allows configuration of the headers and
     fields via [`SerializeFieldOptions`].

3. For structs that derive [`Serialize`] and [`XlsxSerialize`] (a
   `rust_xlsxwriter` provided trait) the library uses proc macros associated
   with `XlsxSerialize` to figure out the required fields and headers. This
   has the advantage of allowing the user to specify header and field
   formatting via struct attributes. See [Controlling Excel output via
   `XlsxSerialize` and struct
   attributes](#controlling-excel-output-via-xlsxserialize-and-struct-attributes)
   for details on how this works.

   Since the formatting can be encoded in struct attributes this approach
   only requires one worksheet method:

   - [`Worksheet::set_serialize_headers()`]: This only requires the type of
     struct that you wish to serialize and that it derives the
     [`XlsxSerialize`] and [`Serialize`] traits.

The availability of 3 different approaches (and associated methods) will
probably cause mild anxiety in the reader as they will sense that they are
required to make a choice without knowing which is the best approach. In
general you can decide as follows: if your struct derives only have
[`Serialize`] and changing it is outside your control you must use the
serialization method (Approach 1); if you struct also supports
[`Deserialize`] you can save yourself the step of requiring a concrete
instance type and use the deserialization method (Approach 2); finally, if
the definition of the struct is under your control and you can add
[`XlsxSerialize`] then can define the formatting along with the struct and
also save yourself a lot of additional structural code to set up formatting
(Approach 3). The [`XlsxSerialize`] approach does, however,  limit you to
having a fixed set of formatting per struct.

The two examples below should help inform your decision. If not, note that
the author mainly uses [`XlsxSerialize`], but you can use whichever method
works for you without fear of judgment.

The first example below shows the usage of some of the serialization and
deserialization methods.

```rust
# // This code is available in examples/doc_worksheet_serialize_headers3.rs
#
use rust_xlsxwriter::{
    CustomSerializeField, Format, FormatBorder, SerializeFieldOptions, Workbook, XlsxError,
};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Set some column widths for clarity.
    worksheet.set_column_width(2, 4)?;
    worksheet.set_column_width(5, 4)?;
    worksheet.set_column_width(8, 4)?;

    // Add some formats to use with the serialization data.
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6EFCE");

    let currency_format = Format::new().set_num_format("$0.00");

    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    struct Produce {
        fruit: &'static str,
        cost: f64,
    }

    // Create some data instances.
    let item1 = Produce {
        fruit: "Peach",
        cost: 1.05,
    };

    let item2 = Produce {
        fruit: "Plum",
        cost: 0.15,
    };

    let item3 = Produce {
        fruit: "Pear",
        cost: 0.75,
    };

    // 1. Set the serialization location and headers with `deserialize_headers()`
    //    and serialize some data.
    worksheet.deserialize_headers::<Produce>(0, 0)?;

    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // 2. Set the serialization location and formatted headers with
    //    `serialize_headers_with_format()` and serialize some data.
    worksheet.serialize_headers_with_format(0, 3, &item1, &header_format)?;

    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // 3. Set the serialization location and headers with custom headers. We use
    //    the customization to set the header format and also rename the cell format
    //    for the number values.
    let custom_headers = [
        CustomSerializeField::new("fruit").rename("Item"),
        CustomSerializeField::new("cost")
            .rename("Price")
            .set_value_format(currency_format),
    ];
    let header_options = SerializeFieldOptions::new()
        .set_header_format(header_format)
        .set_custom_headers(&custom_headers);

    // Set the serialization location and custom headers.
    worksheet.deserialize_headers_with_options::<Produce>(0, 6, &header_options)?;

    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // 4. Set the serialization location and headers with custom options. We use
    //    the customization to turn off the headers.
    let header_options = SerializeFieldOptions::new().hide_headers(true);

    // Set the serialization location and custom headers.
    worksheet.serialize_headers_with_options(0, 9, &item1, &header_options)?;

    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // Save the file.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
```

Output file:

![worksheet_serialize_headers3](../../images/worksheet_serialize_headers3.png)


The second example uses `XlsxSerialize` and struct attributes to define the
output. Note that there is a lot less structural code in comparison to the
previous examples and that the formatting definitions are inline with the
struct. On the other hand, you should also note that the previous example
where four different outputs are obtained for the same struct isn't possible
with this approach since the format definition is locked to the struct.

```rust
# // This code is available in examples/doc_xlsxserialize_intro.rs
#
use rust_xlsxwriter::{Workbook, XlsxError, XlsxSerialize};
use serde::Serialize;

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Create a serializable struct.
    #[derive(XlsxSerialize, Serialize)]
    #[xlsx(header_format = Format::new().set_bold())]
    struct Produce {
        #[xlsx(rename = "Item")]
        #[xlsx(column_width = 12.0)]
        fruit: &'static str,

        #[xlsx(rename = "Price", num_format = "$0.00")]
        cost: f64,
    }

    // Create some data instances.
    let item1 = Produce {
        fruit: "Peach",
        cost: 1.05,
    };

    let item2 = Produce {
        fruit: "Plum",
        cost: 0.15,
    };

    let item3 = Produce {
        fruit: "Pear",
        cost: 0.75,
    };

    // Set the serialization location and headers.
    worksheet.set_serialize_headers::<Produce>(0, 0)?;

    // Serialize the data.
    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // Save the file to disk.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
```

Output file:

![xlsxserialize_intro](../../images/xlsxserialize_intro.png)




## Renaming fields when serializing

As explained above, serialization converts the field names of structs to
column headers at the top of serialized data. The default field names are
generally `lowercase` and `snake_case` and may not be the way you want the
header names displayed in Excel. In such cases, you can use one of the three
main methods to rename the fields/headers:

1. Rename the field during serialization using the Serde:
   - [field attribute]: `#[serde(rename = "name")` or
   - [container attribute]: `#[serde(rename_all = "...")]`.
2. Rename the header (not field) when setting up custom serialization
   headers via [`Worksheet::deserialize_headers_with_options()`] or
   [`Worksheet::serialize_headers_with_options()`] and
   [`CustomSerializeField::rename()`].
3. Rename the field using the [`XlsxSerialize`] field attribute
   `#[xlsx(rename = "name")`. This is effectively the same as the Serde
   attribute but allows you to specify a different name for the Excel
   serialization. See [Field `xlsx` attributes](#field-xlsx-attributes) for
   details.

[field attribute]: https://serde.rs/field-attrs.html
[container attribute]: https://serde.rs/container-attrs.html

Examples of these methods are shown below.

### Examples of field renaming

The following example demonstrates renaming fields during serialization by
using Serde field attributes.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_serialize_headers_rename1.rs:17:47}}
```

Output file:

![worksheet_serialize_headers_rename1](../../images/worksheet_serialize_headers_rename1.png)


The following example demonstrates renaming fields during serialization by
specifying custom headers and renaming them there. The output is the same as
the image above.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_serialize_headers_rename2.rs:17:51}}
```

The following example demonstrates renaming fields during serialization by
using `XlsxSerialize` `xlsx` field attributes. The output is the same as the
image above.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_xlsxserialize_rename.rs:17:47}}
```




## Skipping fields when serializing

When serializing a struct, you may not want all of the fields to be
serialized. For example, the struct may contain internal fields that aren't
of interest to the end user. There are several ways to skip fields:

1. Using the Serde [field attributes] `#[serde(skip)]`. This is the simplest
   and best method.
2. Explicitly omitting the field by setting up custom serialization headers
   This method is useful when you can't add any additional attributes on the
   struct.
3. Marking the field as skippable via custom headers and the
   [`CustomSerializeField::skip()`] method. This is only required in a few
   edge cases where other methods won't work.
4. Skip the field using the [`XlsxSerialize`] field attribute
   `#[xlsx(skip)]`. This is effectively the same as the Serde attribute but
   allows you to specify a different behavior for the Excel serialization.
   See [Field `xlsx` attributes](#field-xlsx-attributes) for details.

[field attributes]: https://serde.rs/field-attrs.html

Examples of all four methods are shown below.


### Examples of field skipping

The following example demonstrates skipping fields during serialization by
using Serde field attributes. Since the field is no longer used, we also
need to tell `rustc` not to emit a `dead_code` warning.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_serialize_headers_skip1.rs:9:60}}
```

Output file:

![worksheet_serialize_headers_skip1](../../images/worksheet_serialize_headers_skip1.png)



The following example demonstrates skipping fields during serialization by
omitting them from the serialization headers. To do this, we need to specify
custom headers and set [`SerializeFieldOptions::use_custom_headers_only()`].
The output is the same as the image above.

```rust
# // This code is available in examples/doc_worksheet_serialize_headers_skip2.rs
#
# use rust_xlsxwriter::{CustomSerializeField, SerializeFieldOptions, Workbook, XlsxError};
# use serde::{Deserialize, Serialize};
#
# fn main() -> Result<(), XlsxError> {
#     let mut workbook = Workbook::new();
#
#     // Add a worksheet to the workbook.
#     let worksheet = workbook.add_worksheet();
#
    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    struct Produce {
        fruit: &'static str,
        cost: f64,
        in_stock: bool,
    }

    // Create some data instances.
    let item1 = Produce {
        fruit: "Peach",
        cost: 1.05,
        in_stock: true,
    };

    let item2 = Produce {
        fruit: "Plum",
        cost: 0.15,
        in_stock: true,
    };

    let item3 = Produce {
        fruit: "Pear",
        cost: 0.75,
        in_stock: false,
    };

    // Set up only the custom headers we want and omit "in_stock".
    let custom_headers = [
        CustomSerializeField::new("fruit"),
        CustomSerializeField::new("cost"),
    ];

    // Note the use of "use_custom_headers_only" to only serialize the named
    // custom headers.
    let header_options = SerializeFieldOptions::new()
        .use_custom_headers_only(true)
        .set_custom_headers(&custom_headers);

    // Set the serialization location and custom headers.
    worksheet.deserialize_headers_with_options::<Produce>(0, 0, &header_options)?;

    // Serialize the data.
    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;
#
#     // Save the file.
#     workbook.save("serialize.xlsx")?;
#
#     Ok(())
# }
```

The following example is similar in setup to the previous example but
demonstrates skipping fields by explicitly skipping them in the custom
headers. This method should only be required in a few edge cases. The output
is the same as the image above.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_serialize_headers_skip3.rs:44:49}}
```

The following example demonstrates skipping fields during serialization by
using `XlsxSerialize` field attributes. Since the field is no longer used,
we also need to tell `rustc` not to emit a `dead_code` warning.

```rust
# // This code is available in examples/doc_xlsxserialize_skip2.rs
#
# use rust_xlsxwriter::{Workbook, XlsxError, XlsxSerialize};
# use serde::Serialize;
#
# fn main() -> Result<(), XlsxError> {
#     let mut workbook = Workbook::new();
#
#     // Add a worksheet to the workbook.
#     let worksheet = workbook.add_worksheet();
#
    // Create a serializable struct.
    #[derive(XlsxSerialize, Serialize)]
    struct Produce {
        fruit: &'static str,
        cost: f64,

        #[xlsx(skip)]
        #[allow(dead_code)]
        in_stock: bool,
    }
#
#     // Create some data instances.
#     let item1 = Produce {
#         fruit: "Peach",
#         cost: 1.05,
#         in_stock: true,
#     };
#
#     let item2 = Produce {
#         fruit: "Plum",
#         cost: 0.15,
#         in_stock: true,
#     };
#
#     let item3 = Produce {
#         fruit: "Pear",
#         cost: 0.75,
#         in_stock: false,
#     };
#
#     // Set the serialization location and headers.
#     worksheet.set_serialize_headers::<Produce>(0, 0)?;
#
#     // Serialize the data.
#     worksheet.serialize(&item1)?;
#     worksheet.serialize(&item2)?;
#     worksheet.serialize(&item3)?;
#
#     // Save the file to disk.
#     workbook.save("serialize.xlsx")?;
#
#     Ok(())
# }
```

## Setting serialization formatting

Serialization will transfer your data to a worksheet, but it won't format it
without a few additional steps.

The most common requirement is to format the headers/fields at the top of
the serialized data. The simplest way to do this is to use the
[`Worksheet::deserialize_headers_with_format()`] or
[`Worksheet::serialize_headers_with_format()`] methods, as shown in the
[Setting serialization headers](#setting-serialization-headers) section
above. Alternatively, you can use the `#[xlsx(header_format = Format)]`
attribute (see [Container `xlsx` attributes](#container-xlsx-attributes)).

The other common requirement is to format values that are serialized below
the headers, such as numeric data where you need to control the number of
decimal places or make it appear as a currency.

There are a few ways of formatting the values for a field:

- Use [`Worksheet::set_column_format()`] to format the entire column.
- Use [`CustomSerializeField::set_column_format()`] to format the entire
  column. This is the same as the worksheet method but it has the advantage
  of having the column number calculated automatically based on the field
  name.
- Use [`CustomSerializeField::set_value_format()`] to format just the
  serialized data (and not the entire column).
- Using `XlsxSerialize` attributes. This is explained in detail in the
  section below on [Controlling Excel output via `XlsxSerialize` and struct
  attributes](#controlling-excel-output-via-xlsxserialize-and-struct-attributes).

Examples of the first three methods are shown below. Examples of
`XlsxSerialize` attributes are shown in a subsequent section below.

### Examples of formatting

The following example demonstrates serializing instances of a Serde derived
data structure to a worksheet with header and cell formatting.

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_worksheet_serialize_headers_format4.rs:8:67}}
```

Output file:

![worksheet_serialize_headers_custom](../../images/worksheet_serialize_headers_custom.png)

The following variation on the example demonstrates setting formatting via
custom headers. This produces the same output as the previous example but
doesn't require you to manually, or programmatically, calculate the column
number.

```rust
# // This code is available in examples/doc_worksheet_serialize_headers_format5.rs
#
# use rust_xlsxwriter::{
#     CustomSerializeField, Format, FormatBorder, SerializeFieldOptions, Workbook, XlsxError,
# };
# use serde::{Deserialize, Serialize};
#
# fn main() -> Result<(), XlsxError> {
#     let mut workbook = Workbook::new();
#
#     // Add a worksheet to the workbook.
#     let worksheet = workbook.add_worksheet();
#
#     // Add some formats to use with the serialization data.
#     let header_format = Format::new()
#         .set_bold()
#         .set_border(FormatBorder::Thin)
#         .set_background_color("C6EFCE");
#
#     let currency_format = Format::new().set_num_format("$0.00");
#
#     // Create a serializable struct.
#     #[derive(Deserialize, Serialize)]
#     struct Produce {
#         #[serde(rename = "Item")]
#         fruit: &'static str,
#
#         #[serde(rename = "Price")]
#         cost: f64,
#     }
#
#     // Create some data instances.
#     let item1 = Produce {
#         fruit: "Peach",
#         cost: 1.05,
#     };
#
#     let item2 = Produce {
#         fruit: "Plum",
#         cost: 0.15,
#     };
#
#     let item3 = Produce {
#         fruit: "Pear",
#         cost: 0.75,
#     };
#
    // Set up the custom headers.
    let custom_headers =
        [CustomSerializeField::new("Price").set_column_format(currency_format)];

    let header_options = SerializeFieldOptions::new()
        .set_header_format(header_format)
        .set_custom_headers(&custom_headers);

    // Set the serialization location and headers.
    worksheet.deserialize_headers_with_options::<Produce>(1, 1, &header_options)?;
#
#     // Serialize the data.
#     worksheet.serialize(&item1)?;
#     worksheet.serialize(&item2)?;
#     worksheet.serialize(&item3)?;
#
#     // Save the file.
#     workbook.save("serialize.xlsx")?;
#
#     Ok(())
# }
```


The following variation on the example demonstrates setting formatting for
the serialized values, rather than the entire column. This allows a little
bit more precision on cell formatting.

```rust
# // This code is available in examples/doc_worksheet_serialize_headers_format6.rs
#
# use rust_xlsxwriter::{
#     CustomSerializeField, Format, FormatBorder, SerializeFieldOptions, Workbook, XlsxError,
# };
# use serde::{Deserialize, Serialize};
#
# fn main() -> Result<(), XlsxError> {
#     let mut workbook = Workbook::new();
#
#     // Add a worksheet to the workbook.
#     let worksheet = workbook.add_worksheet();
#
#     // Add some formats to use with the serialization data.
#     let header_format = Format::new()
#         .set_bold()
#         .set_border(FormatBorder::Thin)
#         .set_background_color("C6EFCE");
#
#     let currency_format = Format::new().set_num_format("$0.00");
#
#     // Create a serializable struct.
#     #[derive(Deserialize, Serialize)]
#     struct Produce {
#         #[serde(rename = "Item")]
#         fruit: &'static str,
#
#         #[serde(rename = "Price")]
#         cost: f64,
#     }
#
#     // Create some data instances.
#     let item1 = Produce {
#         fruit: "Peach",
#         cost: 1.05,
#     };
#
#     let item2 = Produce {
#         fruit: "Plum",
#         cost: 0.15,
#     };
#
#     let item3 = Produce {
#         fruit: "Pear",
#         cost: 0.75,
#     };
#
    // Set up the custom headers.
    let custom_headers =
        [CustomSerializeField::new("Price").set_value_format(currency_format)];

    let header_options = SerializeFieldOptions::new()
        .set_header_format(header_format)
        .set_custom_headers(&custom_headers);

    // Set the serialization location and headers.
    worksheet.deserialize_headers_with_options::<Produce>(1, 1, &header_options)?;
#
#     // Serialize the data.
#     worksheet.serialize(&item1)?;
#     worksheet.serialize(&item2)?;
#     worksheet.serialize(&item3)?;
#
#     // Save the file.
#     workbook.save("serialize.xlsx")?;
#
#     Ok(())
# }
```




## Serializing dates and times

Dates and times can be serialized to Excel from one of the following types:

- [`ExcelDateTime`]: The inbuilt `rust_xlsxwriter` datetime type.
- [`Chrono`] naive (i.e., timezone unaware) types:
  - [`chrono::NaiveDateTime`]
  - [`chrono::NaiveDate`]
  - [`chrono::NaiveTime`]
- [`Jiff`] civil (i.e., timezone unaware) types:
  - [`jiff::civil::Datetime`]
  - [`jiff::civil::Date`]
  - [`jiff::civil::Time`]

[`ExcelDateTime`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ExcelDateTime.html

[`Chrono`]: https://docs.rs/chrono/latest/chrono
[`chrono::NaiveDate`]:
    https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDate.html
[`chrono::NaiveTime`]:
    https://docs.rs/chrono/latest/chrono/naive/struct.NaiveTime.html
[`chrono::NaiveDateTime`]:
    https://docs.rs/chrono/latest/chrono/naive/struct.NaiveDateTime.html

[`Jiff`]: https://docs.rs/jiff/latest/jiff
[`jiff::civil::Datetime`]:
    https://docs.rs/jiff/latest/jiff/civil/struct.DateTime.html
[`jiff::civil::Date`]:
    https://docs.rs/jiff/latest/jiff/civil/struct.Date.html
[`jiff::civil::Time`]:
    https://docs.rs/jiff/latest/jiff/civil/struct.Time.html

The [`ExcelDateTime`] type is serialized automatically since it implements
the [`Serialize`] trait. The [`Chrono`] and [`Jiff`] types also implement
[`Serialize`] but they will serialize to a string. To serialize them to an
Excel number/datetime format requires a serializing function like
[`Utility::serialize_datetime_to_excel()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/utility/fn.serialize_datetime_to_excel.html)
(as shown in the example below) or
[`Utility::serialize_option_datetime_to_excel()`](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/utility/fn.serialize_option_datetime_to_excel.html).

Excel datetimes also need a number format to display them as a date/time
since they are stored as `f64` values. See [Datetimes in
Excel](https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.ExcelDateTime.html#datetimes-in-excel) and the previous section on
adding formatting.

Note, Excel doesn't use time zones or try to convert or encode time zone
information in any way, so they aren't supported by `rust_xlsxwriter`.

### Examples of serializing dates

The following example demonstrates serializing instances of a Serde derived
data structure with [`ExcelDateTime`] fields.

```rust
# // This code is available in examples/doc_worksheet_serialize_datetime1.rs
#
use rust_xlsxwriter::{
    CustomSerializeField, ExcelDateTime, Format, FormatBorder,
    SerializeFieldOptions, Workbook, XlsxError,
};
use serde::{Deserialize, Serialize};

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Widen the date column for clarity.
    worksheet.set_column_width(1, 12)?;

    // Add some formats to use with the serialization data.
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6E0B4");

    let date_format = Format::new().set_num_format("yyyy/mm/dd");

    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    struct Student<'a> {
        name: &'a str,
        dob: ExcelDateTime,
        id: u32,
    }

    let students = [
        Student {
            name: "Aoife",
            dob: ExcelDateTime::from_ymd(1998, 1, 12)?,
            id: 564351,
        },
        Student {
            name: "Caoimhe",
            dob: ExcelDateTime::from_ymd(2000, 5, 1)?,
            id: 443287,
        },
    ];

    // Set up the start location and headers of the data to be serialized. Note,
    // we need to add a cell format for the datetime data.
    let custom_headers = [
        CustomSerializeField::new("name").rename("Student"),
        CustomSerializeField::new("dob")
            .rename("Birthday")
            .set_value_format(date_format),
        CustomSerializeField::new("id").rename("ID"),
    ];
    let header_options = SerializeFieldOptions::new()
        .set_header_format(header_format)
        .set_custom_headers(&custom_headers);

    worksheet.deserialize_headers_with_options::<Student>(0, 0, &header_options)?;

    // Serialize the data.
    worksheet.serialize(&students)?;

    // Save the file.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
```

Output file:

![worksheet_serialize_datetime1](../../images/worksheet_serialize_datetime1.png)

Here is an example which serializes a struct with a [`chrono::NaiveDate`]
field. The output is the same as the previous example.

```rust
# // This code is available in examples/doc_worksheet_serialize_datetime2.rs
#
use rust_xlsxwriter::utility::serialize_datetime_to_excel;

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Widen the date column for clarity.
    worksheet.set_column_width(1, 12)?;

    // Add some formats to use with the serialization data.
    let header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6E0B4");

    let date_format = Format::new().set_num_format("yyyy/mm/dd");

    // Create a serializable struct.
    #[derive(Deserialize, Serialize)]
    struct Student<'a> {
        name: &'a str,

        // Note, we add a `rust_xlsxwriter` function to serialize the date.
        #[serde(serialize_with = "serialize_datetime_to_excel")]
        dob: NaiveDate,

        id: u32,
    }
#
#     let students = [
#         Student {
#             name: "Aoife",
#             dob: NaiveDate::from_ymd_opt(1998, 1, 12).unwrap(),
#             id: 564351,
#         },
#         Student {
#             name: "Caoimhe",
#             dob: NaiveDate::from_ymd_opt(2000, 5, 1).unwrap(),
#             id: 443287,
#         },
#     ];
#
#     // Set up the start location and headers of the data to be serialized. Note,
#     // we need to add a cell format for the datetime data.
#     let custom_headers = [
#         CustomSerializeField::new("name").rename("Student"),
#         CustomSerializeField::new("dob")
#             .rename("Birthday")
#             .set_value_format(&date_format),
#         CustomSerializeField::new("id").rename("ID"),
#     ];
#     let header_options = SerializeFieldOptions::new()
#         .set_header_format(&header_format)
#         .set_custom_headers(&custom_headers);
#
#     worksheet.deserialize_headers_with_options::<Student>(0, 0, &header_options)?;
#
#     // Serialize the data.
#     worksheet.serialize(&students)?;
#
#     // Save the file.
#     workbook.save("serialize.xlsx")?;

    // ...

    Ok(())
}
```



## Controlling Excel output via `XlsxSerialize` and struct attributes

In the sections above, we saw how to use [Serde
Attributes](https://serde.rs/attributes.html) to control some aspects for
the serialization output.

It is also possible to use the `XlsxSerialize` derive trait and associate
attributes to set `rust_xlsxwriter` specific formatting and options. For
example:


```rust
# // This code is available in examples/doc_xlsxserialize_intro.rs
#
use rust_xlsxwriter::{Workbook, XlsxError, XlsxSerialize};
use serde::Serialize;

fn main() -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Create a serializable struct.
    #[derive(XlsxSerialize, Serialize)]
    #[xlsx(header_format = Format::new().set_bold())]
    struct Produce {
        #[xlsx(rename = "Item")]
        #[xlsx(column_width = 12.0)]
        fruit: &'static str,

        #[xlsx(rename = "Price", num_format = "$0.00")]
        cost: f64,
    }

    // Create some data instances.
    let item1 = Produce {
        fruit: "Peach",
        cost: 1.05,
    };

    let item2 = Produce {
        fruit: "Plum",
        cost: 0.15,
    };

    let item3 = Produce {
        fruit: "Pear",
        cost: 0.75,
    };

    // Set the serialization location and headers.
    worksheet.set_serialize_headers::<Produce>(0, 0)?;

    // Serialize the data.
    worksheet.serialize(&item1)?;
    worksheet.serialize(&item2)?;
    worksheet.serialize(&item3)?;

    // Save the file to disk.
    workbook.save("serialize.xlsx")?;

    Ok(())
}
```

Note the change of column width in Column A, the renamed headers and the
currency format in Column B numbers.

![xlsxserialize_intro](../../images/xlsxserialize_intro.png)

`XlsxSerializer` provides the same formatting functionality as
[`SerializeFieldOptions`] and [`CustomSerializeField`] (in fact it used them
in the background). The advantage of using `XlsxSerializer` is that the
formatting output can be kept with the data format and seen in the same
context. Some users may find this to be a cleaner abstraction.




## The `XlsxSerializer` attributes

The `XlsxSerializer` derived trait adds a wrapper around a Serde
serializable struct to add `rust_xlsxwriter` specific formatting options. It
achieves this via the `rust_xlsxwriter` [`SerializeFieldOptions`] and
[`CustomSerializeField`] serialization configuration structs. It is
available when you enable the `serde` feature.

As in Serde, the [attributes](https://serde.rs/attributes.html) are divided
into "Container Attributes" which apply to the entire struct and "Field
Attributes" which apply to individual fields. In an Excel context, the
container attributes apply to all headers and field attributes apply to the
individual headers and the data below them.

In order to demonstrate each attribute and its effect, we will use
variations of the following example with the relevant attribute applied in
the next sections.

### Base example (without attributes applied)

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_xlsxserialize_base.rs:9:51}}
```

### Container `xlsx` attributes

The following are the "Container" attributes supported by `XlsxSerializer`:

- `#[xlsx(header_format = Format)`

  The `header_format` container attribute sets the [`Format`] property for
  headers. See the [Working with attribute
  Formats](#working-with-attribute-formats) section below for information on
  handling formats.

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        #[xlsx(header_format = Format::new()
               .set_bold()
               .set_border(FormatBorder::Thin)
               .set_background_color("C6EFCE"))]
        struct Produce {
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

![xlsxserialize_header_format](../../images/xlsxserialize_header_format.png)


- `#[xlsx(hide_headers)`

  The `hide_headers` container attribute hides the serialization headers:

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        #[xlsx(hide_headers)]
        struct Produce {
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_hide_headers](../../images/xlsxserialize_hide_headers.png)


- `#[xlsx(table_default)`

  The `table_default` container attribute adds a worksheet [`Table`]
  structure with default formatting to the serialized area:

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        #[xlsx(table_default)]
        struct Produce {
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_table_default](../../images/xlsxserialize_table_default.png)


- `#[xlsx(table_style)`

  The `table_style` container attribute adds a worksheet [`Table`] structure
  with a user specified [`TableStyle`] to the serialized area:

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        #[xlsx(table_style = TableStyle::Medium10)]
        struct Produce {
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_table_style](../../images/xlsxserialize_table_style.png)


- `#[xlsx(table = Table)`

  The `table` container attribute adds a user defined worksheet [`Table`]
  structure to the serialized area:

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        #[xlsx(table = Table::new())]
        struct Produce {
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_table_default](../../images/xlsxserialize_table_default.png)

  See the [Working with attribute Formats](#working-with-attribute-formats)
  section below for information on how to wrap complex objects like
  [`Format`] or [`Table`] in a function so it can be used as an attribute
  parameter.


### Field `xlsx` attributes

The following are the "Field" attributes supported by `XlsxSerializer`:

- `#[xlsx(rename = "")`

  The `rename` field attribute renames the Excel header. This is similar to
  the `#[serde(rename = "")` field attribute except that it doesn't rename
  the field for other types of serialization.

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            #[xlsx(rename = "Item")]
            fruit: &'static str,

            #[xlsx(rename = "Price")]
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_rename](../../images/xlsxserialize_rename.png)


- `#[xlsx(num_format = "")`

  The `num_format` field attribute sets the property to change the number
  formatting of the output. It is a syntactic shortcut for
  [`Format::set_num_format()`].

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            fruit: &'static str,

            #[xlsx(num_format = "$0.00")]
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_num_format](../../images/xlsxserialize_num_format.png)


- `#[xlsx(value_format = Format)`

  The `value_format` field attribute sets the [`Format`] property for the
  fields below the header.

  See the [Working with attribute Formats](#working-with-attribute-formats)
  section below for information on handling formats.

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            #[xlsx(value_format = Format::new().set_font_color("#FF0000"))]
            fruit: &'static str,
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_value_format](../../images/xlsxserialize_value_format.png)


- `#[xlsx(column_format = Format)`

  The `column_format` field attribute is similar to the previous
  `value_format` attribute except that it sets the format for the entire
  column, including any data added manually below the serialized data.

  <br>

- `#[xlsx(header_format = Format)`

  The `header_format` field attribute sets the [`Format`] property for the
  the header. It is similar to the container method of the same name except
  it only sets the format or one header:

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            fruit: &'static str,

            #[xlsx(header_format = Format::new().set_bold().set_font_color("#FF0000"))]
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_field_header_format](../../images/xlsxserialize_field_header_format.png)


- `#[xlsx(column_width = float)`

  The `column_width` field attribute sets the column width in character
  units.

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            fruit: &'static str,
            #[xlsx(column_width = 20.0)]
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_column_width](../../images/xlsxserialize_column_width.png)



- `#[xlsx(column_width_pixels = int)`

  The `column_width_pixels` field attribute field attribute is similar to
  the previous `column_width` attribute except that the width is specified
  in integer pixel units.

  <br>




- `#[xlsx(skip)`

  The `skip` field attribute skips writing the field to the target Excel
  file.  This is similar to the `#[serde(skip)` field attribute except that
  it doesn't skip the field for other types of serialization.

  ```rust
  # use rust_xlsxwriter::XlsxSerialize;
  # use serde::Serialize;
  #
  # fn main() {
        #[derive(XlsxSerialize, Serialize)]
        struct Produce {
            fruit: &'static str,

            #[xlsx(skip)]
            cost: f64,
        }
  # }
  ```

  ![xlsxserialize_skip](../../images/xlsxserialize_skip.png)


Note, if required you can group more than one attribute

```text
#[xlsx(rename = "Item", column_width = 20.0)]
```

### Working with attribute Formats

When working with `XlsxSerialize` attributes that deal with [`Format`]
objects the definition can become quite long for an attribute value:

```rust
# use rust_xlsxwriter::XlsxSerialize;
# use serde::Serialize;
#
# fn main() {
      #[derive(XlsxSerialize, Serialize)]
      #[xlsx(header_format = Format::new()
             .set_bold()
             .set_border(FormatBorder::Thin)
             .set_background_color("C6EFCE"))]
      struct Produce {
          fruit: &'static str,
          cost: f64,
      }
# }
```

<br>

You might, in this case, be tempted to define the format in another part of
your code and use a variable to define the format:

```rust,compile_fail
# use rust_xlsxwriter::{Format, FormatBorder, XlsxSerialize};
# use serde::Serialize;
#
# fn main() {
    let my_header_format = Format::new()
        .set_bold()
        .set_border(FormatBorder::Thin)
        .set_background_color("C6EFCE");

    #[derive(XlsxSerialize, Serialize)]
    #[xlsx(header_format = &my_header_format)] // Won't work.
    struct Produce {
        fruit: &'static str,
        cost: f64,
    }
# }
```

<br>

However, this won't work because Rust derive/proc macros are compiled
statically, and `&header_format` is a dynamic variable.

A workaround for this is to define any formats you wish to use in functions:

```rust
{{#rustdoc_include ../../../rust_xlsxwriter/examples/doc_xlsxserialize_header_format_reuse.rs:17:29}}
```

Output file:

![xlsxserialize_header_format_reuse](../../images/xlsxserialize_header_format_reuse.png)




## Limitations of serializing to Excel

The cell/grid format of Excel sets a physical limitation on what can be
serialized to a worksheet. Unlike other formats such as JSON or XML, you
cannot serialize arbitrary nested data to Excel without making some
concessions to either the format or the contents of the data. When
serializing data to Excel via `rust_xlsxwriter`, it is best to consider what
that data will look like while designing your serialization.

It is only possible to serialize structs or a vector of struct values.
Within a struct most native Rust types are supported (where there is some
mapping to an Excel cell type), as well as the Date/Time types listed above.

Maps are not supported because they don't have a fixed schema that can be
mapped to a 2D grid of cells and the order of the fields is not guaranteed.
As a side effect of this it also not possible to serialize structs that use
the Serde [`flatten`] attribute, since Serde converts the fixed set of named
fields into a dynamically sized map. However, a struct that contains a map
field can be serialized if the field is omitted via one of the [field
skipping](#skipping-fields-when-serializing) methods such as the
`#[serde(skip)]`,`#[xlsx(skip)] or [`CustomSerializeField::skip()`]; or by
excluding it via [`SerializeFieldOptions::use_custom_headers_only()`].

[`flatten`]: https://serde.rs/field-attrs.html#flatten
[Serde data model]: https://serde.rs/data-model.html

Finally, if you hit some serialization limitation using `rust_xlsxwriter`,
remember that there are other non-serialization options available to use in
the standard [`Worksheet`] API to write scalar, vector, and matrix data
types:

- [`Worksheet::write()`]
- [`Worksheet::write_row()`]
- [`Worksheet::write_column()`]
- [`Worksheet::write_row_matrix()`]
- [`Worksheet::write_column_matrix()`]

Magic is great, but the direct approach will also work. Remember Terry
Pratchett's witches.


[`serde::Serializer`]: https://docs.rs/serde/latest/serde/trait.Serializer.html
[`Worksheet`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html
[`Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`SerializeFieldOptions`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.SerializeFieldOptions.html
[`Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[`XlsxSerialize`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/derive.XlsxSerialize.html
[`Worksheet::deserialize_headers_with_options()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.deserialize_headers_with_options
[`Worksheet::serialize_headers_with_options()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.serialize_headers_with_options
[`CustomSerializeField::rename()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.CustomSerializeField.html#method.rename
[`CustomSerializeField::skip()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.CustomSerializeField.html#method.skip
[`SerializeFieldOptions::use_custom_headers_only()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.SerializeFieldOptions.html#method.use_custom_headers_only
[`Worksheet::deserialize_headers_with_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.deserialize_headers_with_format
[`Worksheet::serialize_headers_with_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.serialize_headers_with_format
[`Worksheet::set_column_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.set_column_format
[`CustomSerializeField::set_column_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.CustomSerializeField.html#method.set_column_format
[`CustomSerializeField::set_value_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.CustomSerializeField.html#method.set_value_format
[`CustomSerializeField`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/serializer/struct.CustomSerializeField.html
[`Format`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html
[`Table`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Table.html
[`TableStyle`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/enum.TableStyle.html
[`Format::set_num_format()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/struct.Format.html#method.set_num_format
[`Worksheet::write()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write
[`Worksheet::write_row()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_row
[`Worksheet::write_column()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_column
[`Worksheet::write_row_matrix()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_row_matrix
[`Worksheet::write_column_matrix()`]: https://docs.rs/rust_xlsxwriter/latest/rust_xlsxwriter/worksheet/struct.Worksheet.html#method.write_column_matrix
