#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SimsessionType {
    OpenPractice = 3,
    LoneQualifying = 4,
    OpenQualifying = 5,
    Race = 6
}

impl SimsessionType {
    pub fn from_i32(i: i32) -> Result<Self, &'static str> {
        return match i {
            3 => Ok(SimsessionType::OpenPractice),
            4 => Ok(SimsessionType::LoneQualifying),
            5 => Ok(SimsessionType::OpenQualifying),
            6 => Ok(SimsessionType::Race),
            _ => Err("invalid simsession type int")
        }
    }

    pub fn to_db_type(&self) -> i32 {
        return *self as i32;
    }
}