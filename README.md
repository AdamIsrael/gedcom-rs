# rust-gedcom

[![Continuous integration](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml)

This is a work-in-progress library to parse a [GEDCOM 5.5.1](https://gedcom.io/specifications/ged551.pdf), which is the most commonly used file format for exchanging genealogical data.

In this attempt, I am continuing to learn Rust, and learning ~~[nom](https://docs.rs/nom/latest/nom/)~~ [winnow](https://github.com/winnow-rs/winnow), to parse a gedcom into a structured data source which can then be serialized, manipulated, searched, etc.

## Notes

I've got enough of the GEDCOM parsing that I should write a complete test suite, so I know exactly what is missing.

## Copyright

While this library is open source (license tbd), `data/complete.ged`, used for testing, is © 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.