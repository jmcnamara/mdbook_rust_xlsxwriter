# Formulas added in Excel 2010 and later

Excel 2010 and later versions added functions which weren't defined in the
original file specification. These functions are referred to by Microsoft as
"Future Functions". Examples of these functions are `ACOT`, `CHISQ.DIST.RT` ,
`CONFIDENCE.NORM`, `STDEV.P`, `STDEV.S` and `WORKDAY.INTL`.

Although these formulas are displayed as normal in Excel they are stored with a
prefix. For example `STDEV.S(B1:B5)` is stored in the Excel file as
`xlfn.STDEV.S(B1:B5)`. The `rust_xlsxwriter` crate makes these changes
automatically so in general you don't have to worry about this unless you are
dealing with features such as Lambda functions, see above. However, if required
you can manually prefix any required function with the `_xlfn.` prefix.

For completeness the following is a list of future functions taken from [MS XLSX
extensions documentation on future functions].

[MS XLSX extensions documentation on future functions]:
    http://msdn.microsoft.com/en-us/library/dd907480%28v=office.12%29.aspx

Note, the Python in Excel functions aren't simple functions and aren't
supported.

[MS XLSX extensions documentation on future functions]: http://msdn.microsoft.com/en-us/library/dd907480%28v=office.12%29.aspx

