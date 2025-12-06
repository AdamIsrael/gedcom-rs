# Character Encoding in GEDCOM Files

This document provides comprehensive technical details about character encoding support in gedcom-rs, with special focus on the ANSEL encoding and its limitations.

## Table of Contents

- [Overview](#overview)
- [Supported Encodings](#supported-encodings)
- [ANSEL Encoding Deep Dive](#ansel-encoding-deep-dive)
- [Technical Implementation](#technical-implementation)
- [Examples and Visual Comparisons](#examples-and-visual-comparisons)
- [Workarounds and Best Practices](#workarounds-and-best-practices)
- [Future Work](#future-work)

## Overview

GEDCOM files can be encoded in various character sets, declared in the header with the `CHAR` tag:

```gedcom
0 HEAD
1 CHAR UTF-8
```

The parser automatically detects the declared encoding by scanning the first ~2KB of the file for the `1 CHAR` tag, then converts the entire file to UTF-8 for internal processing.

## Supported Encodings

### Full Support

| Encoding | GEDCOM Tag | Notes |
|----------|-----------|-------|
| **UTF-8** | `UTF-8`, `UTF8` | Recommended for all new GEDCOM files. Supports all Unicode characters. |
| **ASCII** | `ASCII` | 7-bit ASCII characters (0x00-0x7F). Subset of UTF-8. |
| **Windows-1252** | `ANSI` | Common in Western genealogy software. Superset of ISO-8859-1 with additional characters in 0x80-0x9F range. |

### Partial Support

| Encoding | GEDCOM Tag | Support Level | Issues |
|----------|-----------|---------------|--------|
| **ANSEL** | `ANSEL` | ⚠️ Approximated | See [ANSEL section](#ansel-encoding-deep-dive) below |
| **UTF-16** | `UNICODE` | ⚠️ Untested | May work but not thoroughly tested |

## ANSEL Encoding Deep Dive

### What is ANSEL?

ANSEL (American National Standard for Extended Latin Alphabet Coded Character Set for Bibliographic Use) is defined in **ANSI/NISO Z39.47-1993**. It was designed for bibliographic and genealogical applications to support:

- Extended Latin characters with diacritics
- Special symbols used in genealogy (e.g., ℞ for prescription)
- Multi-lingual text in historical records

### Why ANSEL is Problematic

ANSEL uses a **prefix diacritic** system fundamentally incompatible with modern character encodings:

#### Prefix vs. Suffix Diacritics

**ANSEL (prefix system):**
```
Byte sequence: [0xE2] [0x65]
            ↓       ↓
       circumflex  'e'
Result: ê
```

**Unicode (suffix system):**
```
Byte sequence: [0x65] [0x0302]
            ↓       ↓
           'e'  combining circumflex
Result: ê
```

The diacritic **precedes** the base character in ANSEL but **follows** it in Unicode. This requires:

1. **Stateful parsing**: Track whether the current byte is a diacritic
2. **Character reordering**: Swap diacritic and base character positions
3. **Complete mapping table**: Map all ANSEL diacritics to Unicode combining characters

### ANSEL Character Ranges

ANSEL uses a three-level character set:

| Range | Type | Description |
|-------|------|-------------|
| 0x00-0x7F | **G0** | Standard ASCII |
| 0x80-0xFF | **G1** | Extended characters (when not combining) |
| 0xE0-0xFF | **Combining** | Prefix diacritical marks |

#### ANSEL Combining Diacritics (Partial List)

| ANSEL Byte | Name | Unicode Combining | Example (with 'e') |
|------------|------|-------------------|-------------------|
| 0xE0 | Hook above | U+0309 | ẻ |
| 0xE1 | Grave | U+0300 | è |
| 0xE2 | Acute | U+0301 | é |
| 0xE3 | Circumflex | U+0302 | ê |
| 0xE4 | Tilde | U+0303 | ẽ |
| 0xE5 | Macron | U+0304 | ē |
| 0xE6 | Breve | U+0306 | ĕ |
| 0xE7 | Dot above | U+0307 | ė |
| 0xE8 | Umlaut/Diaeresis | U+0308 | ë |
| 0xE9 | Caron | U+030C | ě |
| 0xEA | Ring above | U+030A | e̊ |
| 0xEB | Ligature left | - | (special) |
| 0xEC | Ligature right | - | (special) |
| 0xED | High comma | U+0315 | e̕ |
| 0xEE | Double acute | U+030B | e̋ |
| 0xEF | Candrabindu | U+0310 | e̐ |
| 0xF0 | Cedilla | U+0327 | ȩ |
| 0xF1 | Right hook | U+0328 | ę |
| 0xF2 | Dot below | U+0323 | ẹ |
| 0xF3 | Double dot below | U+0324 | e̤ |
| 0xF4 | Ring below | U+0325 | e̥ |
| 0xF5 | Double underscore | U+0333 | e̳ |
| 0xF6 | Underscore | U+0332 | e̲ |
| 0xF7 | Left hook | U+0326 | e̦ |
| 0xF8 | Right cedilla | U+0327 | ȩ |
| 0xF9 | Upadhmaniya | U+0310 | e̐ |
| 0xFA | Double tilde left | - | (special) |
| 0xFB | Double tilde right | - | (special) |
| 0xFE | High comma off center | - | (special) |

#### ANSEL Non-Combining Characters (Partial List)

| ANSEL Byte | Character | Description | Unicode |
|------------|-----------|-------------|---------|
| 0xA1 | Ł | L with stroke | U+0141 |
| 0xA2 | Ø | O with stroke | U+00D8 |
| 0xA3 | Đ | D with stroke | U+0110 |
| 0xA4 | Þ | Thorn | U+00DE |
| 0xA5 | Æ | AE ligature | U+00C6 |
| 0xA6 | Œ | OE ligature | U+0152 |
| 0xA7 | ʹ | Soft sign | U+02B9 |
| 0xA8 | · | Middle dot | U+00B7 |
| 0xA9 | ♭ | Music flat | U+266D |
| 0xAA | ® | Patent mark | U+00AE |
| 0xAB | ± | Plus or minus | U+00B1 |
| 0xAC | Ơ | O-hook | U+01A0 |
| 0xAD | Ư | U-hook | U+01AF |
| 0xAE | ʼ | Alif | U+02BC |
| 0xB0 | ʻ | Ayn | U+02BB |
| 0xB1 | ł | l with stroke | U+0142 |
| 0xB2 | ø | o with stroke | U+00F8 |
| 0xB3 | đ | d with stroke | U+0111 |
| 0xB4 | þ | thorn | U+00FE |
| 0xB5 | æ | ae ligature | U+00E6 |
| 0xB6 | œ | oe ligature | U+0153 |
| 0xB7 | ʺ | Hard sign | U+02BA |
| 0xB8 | ı | dotless i | U+0131 |
| 0xB9 | £ | British pound | U+00A3 |
| 0xBA | ð | eth | U+00F0 |

### Current Implementation (Windows-1252 Approximation)

The library currently maps `ANSEL` → `Windows-1252`, which provides:

**✅ Works correctly for:**
- Basic ASCII (0x00-0x7F)
- Some Western European characters that happen to align
- Basic punctuation

**❌ Fails for:**
- **All combining diacritics** (0xE0-0xFF) - interpreted as Windows-1252 control/symbol characters instead
- **Special genealogical symbols** - mapped to wrong characters
- **Character ordering** - no reordering performed

### Visual Examples of Incorrect Conversions

#### Example 1: French Name with Acute Accent

**Intended (correct ANSEL):**
```
Name: André Beauché
ANSEL bytes: [0x41 0x6E 0x64 0x72 0xE2 0x65] [0x42 0x65 0x61 0x75 0x63 0x68 0xE2 0x65]
             [A    n    d    r    á    e  ] [B    e    a    u    c    h    é    e  ]
                               ^diacritic^                                ^diacritic^
```

**Current output (Windows-1252 interpretation):**
```
Name: Andrâe Beauchâe
           ^^          ^^
         Wrong!      Wrong!
```

The byte `0xE2` is interpreted as **â** (a-circumflex) in Windows-1252, not as "acute accent to be combined with next character."

#### Example 2: German Name with Umlaut

**Intended (correct ANSEL):**
```
Name: Müller
ANSEL bytes: [0x4D 0xE8 0x75 0x6C 0x6C 0x65 0x72]
             [M    ¨    u    l    l    e    r  ]
                   ^diacritic (umlaut)
```

**Current output (Windows-1252 interpretation):**
```
Name: Mèuller
        ^^
      Wrong!
```

The byte `0xE8` is interpreted as **è** (e-grave) in Windows-1252, not as "umlaut to be combined with next character."

#### Example 3: Polish Name with Stroke

**Intended (correct ANSEL):**
```
Name: Łukasz
ANSEL bytes: [0xA1 0x75 0x6B 0x61 0x73 0x7A]
             [Ł    u    k    a    s    z  ]
```

**Current output (Windows-1252 interpretation):**
```
Name: ¡ukasz
      ^^
    Wrong!
```

The byte `0xA1` is **¡** (inverted exclamation) in Windows-1252, not **Ł** (L-stroke).

## Technical Implementation

### Encoding Detection

The parser scans the first 2KB of the file (as ASCII) to find:

```gedcom
1 CHAR ANSEL
```

This works because GEDCOM header tags are always ASCII-compatible regardless of the file's encoding.

**Code location:** `src/parse.rs:detect_gedcom_encoding()`

### Encoding Conversion

Once detected, the entire file is converted to UTF-8 using the `encoding_rs` crate:

```rust
let (encoding, encoding_name) = detect_gedcom_encoding(&bytes);
let (cow, _encoding_used, had_errors) = encoding.decode(&bytes);
```

**Code location:** `src/parse.rs:read_file_with_encoding()`

### Mapping Table

| GEDCOM CHAR Tag | Rust Encoding | Notes |
|----------------|---------------|-------|
| `UTF-8`, `UTF8` | `encoding_rs::UTF_8` | Direct mapping |
| `ASCII` | `encoding_rs::UTF_8` | ASCII is subset |
| `ANSI` | `encoding_rs::WINDOWS_1252` | Direct mapping |
| `ANSEL` | `encoding_rs::WINDOWS_1252` | **Approximation** |
| `UNICODE` | `encoding_rs::UTF_16LE` | Untested |
| (unknown) | `encoding_rs::WINDOWS_1252` | Default fallback |

## Examples and Visual Comparisons

### Real-World GEDCOM File Test: TGC551LF.ged

This test file uses ANSEL encoding with a copyright symbol:

**Header:**
```gedcom
0 HEAD
1 CHAR ANSEL
1 COPR Copyright Ò 1997-2000 by H. Eichmann.
              ^^^ This is 0xC3 in ANSEL
```

**Byte Analysis:**
```
Offset  Hex      ANSEL        Windows-1252   UTF-8 (correct)
------  ----     -----------  -------------  ---------------
...
0x0035  0xC3     © (copyright) Ã             ©
...
```

**Before fix:** File failed to parse (invalid UTF-8)  
**After fix (current):** Parses but displays `Ã` instead of `©`  
**Correct ANSEL:** Should display `©`

## Workarounds and Best Practices

### For Users

1. **Convert to UTF-8**: If you control the GEDCOM file, use a genealogy application to export/save as UTF-8
2. **Use verbose mode**: Run with `--verbose` flag to see detailed encoding warnings
3. **Verify data**: After parsing ANSEL files, manually check names with accents
4. **Report issues**: Help improve the library by reporting specific encoding problems

### For Contributors

1. **Test files**: Always test with ANSEL-encoded files from `data/` directory
2. **Character tables**: Reference the complete ANSEL specification (ANSI/NISO Z39.47-1993)
3. **Stateful parser**: Proper ANSEL support requires a stateful byte-level parser
4. **Unicode normalization**: After conversion, apply Unicode normalization (NFC)

### Converting ANSEL Files to UTF-8

**Option 1: Using Gramps (Free, Open Source)**
```
1. Import GEDCOM file into Gramps
2. Export as GEDCOM
3. Select "UTF-8" as character encoding
```

**Option 2: Using RootsMagic**
```
1. Open GEDCOM file
2. File > Export
3. Select UTF-8 encoding option
```

**Option 3: Using gedcom-rs (future)**
```bash
# Not yet implemented
gedcom-rs convert --from ANSEL --to UTF-8 input.ged output.ged
```

## Future Work

### Full ANSEL Support Implementation Plan

**Tracking Issue:** [#TBD](https://github.com/adamgiacomelli/gedcom-rs/issues/TBD)

**Phase 1: Research & Specification**
- [ ] Obtain complete ANSEL specification (ANSI/NISO Z39.47-1993)
- [ ] Create comprehensive character mapping table (ANSEL → Unicode)
- [ ] Collect test files with all ANSEL characters
- [ ] Document edge cases and ambiguities

**Phase 2: Core Implementation**
- [ ] Implement stateful byte-level parser for ANSEL
- [ ] Handle prefix diacritics with character reordering
- [ ] Implement combining character logic
- [ ] Add special handling for ligatures (0xEB, 0xEC)
- [ ] Support multi-byte sequences

**Phase 3: Testing & Validation**
- [ ] Unit tests for each ANSEL character
- [ ] Integration tests with real GEDCOM files
- [ ] Visual verification of rendered output
- [ ] Benchmark performance impact

**Phase 4: Polish**
- [ ] Optimize performance
- [ ] Add comprehensive error messages
- [ ] Document ANSEL-specific API
- [ ] Add conversion utility (ANSEL → UTF-8)

### Alternative Approaches

**Option A: External Library**
- Look for existing Rust crates that handle ANSEL
- Pros: Less maintenance
- Cons: None currently exist

**Option B: Pre-processing Tool**
- Create separate tool to convert ANSEL → UTF-8
- Pros: Simpler parser logic
- Cons: Extra step for users

**Option C: Current Approximation + Warnings**
- Keep Windows-1252 approximation
- Improve warning messages
- Pros: Simple, works for many cases
- Cons: Incorrect for accented text (current approach)

## References

- [ANSI/NISO Z39.47-1993 Standard](https://www.niso.org/publications/z3947-1993-r2017-ansel)
- [GEDCOM 5.5.1 Specification](https://gedcom.io/specifications/ged551.pdf) - Section on CHARACTER_SET
- [Unicode Normalization](https://unicode.org/reports/tr15/)
- [encoding_rs Documentation](https://docs.rs/encoding_rs/)

## Contributing

If you have expertise in character encoding or access to the full ANSEL specification, your contributions would be greatly appreciated! See [CONTRIBUTING.md](../CONTRIBUTING.md) for details.
