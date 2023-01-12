#[derive(Copy, Clone)]
pub enum CategoryType {
    Oval = 1,
    Road = 2,
    DirtOval = 3,
    DirtRoad = 4
}

impl CategoryType {
    pub fn from_string(str: &str) -> Result<Self, &'static str> {
        match str.to_lowercase().as_str() {
            "oval" => Ok(CategoryType::Oval),
            "road" => Ok(CategoryType::Road),
            "dirtoval" => Ok(CategoryType::DirtOval),
            "dirtroad" => Ok(CategoryType::DirtRoad),
            _ => Err("invalid category string")
        }
    }
    pub fn to_db_type(&self) -> i32 {
        return *self as i32;
    }
}