| Future Functions                 |
| -------------------------------- |
| `_xlfn.ACOT`                     |
| `_xlfn.ACOTH`                    |
| `_xlfn.AGGREGATE`                |
| `_xlfn.ARABIC`                   |
| `_xlfn.BASE`                     |
| `_xlfn.BETA.DIST`                |
| `_xlfn.BETA.INV`                 |
| `_xlfn.BINOM.DIST`               |
| `_xlfn.BINOM.DIST.RANGE`         |
| `_xlfn.BINOM.INV`                |
| `_xlfn.BITAND`                   |
| `_xlfn.BITLSHIFT`                |
| `_xlfn.BITOR`                    |
| `_xlfn.BITRSHIFT`                |
| `_xlfn.BITXOR`                   |
| `_xlfn.CEILING.MATH`             |
| `_xlfn.CEILING.PRECISE`          |
| `_xlfn.CHISQ.DIST`               |
| `_xlfn.CHISQ.DIST.RT`            |
| `_xlfn.CHISQ.INV`                |
| `_xlfn.CHISQ.INV.RT`             |
| `_xlfn.CHISQ.TEST`               |
| `_xlfn.COMBINA`                  |
| `_xlfn.CONCAT`                   |
| `_xlfn.CONFIDENCE.NORM`          |
| `_xlfn.CONFIDENCE.T`             |
| `_xlfn.COT`                      |
| `_xlfn.COTH`                     |
| `_xlfn.COVARIANCE.P`             |
| `_xlfn.COVARIANCE.S`             |
| `_xlfn.CSC`                      |
| `_xlfn.CSCH`                     |
| `_xlfn.DAYS`                     |
| `_xlfn.DECIMAL`                  |
| `ECMA.CEILING`                   |
| `_xlfn.ERF.PRECISE`              |
| `_xlfn.ERFC.PRECISE`             |
| `_xlfn.EXPON.DIST`               |
| `_xlfn.F.DIST`                   |
| `_xlfn.F.DIST.RT`                |
| `_xlfn.F.INV`                    |
| `_xlfn.F.INV.RT`                 |
| `_xlfn.F.TEST`                   |
| `_xlfn.FILTERXML`                |
| `_xlfn.FLOOR.MATH`               |
| `_xlfn.FLOOR.PRECISE`            |
| `_xlfn.FORECAST.ETS`             |
| `_xlfn.FORECAST.ETS.CONFINT`     |
| `_xlfn.FORECAST.ETS.SEASONALITY` |
| `_xlfn.FORECAST.ETS.STAT`        |
| `_xlfn.FORECAST.LINEAR`          |
| `_xlfn.FORMULATEXT`              |
| `_xlfn.GAMMA`                    |
| `_xlfn.GAMMA.DIST`               |
| `_xlfn.GAMMA.INV`                |
| `_xlfn.GAMMALN.PRECISE`          |
| `_xlfn.GAUSS`                    |
| `_xlfn.HYPGEOM.DIST`             |
| `_xlfn.IFNA`                     |
| `_xlfn.IFS`                      |
| `_xlfn.IMCOSH`                   |
| `_xlfn.IMCOT`                    |
| `_xlfn.IMCSC`                    |
| `_xlfn.IMCSCH`                   |
| `_xlfn.IMSEC`                    |
| `_xlfn.IMSECH`                   |
| `_xlfn.IMSINH`                   |
| `_xlfn.IMTAN`                    |
| `_xlfn.ISFORMULA`                |
| `ISO.CEILING`                    |
| `_xlfn.ISOWEEKNUM`               |
| `_xlfn.LOGNORM.DIST`             |
| `_xlfn.LOGNORM.INV`              |
| `_xlfn.MAXIFS`                   |
| `_xlfn.MINIFS`                   |
| `_xlfn.MODE.MULT`                |
| `_xlfn.MODE.SNGL`                |
| `_xlfn.MUNIT`                    |
| `_xlfn.NEGBINOM.DIST`            |
| `NETWORKDAYS.INTL`               |
| `_xlfn.NORM.DIST`                |
| `_xlfn.NORM.INV`                 |
| `_xlfn.NORM.S.DIST`              |
| `_xlfn.NORM.S.INV`               |
| `_xlfn.NUMBERVALUE`              |
| `_xlfn.PDURATION`                |
| `_xlfn.PERCENTILE.EXC`           |
| `_xlfn.PERCENTILE.INC`           |
| `_xlfn.PERCENTRANK.EXC`          |
| `_xlfn.PERCENTRANK.INC`          |
| `_xlfn.PERMUTATIONA`             |
| `_xlfn.PHI`                      |
| `_xlfn.POISSON.DIST`             |
| `_xlfn.QUARTILE.EXC`             |
| `_xlfn.QUARTILE.INC`             |
| `_xlfn.QUERYSTRING`              |
| `_xlfn.RANK.AVG`                 |
| `_xlfn.RANK.EQ`                  |
| `_xlfn.RRI`                      |
| `_xlfn.SEC`                      |
| `_xlfn.SECH`                     |
| `_xlfn.SHEET`                    |
| `_xlfn.SHEETS`                   |
| `_xlfn.SKEW.P`                   |
| `_xlfn.STDEV.P`                  |
| `_xlfn.STDEV.S`                  |
| `_xlfn.SWITCH`                   |
| `_xlfn.T.DIST`                   |
| `_xlfn.T.DIST.2T`                |
| `_xlfn.T.DIST.RT`                |
| `_xlfn.T.INV`                    |
| `_xlfn.T.INV.2T`                 |
| `_xlfn.T.TEST`                   |
| `_xlfn.TEXTJOIN`                 |
| `_xlfn.UNICHAR`                  |
| `_xlfn.UNICODE`                  |
| `_xlfn.VAR.P`                    |
| `_xlfn.VAR.S`                    |
| `_xlfn.WEBSERVICE`               |
| `_xlfn.WEIBULL.DIST`             |
| `WORKDAY.INTL`                   |
| `_xlfn.XOR`                      |
| `_xlfn.Z.TEST`                   |


The dynamic array functions shown in the [Dynamic Array
support](dynamic_arrays.md) section are also future functions, however the
`rust_xlsxwriter` library automatically adds the required prefixes on the fly so
you don't have to add them explicitly.


| Dynamic Array Functions          |
| -------------------------------- |
| `_xlfn.ANCHORARRAY`              |
| `_xlfn.LAMBDA`                   |
| `_xlfn.RANDARRAY`                |
| `_xlfn.SEQUENCE`                 |
| `_xlfn.SINGLE`                   |
| `_xlfn.SORTBY`                   |
| `_xlfn.UNIQUE`                   |
| `_xlfn.XLOOKUP`                  |
| `_xlfn.XMATCH`                   |
| `_xlfn._xlws.FILTER`             |
| `_xlfn._xlws.SORT`               |

