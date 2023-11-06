use crate::parse;
use crate::types::Line;

use nom::IResult;

// PERSONAL_NAME_PIECES:=
// n NPFX <NAME_PIECE_PREFIX>
// n GIVN <NAME_PIECE_GIVEN>
// n NICK <NAME_PIECE_NICKNAME>
// n SPFX <NAME_PIECE_SURNAME_PREFIX
// n SURN <NAME_PIECE_SURNAME>
// n NSFX <NAME_PIECE_SUFFIX>
// n <<NOTE_STRUCTURE>>
// n <<SOURCE_CITATION>>
#[derive(Debug, Default)]
pub struct Name {
    /// The value of the Name level
    pub value: Option<String>,

    /// GIVN
    /// Given name or earned name. Different given names are separated by a comma.
    pub given: Option<String>,

    /// SURN
    /// Surname or family name. Different surnames are separated by a comma.
    pub surname: Option<String>,

    /// NICK
    /// A descriptive or familiar name used in connection with one's proper name.
    pub nickname: Option<String>,

    /// NPFX
    /// Non indexing name piece that appears preceding the given name and surname
    /// parts. Different name prefix parts are separated by a comma.
    ///
    /// Lt. Cmndr. Joseph /Allen/ jr.
    /// In this example Lt. Cmndr. is considered as the name prefix portion.
    pub prefix: Option<String>,

    /// SPFX
    /// Non-indexing name piece that appears after the given name and surname parts.
    /// Different name suffix parts are separated by a comma.
    ///
    /// For example:
    /// Lt. Cmndr. Joseph /Allen/ jr.
    /// In this example jr. is considered as the name suffix portion.
    pub suffix: Option<String>,

    /// NSFX
    /// Surname prefix or article used in a family name. Different surname articles
    /// are separated by a comma, for example in the name "de la Cruz", this value
    /// would be "de, la".
    pub surname_prefix: Option<String>,

    pub note: Option<crate::types::NoteStructure>,

    // TODO: do we need a specific struct for type or is it just a string?
    pub r#type: Option<String>,
}
impl Name {
    fn parse(record: &str) -> IResult<&str, Name> {
        let mut name = Name {
            value: None,
            given: None,
            surname: None,
            nickname: None,
            prefix: None,
            suffix: None,
            surname_prefix: None,
            note: None,
            r#type: None,
        };

        // We're on level two, so parse until we hit another level two?
        // let min_level: i32 = 2;

        let mut buffer: &str = record;

        // let mut _level: u8;
        // let mut _xref: Option<&str>;
        // let mut tag: Option<&str>;
        // let mut value: Option<&str>;
        let mut line: Line;

        while !buffer.is_empty() {
            (buffer, line) = parse::line(buffer).unwrap();
            // println!("Name::level = {level}, tag = {tag:?}, value={value:?}");

            match line.tag {
                "NAME" => {
                    name.value = Some(line.value.unwrap_or("").to_string());
                }
                "TYPE" => {
                    // type
                    name.r#type = Some(line.value.unwrap_or("").to_string());
                }
                "GIVN" => {
                    name.given = Some(line.value.unwrap_or("").to_string());
                }
                "SURN" => {
                    name.surname = Some(line.value.unwrap_or("").to_string());
                }
                "NICK" => {
                    name.nickname = Some(line.value.unwrap_or("").to_string());
                }
                "NOTE" => {}
                "NPFX" => {
                    name.prefix = Some(line.value.unwrap_or("").to_string());
                }
                "SPFX" => {
                    name.suffix = Some(line.value.unwrap_or("").to_string());
                }
                "NSFX" => {
                    name.surname_prefix = Some(line.value.unwrap_or("").to_string());
                }
                _ => {
                    // println!("Unhandled name tag: {:?}", tag.unwrap());
                }
            }

            // Check if the next line is a new NAME record
            // TODO: a peek_line method so we can check level and tag in one call
            let level = parse::peek_level(buffer).unwrap_or(("", 0_u8)).1;
            // let tag = Some(parse::peek_tag(buffer).unwrap().1);
            let tag = parse::peek_tag(buffer).unwrap().1;

            if level == 1 && tag == "NAME" {
                break;
            }

            if level == 2 && (tag == "ROMN" || tag == "FONE") {
                break;
            }
        }

        Ok((buffer, name))
    }
}

