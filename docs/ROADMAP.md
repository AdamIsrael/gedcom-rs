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

**Status:** ✅ Implemented and tested

- [x] XREF
- [x] RESN
- [x] FAMILY_EVENT_STRUCTURE (MARR, ENGA, DIV, ANUL, CENS, DIVF, EVEN)
- [x] HUSB
- [x] WIFE
- [x] CHIL
- [x] NCHI
- [x] SUBM
- [x] LDS_SPOUSE_SEALING
- [x] REFN (supports multiple)
  - [x] TYPE
- [x] RIN
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)
- [x] NOTE_STRUCTURE
- [x] SOURCE_CITATION
- [x] MULTIMEDIA_LINK

### INDIVIDUAL_RECORD

**Status:** ✅ Implemented and tested

- [x] XREF
- [x] PERSONAL_NAME_STRUCTURE (full support including phonetic, romanized variants, source citations)
- [x] SEX
- [x] INDIVIDUAL_EVENT_STRUCTURE (full support - see event details below)
  - [x] BIRT (Birth) - with source citations, media, notes, place, address
  - [x] DEAT (Death) - with source citations, media, notes, place, address
  - [x] CHR (Christening) - with source citations, family link
  - [x] BAPM (Baptism) - with source citations, media, notes
  - [x] BARM (Bar Mitzvah)
  - [x] BASM (Bas Mitzvah)
  - [x] ADOP (Adoption) - with family link and ADOP type (BOTH/HUSB/WIFE)
  - [x] CHRA (Adult Christening)
  - [x] CONF (Confirmation)
  - [x] FCOM (First Communion)
  - [x] GRAD (Graduation)
  - [x] EMIG (Emigration)
  - [x] IMMI (Immigration)
  - [x] NATU (Naturalization)
  - [x] CENS (Census)
  - [x] RETI (Retirement)
  - [x] PROB (Probate)
  - [x] BURI (Burial)
  - [x] WILL (Will)
  - [x] CREM (Cremation)
  - [x] EVEN (Generic events)
- [x] INDIVIDUAL_ATTRIBUTE_STRUCTURE (full support)
  - [x] RESI (Residence) - with address, phone, source citations
  - [x] OCCU (Occupation) - stored as events
  - [x] EDUC (Education) - with TYPE, DATE, PLAC, NOTE, SOUR
  - [x] DSCR (Physical description)
  - [x] RELI (Religion)
  - [x] IDNO (National ID)
  - [x] PROP (Property)
  - [x] CAST (Caste)
  - [x] NCHI (Number of children)
  - [x] NMR (Number of marriages)
  - [x] TITL (Nobility title)
  - [x] NATI (Nationality)
- [ ] LDS_INDIVIDUAL_ORDINANCE - not implemented
- [x] CHILD_TO_FAMILY_LINK (FAMC) - with notes, pedigree (adopted, birth, foster, sealing)
- [x] SPOUSE_TO_FAMILY_LINK (FAMS) - with notes
- [x] SUBM (submitter references)
- [x] ASSOCIATION_STRUCTURE (ASSO) - with RELA, NOTE, SOUR
- [x] ALIA (Alias)
- [x] ANCI (Ancestor interest)
- [x] DESI (Descendant interest)
- [x] RFN (Permanent record file number)
- [x] AFN (Ancestral file number)
- [x] REFN (User reference numbers - supports multiple)
  - [x] TYPE
- [x] RIN (automated record ID)
- [x] RESN (restriction notice)
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)
- [x] NOTE_STRUCTURE (supports multiple notes and note references)
- [x] SOURCE_CITATION (supports multiple citations)
- [x] MULTIMEDIA_LINK (supports multiple object references)

### MULTIMEDIA_RECORD

**Status:** ✅ Implemented and tested

- [x] OBJE (XREF)
- [x] FILE (supports multiple files per record)
  - [x] FORM
    - [x] TYPE
  - [x] TITL
- [x] REFN (supports multiple)
  - [x] TYPE
- [x] RIN
- [x] NOTE_STRUCTURE (supports multiple notes and note references)
- [x] SOURCE_CITATION
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)

### NOTE_RECORD

**Status:** ✅ Implemented and tested

- [x] NOTE (XREF with CONC/CONT support)
- [x] SOURCE_CITATION (supports multiple citations)
- [x] REFN (supports multiple)
  - [x] TYPE
- [x] RIN
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)

### REPOSITORY_RECORD

**Status:** ✅ Implemented and tested

- [x] REPO (XREF)
- [x] NAME
- [x] ADDRESS_STRUCTURE (with phone, email, fax, www support)
- [x] NOTE_STRUCTURE (supports both inline text and note references)
- [x] REFN (supports multiple)
  - [x] TYPE
- [x] RIN
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)

### SOURCE_RECORD

**Status:** ✅ Implemented and tested

- [x] SOUR (XREF)
- [x] DATA
  - [x] EVEN (supports multiple events)
    - [x] DATE
    - [x] PLAC
  - [x] AGNC
  - [x] NOTE_STRUCTURE
- [x] AUTH (with CONC/CONT support)
- [x] TITL (with CONC/CONT support)
- [x] ABBR
- [x] PUBL (with CONC/CONT support)
- [x] TEXT (with CONC/CONT support)
- [x] SOURCE_REPOSITORY_CITATION (REPO with CALN)
- [x] REFN (supports multiple)
  - [x] TYPE
