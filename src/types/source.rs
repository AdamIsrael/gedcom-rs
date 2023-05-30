use crate::types::corporation::Corporation;

// +1 SOUR <APPROVED_SYSTEM_ID>
//     +2 VERS <VERSION_NUMBER>
//     +2 NAME <NAME_OF_PRODUCT>
//     +2 CORP <NAME_OF_BUSINESS>
//         +3 <<ADDRESS_STRUCTURE>>
//     +2 DATA <NAME_OF_SOURCE_DATA>
//         +3 DATE <PUBLICATION_DATE>
//         +3 COPR <COPYRIGHT_SOURCE_DATA>
//         +4 [CONT|CONC]<COPYRIGHT_SOURCE_DATA>

// 1 SOUR Ancestry.com Family Trees
// 2 NAME Ancestry.com Member Trees
// 2 VERS 2021.07
// 2 _TREE Ambrose Bierce Family Tree
// 3 RIN 116823582
// 3 _ENV prd
// 2 CORP Ancestry.com
// 3 PHON 801-705-7000
// 3 WWW www.ancestry.com
// 3 ADDR 1300 West Traverse Parkway
// 4 CONT Lehi, UT  84043
// 4 CONT USA

#[derive(Debug, Default)]
pub struct Source {
    /// A corporation tag contains the name of the corporation and its address.
    pub corporation: Option<Corporation>,
    pub name: Option<String>,
    pub source: String,
    pub version: Option<String>,
}