// PERSONAL_NAME_STRUCTURE
// n NAME <NAME_PERSONAL>
// +1 TYPE <NAME_TYPE>
// +1 <<PERSONAL_NAME_PIECES>>
// +1 FONE <NAME_PHONETIC_VARIATION>
// +2 TYPE <PHONETIC_TYPE>
// +2 <<PERSONAL_NAME_PIECES>>
// +1 ROMN <NAME_ROMANIZED_VARIATION>
// +2 TYPE <ROMANIZED_TYPE>
// +2 <<PERSONAL_NAME_PIECES>>
#[derive(Debug, Default)]
pub struct PersonalName {
    /// The surname of an individual, if known, is enclosed between two slash (/)
    /// characters. The order of the name parts should be the order that the person
    /// would, by custom of their culture, have used when giving it to a recorder.
    /// Early versions of Personal Ancestral File ® and other products did not use
    /// the trailing slash when the surname was the last element of the name. If
    /// part of name is illegible, that part is indicated by an ellipsis (...).
    /// Capitalize the name of a person or place in the conventional manner—
    /// capitalize the first letter of each part and lowercase the other letters,
    /// unless conventional usage is otherwise. For example: McMurray.
    ///
    /// Examples:
    /// William Lee (given name only or surname not known)
    /// /Parry/ (surname only)
    /// William Lee /Parry/
    /// William Lee /Mac Parry/ (both parts (Mac and Parry) are surname parts William /Lee/ Parry (surname imbedded in the name string)
    /// William Lee /Pa.../
    ///
    /// TODO: Can there be multiple names? The torture file seems to indicate so
    pub name: Name,

    // pub pieces: PersonalNamePieces,
    pub r#type: Option<String>,

    // The romanized variation of the name is written in the same form prescribed
    /// for the name used in the superior <NAME_PERSONAL> context. The method used
    /// to romanize the name is indicated by the line_value of the subordinate
    /// <ROMANIZED_TYPE>, for example if romaji was used to provide a reading of
    /// a name written in kanji, then the ROMANIZED_TYPE subordinate to the ROMN
    /// tag would indicate romaji.
    pub romanized: Name,

    /// FONE
    /// The phonetic variation of the name is written in the same form as the was
    /// the name used in the superior <NAME_PERSONAL> primitive, but phonetically
    /// written using the method indicated by the subordinate <PHONETIC_TYPE> value,
    /// for example if hiragana was used to provide a reading of a name written
    /// in kanji, then the <PHONETIC_TYPE> value would indicate ‘kana’.
    pub phonetic: Name,
}
// impl AsRef<PersonalNamePieces> for PersonalName {
//     fn as_ref(&self) -> &PersonalNamePieces {
//         &self.pieces
//     }
// }

