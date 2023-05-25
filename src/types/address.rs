#[derive(Debug, Default)]
pub struct Address {
    pub addr1: Option<String>,
    pub addr2: Option<String>,
    pub addr3: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Vec<String>,
    pub email: Vec<String>,
    pub fax: Vec<String>,
    pub www: Vec<String>,
}
