# Development Roadmap

This document tracks the implementation status of GEDCOM 5.5.1 specification features in gedcom-rs.

## Legend

- ✅ Implemented and tested
- 🚧 Partially implemented
- ❌ Not yet implemented

## Implementation Status

### HEADER Record

- [x] HEAD
  - [x] SOUR
    - [x] VERS
    - [x] NAME
    - [x] CORP
      - [x] ADDRESS_STRUCTURE
    - [x] DATA
      - [x] DATE
      - [x] COPR
  - [x] DEST
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
  - [x] SUBN
  - [x] FILE
  - [x] COPR
  - [x] GEDC
    - [x] VERS
    - [x] FORM
  - [x] CHAR
    - [x] VERS
  - [x] LANG
  - [x] PLAC
    - [x] FORM
  - [x] NOTE

### FAM_RECORD (Family Record)

**Status:** 🚧 Recognized but not fully parsed

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

### INDIVIDUAL_RECORD

**Status:** 🚧 Partially implemented

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

### MULTIMEDIA_RECORD

**Status:** ❌ Recognized but not parsed

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

### NOTE_RECORD

**Status:** ❌ Recognized but not parsed

- [ ] NOTE
- [ ] REFN
  - [ ] TYPE
- [ ] RIN
- [ ] SOURCE_CITATION
- [ ] CHANGE_DATE

### REPOSITORY_RECORD

**Status:** ❌ Recognized but not parsed

- [ ] REPO
- [ ] NAME
- [ ] ADDRESS_STRUCTURE
- [ ] NOTE_STRUCTURE
- [ ] REFN
  - [ ] TYPE
- [ ] RIN
- [ ] CHANGE_DATE

### SOURCE_RECORD

**Status:** ❌ Recognized but not parsed

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

### SUBMITTER_RECORD

**Status:** ❌ Not implemented

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

## Priority Features

### High Priority

1. **Full ANSEL Encoding Support**
   - Current: Approximated with Windows-1252
   - Target: Complete ANSEL implementation with prefix diacritics
   - See [docs/ENCODING.md](ENCODING.md) for details

2. **Family Record Parsing (FAM)**
   - Essential for relationship graphs
   - Required for family tree reconstruction

3. **Complete Individual Record Support**
   - Many individual event types partially implemented
   - Need full NOTE_STRUCTURE and SOURCE_CITATION support

### Medium Priority

4. **Source Record Parsing (SOUR)**
   - Important for citation tracking
   - Genealogical research verification

5. **Repository Record Parsing (REPO)**
   - Complements source records
   - Archive location tracking

6. **Multimedia Record Parsing (OBJE)**
   - Photo and document attachments
   - Increasingly common in modern GEDCOM files

### Low Priority

7. **Note Record Parsing (NOTE)**
   - Shared notes across records
   - Less commonly used standalone

8. **Submitter Record Enhancement**
   - Basic support exists
   - Complete implementation needed

## Future Enhancements

### Version 0.2.0 Goals

- [ ] Complete FAM record parsing
- [ ] Full INDIVIDUAL_RECORD implementation
- [ ] Basic SOUR record parsing
- [ ] Improved error messages
- [ ] Performance optimizations for large files

### Version 0.3.0 Goals

- [ ] Full ANSEL encoding support
- [ ] Complete SOUR, REPO, OBJE record parsing
- [ ] Graph API for family relationships
- [ ] GEDCOM 7.0 exploration

### Long-term Vision

- [ ] GEDCOM 7.0 support
- [ ] Write/export capabilities
- [ ] Validation and lint tools
- [ ] Migration tools (GEDCOM 5.5 → 5.5.1 → 7.0)
- [ ] Privacy filtering utilities
- [ ] GEDCOM comparison and merge tools

## How to Contribute

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed contribution guidelines.

Priority areas for contribution:
1. ANSEL encoding implementation
2. Family record parsing
3. Test cases with real-world GEDCOM files
4. Documentation and examples

## References

- [GEDCOM 5.5.1 Specification](https://gedcom.io/specifications/ged551.pdf)
- [ANSI/NISO Z39.47-1993 (ANSEL)](https://www.niso.org/publications/z3947-1993-r2017-ansel)
- [FamilySearch GEDCOM](https://www.familysearch.org/developers/docs/gedcom/)