impl PersonalName {
    pub fn parse(record: &str) -> IResult<&str, PersonalName> {
        let mut pn = PersonalName {
            name: Name {
                value: None,
                given: None,
                surname: None,
                nickname: None,
                prefix: None,
                suffix: None,
                surname_prefix: None,
                note: None,
                r#type: None,
            },
            romanized: Name {
                value: None,
                given: None,
                surname: None,
                nickname: None,
                prefix: None,
                suffix: None,
                surname_prefix: None,
                note: None,
                r#type: None,
            },
            phonetic: Name {
                value: None,
                given: None,
                surname: None,
                nickname: None,
                prefix: None,
                suffix: None,
                surname_prefix: None,
                note: None,
                r#type: None,
            },
            r#type: None,
        };

        // We're on level one, so parse until we hit another level one?
        // let min_level = 1;

        let buffer: &str;

        // TODO: make a Line struct?
        // let mut level: u8 = 0;
        // let mut _xref: Option<&str>;
        // let mut tag: Option<&str>;
        // let mut value: Option<&str> = None;

        // Parse the name out of the record, and switch to a buffer
        (buffer, pn.name) = Name::parse(record).unwrap();

        // level = parse::peek_level(&buffer).unwrap().1;
        // tag = Some(parse::peek_tag(&buffer).unwrap().1);
        // println!("DEBUG: Level {level}, tag {tag:?}");
        let (mut buffer, mut line) = parse::line(buffer).unwrap();

        while line.level > 1 && !buffer.is_empty() {
            if line.level == 2 {
                match line.tag {
                    "ROMN" => {
                        (buffer, pn.romanized) = Name::parse(buffer).unwrap();
                        if let Some(value) = line.value {
                            pn.romanized.value = Some(value.to_string());
                        } else {
                            println!(
                                "Romanized value is missing; Level={}, tag={}",
                                line.level, line.tag
                            );
                        }
                    }
                    "FONE" => {
                        (buffer, pn.phonetic) = Name::parse(buffer).unwrap();
                        if let Some(value) = line.value {
                            pn.phonetic.value = Some(value.to_string());
                        } else {
                            println!(
                                "Phonetic value is missing; Level={}, tag={}",
                                line.level, line.tag
                            );
                        }
                    }
                    _ => {
                        // println!("skipping PersonalName tag {:?}", tag);
                    }
                }
            }
            if line.level == 1 {
                break;
            } else {
                (buffer, line) = parse::line(buffer).unwrap();
            }
        }

        Ok((buffer, pn))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests a possible bug in Ancestry's format, if a line break is embedded within the content of a note
    /// As far as I can tell, it's a \n embedded into the note, at least, from a hex dump of that content.
    fn parse_indi() {
        let data = vec![
            // "0 @I1@ INDI",
            "1 NAME Joseph Tag /Torture/",
            "2 TYPE birth",
            "2 NPFX Prof.",
            "2 GIVN Joseph",
            "2 NICK Joe",
            "2 SPFX Le",
            "2 SURN Torture",
            "2 NSFX Jr.",
            "2 NOTE These are notes about the first NAME structure in this record. These notes are ",
            "3 CONC embedded in the INDIVIDUAL record itself.",
            "3 CONT ",
            "3 CONT This name structure uses all possible tags for a personal name structure.",
            "3 CONT ",
            "3 CONT NOTE: many applications are confused by two NAME structures.",
            "2 SOUR @S1@",
            "3 PAGE 55",
            "3 EVEN BIRT",
            "4 ROLE CHIL",
            "3 DATA",
            "4 DATE 1 JAN 1900",
            "4 TEXT Here is some text from the source specific to this source ",
            "5 CONC citation.",
            "5 CONT Here is more text but on a new line.",
            "3 OBJE @M8@",
            "3 NOTE @N7@",
            "3 QUAY 0",
            "2 FONE Joseph Tag /Torture/",
            "3 TYPE user defined",
            "3 NPFX Prof.",
            "3 GIVN Joseph",
            "3 NICK Joe",
            "3 SPFX Le",
            "3 SURN Torture",
            "3 NSFX Jr.",
            "3 NOTE Phonetisation",
            "3 SOUR @S1@",
            "4 PAGE 55",
            "4 EVEN BIRT",
            "5 ROLE CHIL",
            "4 DATA",
            "5 DATE 1 JAN 1900",
            "5 TEXT Here is some text from the source specific to this source ",
            "6 CONC citation.",
            "6 CONT Here is more text but on a new line.",
            "4 OBJE @M8@",
            "4 NOTE @N7@",
            "4 QUAY 0",
            "2 ROMN Joseph Tag /Torture/",
            "3 TYPE user defined",
            "3 NPFX Prof.",
            "3 GIVN Joseph",
            "3 NICK Joe",
            "3 SPFX Le",
            "3 SURN Torture",
            "3 NSFX Jr.",
            "3 NOTE Romanisation",
            "3 SOUR @S1@",
            "4 PAGE 55",
            "4 EVEN BIRT",
            "5 ROLE CHIL",
            "4 DATA",
            "5 DATE 1 JAN 1900",
            "5 TEXT Here is some text from the source specific to this source ",
            "6 CONC citation.",
            "6 CONT Here is more text but on a new line.",
            "4 OBJE @M8@",
            "4 NOTE @N7@",
            "4 QUAY 0",
            "1 NAME William John /Smith/",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "2 NOTE This is a second personal NAME structure in a single INDIVIDUAL record ",
            "3 CONC which is allowed in GEDCOM.",
            "3 CONT ",
            "3 CONT These notes are embedded in the INDIVIDUAL record.",
            "1 SEX M",
            "1 BIRT",
            "2 TYPE Normal",
            "2 DATE 31 DEC 1965",
            "2 PLAC Salt Lake City, UT, USA",
            "3 FONE Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 ROMN Salt Lake City, UT, USA",
            "4 TYPE user defined",
            "3 MAP",
            "4 LATI N0",
            "4 LONG E0",
            "3 NOTE Place note",
            "2 ADDR",
            "3 ADR1 St. Marks Hospital",
            "3 CITY Salt Lake City",
            "3 STAE UT",
            "3 POST 84121",
            "3 CTRY USA",
            "2 AGNC none",
            "2 RELI Religion",
            "2 CAUS Conception",
            "2 NOTE @N8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 EVEN BIRT",
            "4 ROLE CHIL",
            "3 DATA",
            "4 DATE 1 JAN 1900",
            "4 TEXT Here is some text from the source specific to this source ",
            "5 CONC citation.",
            "5 CONT Here is more text but on a new line.",
            "3 OBJE @M8@",
            "3 NOTE Some notes about this birth source citation which are embedded in the citation ",
            "4 CONC structure itself.",
            "3 QUAY 2",
            "2 OBJE @M15@",
            "2 AGE 0y",
            "2 FAMC @F2@",
            "1 DEAT",
            "2 DATE ABT 15 JAN 2001",
            "2 PLAC New York, New York, USA",
            "3 NOTE The place structure has more detail than usually used for places",
            "2 AGE 76y",
            "2 TYPE slow",
            "2 ADDR",
            "3 ADR1 at Home",
            "2 CAUS Cancer",
            "2 AGNC none",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some death source text.",
            "3 QUAY 3",
            "3 NOTE A death source note.",
            "2 NOTE A death event note.",
            "1 FAMS @F1@",
            "2 NOTE Note about the link to the family record with his first spouse.",
            "2 NOTE Another note about the link to the family record with his first spouse.",
            "1 FAMS @F4@",
            "1 FAMC @F2@",
            "2 NOTE Note about this link to his parents family record.",
            "2 NOTE Another note about this link to his parents family record",
            "1 FAMC @F3@",
            "2 PEDI adopted",
            "2 NOTE Note about the link to his adoptive parents family record.",
            "1 BAPM",
            "2 DATE ABT 31 DEC 1997",
            "2 PLAC The place",
            "2 AGE 3m",
            "2 TYPE BAPM",
            "2 ADDR",
            "3 ADR1 Church Name",
            "3 ADR2 Street Address",
            "3 CITY City Name",
            "3 POST zip",
            "3 CTRY Country",
            "2 CAUS Birth",
            "2 AGNC The Church",
            "2 OBJE @M8@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample baptism Source text.",
            "3 QUAY 3",
            "3 NOTE A baptism source note.",
            "2 NOTE A baptism event note (the event of baptism (not LDS), performed in infancy or later. See also BAPL and CHR).",
            "1 CHR",
            "2 DATE CAL 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CHR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample CHR Source text.",
            "3 QUAY 3",
            "3 NOTE A christening Source note.",
            "2 NOTE Christening event note (the religious event (not LDS) of baptizing and/or naming a ",
            "3 CONC child).",
            "2 FAMC @F3@",
            "1 CHR",
            "2 DATE EST 30 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CHR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some christening source text.",
            "5 CONT This is the second christening structure.",
            "3 QUAY 3",
            "3 NOTE A christening Source note.",
            "2 NOTE Alternative christening event note. GEDOM allows more than one of the same type ",
            "3 CONC of event.",
            "1 BARM",
            "2 DATE AFT 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE BARM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some Bar Mitzvah source text.",
            "3 QUAY 3",
            "3 NOTE A Bar Mitzvah source note.",
            "2 NOTE Bar Mitzvah event note (the ceremonial event held when a Jewish boy reaches age ",
            "3 CONC 13).",
            "1 BASM",
            "2 DATE AFT 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE BASM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some Bas Mitzvah source text.",
            "3 QUAY 3",
            "3 NOTE A Bas Mitzvah source note.",
            "2 NOTE Bas Mitzvah event note (the ceremonial event held when a Jewish girl reaches age 13, ",
            "3 CONC also known as \"Bat Mitzvah\").",
            "1 ADOP",
            "2 DATE BEF 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE ADOP",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some adoption source text.",
            "3 QUAY 3",
            "3 NOTE An adoption source note.",
            "2 NOTE Adoption event note (pertaining to creation of a child-parent relationship that does ",
            "3 CONC not exist biologically).",
            "2 FAMC @F3@",
            "3 ADOP BOTH",
            "1 CHRA",
            "2 DATE BET 31 DEC 1997 AND 1 FEB 1998",
            "2 PLAC The place",
            "2 TYPE CHRA",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some christening source text.",
            "3 QUAY 3",
            "3 NOTE A christening source note.",
            "2 NOTE Adult christening event note (the religious event (not LDS) of baptizing and/or ",
            "3 CONC naming an adult person).",
            "1 CONF",
            "2 DATE BET 31 DEC 1997 AND 2 JAN 1998",
            "2 PLAC The place",
            "2 TYPE CONF",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some CONF Source text.",
            "3 QUAY 3",
            "3 NOTE A CONF Source note.",
            "2 NOTE CONFIRMATION event note (the religious event (not LDS) of conferring the gift of the Holy Ghost and, among protestants, full church membership).",
            "1 FCOM",
            "2 DATE INT 31 DEC 1997 (a test)",
            "2 PLAC The place",
            "2 TYPE FCOM",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some first communion source text.",
            "3 QUAY 3",
            "3 NOTE An first communion source note.",
            "2 NOTE First communion event note (a religious rite, the first act of sharing in the Lord's ",
            "3 CONC supper as part of church worship).",
            "1 GRAD",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE GRAD",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some graduation source text.",
            "3 QUAY 3",
            "3 NOTE A graduation source note.",
            "2 NOTE Graduation event note (an event of awarding educational diplomas or degrees to ",
            "3 CONC individuals).",
            "1 EMIG",
            "2 DATE 1997",
            "2 PLAC The place",
            "2 TYPE EMIG",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some emigration source text.",
            "3 QUAY 3",
            "3 NOTE An emigration source note.",
            "2 NOTE Emigration event note (an event of leaving one's homeland with the intent of residing ",
            "3 CONC elsewhere).",
            "1 IMMI",
            "2 DATE DEC 1997",
            "2 PLAC The place",
            "2 TYPE IMMI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some immigration source text.",
            "3 QUAY 3",
            "3 NOTE An immigration source note.",
            "2 NOTE Immigration event note (an event of entering into a new locality with the intent of ",
            "3 CONC residing there).",
            "1 NATU",
            "2 DATE 1100 BCE",
            "2 PLAC The place",
            "2 TYPE NATU",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some naturalization source text.",
            "3 QUAY 3",
            "3 NOTE A naturalization source note.",
            "2 NOTE Naturalization event note (the event of obtaining citizenship).",
            "1 CENS",
            "2 DATE @#DHEBREW@ 2 TVT 5758",
            "2 PLAC The place",
            "2 TYPE CENS",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some census source text.",
            "3 QUAY 3",
            "3 NOTE A census source note.",
            "2 NOTE Census event note (the event of the periodic count of the population for a designated ",
            "3 CONC locality, such as a national or state Census).",
            "1 RETI",
            "2 DATE @#DFRENCH R@ 11 NIVO 0006",
            "2 PLAC The place",
            "2 TYPE RETI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some retirement source text.",
            "3 QUAY 3",
            "3 NOTE A retirement source note.",
            "2 NOTE Retirement event note (an event of exiting an occupational relationship with an ",
            "3 CONC employer after a qualifying time period).",
            "1 PROB",
            "2 DATE FROM @#DHEBREW@ 25 SVN 5757 TO @#DHEBREW@ 26 IYR 5757",
            "2 PLAC The place",
            "2 TYPE PROB",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some probate source text.",
            "3 QUAY 3",
            "3 NOTE A probate source note.",
            "2 NOTE Probate event note (an event of judicial determination of the validity of a will. May ",
            "3 CONC indicate several related court activities over several dates).",
            "1 BURI",
            "2 DATE @#DFRENCH R@ 5 VEND 0010",
            "2 PLAC The place",
            "2 TYPE BURI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some burial source text.",
            "3 QUAY 3",
            "3 NOTE A burial source note.",
            "2 NOTE Burial event note (the event of the proper disposing of the mortal remains of a ",
            "3 CONC deceased person).",
            "1 WILL",
            "2 DATE INT @#DHEBREW@ 2 TVT 5758 (interpreted)",
            "2 PLAC The place",
            "2 TYPE WILL",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some will source text.",
            "3 QUAY 3",
            "3 NOTE A will source note.",
            "2 NOTE Will event note (a legal document treated as an event, by which a person disposes of ",
            "3 CONC his or her estate, to take effect after death. The event date is the date the will was ",
            "3 CONC signed while the person was alive. See also Probate).",
            "1 CREM",
            "2 DATE AFT 15 JAN 2001",
            "1 EVEN",
            "2 DATE 5 MAY 0005",
            "2 PLAC The place",
            "2 TYPE EVEN",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some generic event source text.",
            "3 QUAY 3",
            "3 NOTE A generic event source note.",
            "2 NOTE Generic event note (a noteworthy happening related to an individual, a group, or an ",
            "3 CONC organization). The TYPE tag specifies the type of event.",
            "1 RESI",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 AGE 35y",
            "2 TYPE RESI",
            "2 ADDR",
            "3 ADR1 Special Address Line 1",
            "3 ADR2 Special Address Line 2",
            "3 ADR3 Special Address Line 3",
            "3 CITY City Name",
            "3 STAE State name",
            "3 POST 0123456789",
            "3 CTRY USA",
            "2 PHON +1-800-555-5555",
            "2 CAUS Needed housing",
            "2 AGNC None",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some residence source text.",
            "3 QUAY 3",
            "3 NOTE A residence source note.",
            "2 NOTE Residence attribute note (the act of dwelling at an address for a period of time).",
            "1 OCCU Occupation",
            "2 DATE 31 DEC 1997",
            "2 AGE 40y",
            "2 PLAC The place",
            "2 TYPE OCCU",
            "2 ADDR",
            "3 ADR1 Work address line 1",
            "3 ADR2 Work address line 2",
            "3 ADR3 Work address line 3",
            "3 CITY Work city",
            "3 STAE Work state",
            "3 POST Work post",
            "3 CTRY Work country",
            "2 CAUS Need for money",
            "2 AGNC Employer",
            "2 OBJE @M7@",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some occupation source text.",
            "3 QUAY 3",
            "3 NOTE An occupation source note.",
            "2 NOTE Occupation attribute note (the type of work or profession of an individual).",
            "1 OCCU Another occupation",
            "2 DATE 31 DEC 1998",
            "2 PLAC The place",
            "2 TYPE OCCU",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some occupation source text.",
            "3 QUAY 3",
            "3 NOTE An occupation source note.",
            "2 NOTE Occupation attribute note. This is the second occupation attribute in the record.",
            "1 EDUC Education",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE EDUC",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some education source text.",
            "3 QUAY 3",
            "3 NOTE An education source note.",
            "2 NOTE Education attribute note (indicator of a level of education attained).",
            "1 DSCR Physical description",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE PHYS",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some physical description source text.",
            "3 QUAY 3",
            "3 NOTE A physical description source note.",
            "2 NOTE Physical description attribute note (the physical characteristics of a person, place, or ",
            "3 CONC thing).",
            "1 RELI Religion",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE RELI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some religion source text.",
            "3 QUAY 3",
            "3 NOTE A religion source note.",
            "2 NOTE Religion attribute note (a religious denomination to which a person is affiliated or for ",
            "3 CONC which a record applies).",
            "1 IDNO 6942",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE IDNO",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some national identification number source text.",
            "3 QUAY 3",
            "3 NOTE An national identification number source note.",
            "2 NOTE National identification number attribute note (a number assigned to identify a person ",
            "3 CONC within some significant external system).",
            "1 PROP Possessions",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE PROP",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some possessions source text.",
            "3 QUAY 3",
            "3 NOTE @N11@",
            "2 NOTE Possessions or property attribute note (pertaining to possessions such as real estate ",
            "3 CONC or other property of interest).",
            "1 CAST Cast name",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE CAST",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some caste name source text.",
            "3 QUAY 3",
            "3 NOTE A caste name source note.",
            "2 NOTE Caste name attribute note (the name of an individual's rank or status in society, based ",
            "3 CONC on racial or religious differences, or differences in wealth, inherited rank, profession, ",
            "3 CONC occupation, etc).",
            "1 NCHI 42",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NCHI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some number of children source text.",
            "3 QUAY 3",
            "3 NOTE Am number of children source note.",
            "2 NOTE Number of children attribute note.",
            "1 NMR 42",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NMR",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some number of marriages source text.",
            "3 QUAY 3",
            "3 NOTE An number of marriages source note.",
            "2 NOTE Number of marriages attribute note.",
            "1 TITL Nobility title",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE TITL",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some title source text.",
            "3 QUAY 3",
            "3 NOTE A title source note.",
            "2 NOTE Title attribute note (a description of a specific writing or other work, such as the title ",
            "3 CONC of a book when used in a source context, or a formal designation used by an ",
            "3 CONC individual in connection with positions of royalty or other social status, ",
            "3 CONT such as Grand Duke).",
            "1 NATI National or tribe origin",
            "2 DATE 31 DEC 1997",
            "2 PLAC The place",
            "2 TYPE NATI",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Some nationality source text.",
            "3 QUAY 3",
            "3 NOTE An nationality source note.",
            "2 NOTE Nationality attribute note (the national heritage of an individual).",
            "1 NOTE @N4@",
            "1 NOTE This is a second set of notes for this single individual record. It is embedded in the ",
            "2 CONC INDIVIDUAL record instead of being in a separate NOTE record.",
            "2 CONT ",
            "2 CONT These notes also have a source citation to a SOURCE record. In GEDCOM ",
            "2 CONC this source can only be a single line and links to a SOURCE record.",
            "1 SOUR @S1@",
            "2 PAGE 42",
            "2 DATA",
            "3 DATE 31 DEC 1900",
            "3 TEXT Some sample text from the first source on this record.",
            "2 QUAY 0",
            "2 NOTE A source note.",
            "1 SOUR @S2@",
            "2 NOTE @N12@",
            "1 OBJE @M7@",
            "1 ASSO @I9@",
            "2 RELA Has multimedia links",
            "2 SOUR @S1@",
            "3 PAGE 42",
            "3 DATA",
            "4 DATE 31 DEC 1900",
            "4 TEXT Sample text about this source on an association.",
            "2 NOTE Note on association link.",
            "1 ASSO @I5@",
            "2 RELA Father",
            "1 REFN 01234567890123456789",
            "2 TYPE reference",
            "1 RIN 12",
            "1 CHAN",
            "2 DATE 12 FEB 2001",
            "3 TIME 19:16:42",
        ];

        let buffer = data.join("\n");
        let (_, name) = PersonalName::parse(&buffer).unwrap();
        // println!("{name:#?}");

        // Check the name.name
        assert_eq!(Some("Joseph Tag /Torture/"), name.name.value.as_deref());
        assert_eq!(Some("Joseph"), name.name.given.as_deref());
        assert_eq!(Some("Torture"), name.name.surname.as_deref());
        assert_eq!(Some("Joe"), name.name.nickname.as_deref());
        assert_eq!(Some("Prof."), name.name.prefix.as_deref());
        assert_eq!(Some("Le"), name.name.suffix.as_deref());
        assert_eq!(Some("Jr."), name.name.surname_prefix.as_deref());
        assert_eq!(Some("birth"), name.name.r#type.as_deref());

        // Check the name.romanized
        assert_eq!(
            Some("Joseph Tag /Torture/"),
            name.romanized.value.as_deref()
        );
        assert_eq!(Some("Joseph"), name.romanized.given.as_deref());
        assert_eq!(Some("Torture"), name.romanized.surname.as_deref());
        assert_eq!(Some("Joe"), name.romanized.nickname.as_deref());
        assert_eq!(Some("Prof."), name.romanized.prefix.as_deref());
        assert_eq!(Some("Le"), name.romanized.suffix.as_deref());
        assert_eq!(Some("Jr."), name.romanized.surname_prefix.as_deref());
        assert_eq!(Some("user defined"), name.romanized.r#type.as_deref());

        // Check the name.phonetic
        assert_eq!(Some("Joseph Tag /Torture/"), name.phonetic.value.as_deref());
        assert_eq!(Some("Joseph"), name.phonetic.given.as_deref());
        assert_eq!(Some("Torture"), name.phonetic.surname.as_deref());
        assert_eq!(Some("Joe"), name.phonetic.nickname.as_deref());
        assert_eq!(Some("Prof."), name.phonetic.prefix.as_deref());
        assert_eq!(Some("Le"), name.phonetic.suffix.as_deref());
        assert_eq!(Some("Jr."), name.phonetic.surname_prefix.as_deref());
        assert_eq!(Some("user defined"), name.phonetic.r#type.as_deref());
    }

    // #[test]
    // fn parse_addr() {
    //     let data = vec![
    //         "3 ADDR",
    //         "4 ADR1 RSAC Software",
    //         "4 ADR2 7108 South Pine Cone Street",
    //         "4 ADR3 Ste 1",
    //         "4 CITY Salt Lake City",
    //         "4 STAE UT",
    //         "4 POST 84121",
    //         "4 CTRY USA",
    //         "3 PHON +1-801-942-7768",
    //         "3 PHON +1-801-555-1212",
    //         "3 PHON +1-801-942-1148",
    //         "3 EMAIL a@@example.com",
    //         "3 EMAIL b@@example.com",
    //         "3 EMAIL c@@example.com",
    //         "3 FAX +1-801-942-7768",
    //         "3 FAX +1-801-555-1212",
    //         "3 FAX +1-801-942-1148",
    //         "3 WWW https://www.example.com",
    //         "3 WWW https://www.example.org",
    //         "3 WWW https://www.example.net",
    //     ];
    //     addr = header::parse_address(data);

    // }
}
