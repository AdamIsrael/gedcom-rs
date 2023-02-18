use crate::types::{
    Address,
};

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
    pub corporation: Option<String>,
    pub phone: Option<Vec<String>>,
    pub email: Option<Vec<String>>,
    pub fax: Option<Vec<String>>,
    pub www: Option<Vec<String>>,
    pub source: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub address: Option<Address>,
}