- [x] RIN
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)
- [x] NOTE_STRUCTURE (supports multiple notes)
- [x] MULTIMEDIA_LINK (OBJE references)

### SUBMITTER_RECORD

**Status:** ✅ Implemented and tested

- [x] SUBM (XREF)
- [x] NAME
- [x] ADDRESS_STRUCTURE (with phone, email, fax, www support)
- [x] MULTIMEDIA_LINK (supports multiple)
- [x] LANG (supports up to 3 language preferences)
- [x] RFN (registered RFN)
- [x] RIN (automated record ID)
- [x] NOTE_STRUCTURE (supports multiple notes and note references)
- [x] CHANGE_DATE (full support: DATE, TIME, NOTE)

## Priority Features

### High Priority

1. **Full ANSEL Encoding Support**
   - Current: Approximated with Windows-1252
   - Target: Complete ANSEL implementation with prefix diacritics
   - See [docs/ENCODING.md](ENCODING.md) for details

2. **Complete Family Record Support (FAM)** ✅ Complete
   - All GEDCOM 5.5.1 FAM_RECORD fields implemented
   - Supports RESN, SUBM, LDS_SPOUSE_SEALING, CHANGE_DATE, SOURCE_CITATION, MULTIMEDIA_LINK
   - See examples/family_tree.rs for usage

3. **Complete Individual Record Support** ✅ Complete
   - All GEDCOM 5.5.1 INDIVIDUAL_RECORD fields implemented
   - Full event support: BIRT, DEAT, CHR, BAPM, and 20+ other event types
   - PERSONAL_NAME_STRUCTURE fully implemented with phonetic/romanized variants
   - All attributes: EDUC, DSCR, RELI, IDNO, PROP, CAST, NCHI, NMR, TITL, NATI
   - Family links (FAMC/FAMS) with pedigree support
   - Record-level SOURCE_CITATION, NOTE_STRUCTURE, MULTIMEDIA_LINK
   - Admin fields: REFN, RIN, CHAN, RESN, SUBM
   - Associations and references: ASSO, ALIA, ANCI, DESI, RFN, AFN
   - LDS ordinances not yet implemented (low priority)

4. **Repository Record Parsing (REPO)** ✅ Complete
   - Full GEDCOM 5.5.1 REPOSITORY_RECORD implementation
   - Supports address structure with contact information
   - Note references and inline notes
   - Access via `gedcom.repositories` Vec

5. **Filtering**

Filters should allow for the partial parsing of genealogical data. TBD the extent of filtering capabilities.

   - Implement filtering capabilities for genealogical data
   - Enhance search and query functionalities

6. **Date Parsing**

Dates are a freeform field, with some common conventions. We need to implement parsing and validation, as much as it's possible, to enable searching through events by date.

   - Implement date parsing and validation
   - Support various date formats and ranges
   - Enhance date-related functionalities

### Medium Priority

4. **Source Record Parsing (SOUR)** ✅ Complete
   - Full GEDCOM 5.5.1 SOURCE_RECORD implementation
   - Supports all standard fields including DATA events, repository citations
   - See `src/types/source_record.rs` for implementation details
   - Access via `gedcom.sources` Vec

5. **Multimedia Record Parsing (OBJE)** ✅ Complete
   - Full GEDCOM 5.5.1 MULTIMEDIA_RECORD implementation
   - Supports multiple files per record with format, media type, and title
   - Source citations and reference numbers
   - Access via `gedcom.multimedia` Vec

### Low Priority

7. **Note Record Parsing (NOTE)** ✅ Complete
   - Full GEDCOM 5.5.1 NOTE_RECORD implementation
   - Supports multi-line notes with CONC/CONT
   - Source citations and reference numbers
   - Access via `gedcom.notes` Vec

8. **Submitter Record Enhancement**
   - Basic support exists
   - Complete implementation needed

## Future Enhancements

### Version 0.2.0 Goals

- [x] Core FAM record parsing (XREF, HUSB, WIFE, CHIL, events, notes)
- [x] Complete FAM record parsing (all GEDCOM 5.5.1 fields including RESN, SUBM, LDS, CHANGE_DATE, citations)
- [x] Full INDIVIDUAL_RECORD implementation (all GEDCOM 5.5.1 fields except LDS ordinances)
- [x] Complete SOUR record parsing (all GEDCOM 5.5.1 fields)
- [x] Complete NOTE record parsing (all GEDCOM 5.5.1 fields)
- [x] Complete OBJE record parsing (all GEDCOM 5.5.1 fields)
- [x] Complete REPO record parsing (all GEDCOM 5.5.1 fields)
- [ ] Improved error messages
- [x] Performance optimizations for large files (minimal overhead for new record types)

### Version 0.3.0 Goals

- [ ] Full ANSEL encoding support
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
2. LDS ordinances for INDIVIDUAL_RECORD (low priority feature)
3. Test cases with real-world GEDCOM files
4. Documentation and examples

## References

- [GEDCOM 5.5.1 Specification](https://gedcom.io/specifications/ged551.pdf)
- [ANSI/NISO Z39.47-1993 (ANSEL)](https://www.niso.org/publications/z3947-1993-r2017-ansel)
- [FamilySearch GEDCOM](https://www.familysearch.org/developers/docs/gedcom/)
