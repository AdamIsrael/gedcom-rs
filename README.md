# gedcom-rs

[![Continuous integration](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/AdamIsrael/rust-gedcom/actions/workflows/ci.yaml)

This is a **work-in-progress** library to parse a [GEDCOM 5.5.1](https://gedcom.io/specifications/ged551.pdf), which is the most commonly used file format for exchanging genealogical data. It is not ready for production.

## TODO

- [ ] HEADER
  - [ ] HEAD
    - [x] SOUR
      - [x] VERS
      - [x] NAME
      - [x] CORP
        - [x] ADDRESS_STRUCTURE
      - [x] DATA
        - [x] DATE
        - [x] COPR
    - [ ] DEST
    - [x] DATE
      - [x] TIME
    - [x] SUBM
      - [x] XREF
      - [x] NAME
      - [x] ADDRESS_STRUCTURE
      - [x] MULTIMEDIA_LINK
      - [x] LANG
      - [x] RFN
      - [x] RIN
      - [x] NOTE_STRUCTURE
      - [x] CHANGE_DATE
    - [ ] SUBN
    - [x] FILE
    - [x] COPR
    - [x] GEDC
      - [x] VERS
      - [x] FORM
    - [X] CHAR
      - [ ] VERS
    - [x] LANG
    - [x] PLAC
      - [ ] FORM
    - [x] NOTE
- [ ] FAM_RECORD
  - [ ] XREF
  - [ ] RESN
  - [ ] FAMILY_EVENT_STRUCTURE
  - [ ] HUSB
  - [ ] WIFE
  - [ ] CHIL
  - [ ] NCHI
  - [ ] SUBM
  - [ ] LDS_SPOUSE_SEALING
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] CHANGE_DATE
  - [ ] NOTE_STRUCTURE
  - [ ] SOURCE_CITATION
  - [ ] MULTIMEDIA_LINK
- [ ] INDIVIDUAL_RECORD
  - [ ] XREF
  - [ ] PERSONAL_NAME_STRUCTURE
  - [ ] SEX
  - [ ] INDIVIDUAL_EVENT_STRUCTURE
  - [ ] INDIVIDUAL_ATTRIBUTE_STRUCTURE
  - [ ] LDS_INDIVIDUAL_ORDINANCE
  - [ ] CHILD_TO_FAMILY_LINK
  - [ ] SPOUSE_TO_FAMILY_LINK
  - [ ] SUBM
  - [ ] ASSOCIATION_STRUCTURE
  - [ ] ALIA
  - [ ] ANCI
  - [ ] DESI
  - [ ] RFN
  - [ ] AFN
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] CHANGE_DATE
  - [ ] NOTE_STRUCTURE
  - [ ] SOURCE_CITATION
  - [ ] MULTIMEDIA_LINK
- [ ] MULTIMEDIA_RECORD
  - [ ] OBJE
  - [ ] FILE
    - [ ] FORM
      - [ ] TYPE
    - [ ] TITL
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] NOTE_STRUCTURE
  - [ ] SOURCE_CITATION
  - [ ] CHANGE_DATE
- [ ] NOTE_RECORD
  - [ ] NOTE
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] SOURCE_CITATION
  - [ ] CHANGE_DATE
- [ ] REPOSITORY_RECORD
  - [ ] REPO
  - [ ] NAME
  - [ ] ADDRESS_STRUCTURE
  - [ ] NOTE_STRUCTURE
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] CHANGE_DATE
- [ ] SOURCE_RECORD
  - [ ] SOUR
  - [ ] DATA
    - [ ] EVEN
      - [ ] DATE
      - [ ] PLAC
    - [ ] AGNC
    - [ ] NOTE_STRUCTURE
  - [ ] AUTH
  - [ ] TITL
  - [ ] ABBR
  - [ ] PUBL
  - [ ] TEXT
  - [ ] SOURCE_REPOSITORY_CITATION
  - [ ] REFN
    - [ ] TYPE
  - [ ] RIN
  - [ ] CHANGE_DATE
  - [ ] NOTE_STRUCTURE
  - [ ] MULTIMEDIA_LINK
- [ ] SUBMITTER_RECORD
  - [ ] SUBN
  - [ ] SUBM
  - [ ] FAMF
  - [ ] TEMP
  - [ ] ANCE
  - [ ] DESC
  - [ ] ORDI
  - [ ] RIN
  - [ ] NOTE_STRUCTURE
  - [ ] CHANGE_DATE

## Copyright

While this library is open source under the MIT license, `data/complete.ged`, used for testing, is © 1997 by H. Eichmann, parts © 1999-2000 by J. A. Nairn.
