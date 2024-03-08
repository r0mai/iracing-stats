#[derive(Copy, Clone)]
pub enum CategoryType {
    Oval = 1,
    Road = 2,
    DirtOval = 3,
    DirtRoad = 4,
    SportsCar = 5,
    FormulaCar = 6,
}

impl CategoryType {
    pub fn from_i32(i: i32) -> Result<Self, &'static str> {
        return match i {
            1 => Ok(CategoryType::Oval),
            2 => Ok(CategoryType::Road),
            3 => Ok(CategoryType::DirtOval),
            4 => Ok(CategoryType::DirtRoad),
            5 => Ok(CategoryType::SportsCar),
            6 => Ok(CategoryType::FormulaCar),
            _ => Err("invalid category int")
        }
    }
    pub fn to_db_type(&self) -> i32 {
        return *self as i32;
    }
}