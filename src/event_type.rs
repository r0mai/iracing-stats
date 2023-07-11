#[derive(Copy, Clone, PartialEq, Eq)]
pub enum EventType {
    Practice = 2,
    Qualify = 3,
    TimeTrial = 4,
    Race = 5
}

impl EventType {
    pub fn from_i32(i: i32) -> Result<Self, &'static str> {
        return match i {
            2 => Ok(EventType::Practice),
            3 => Ok(EventType::Qualify),
            4 => Ok(EventType::TimeTrial),
            5 => Ok(EventType::Race),
            _ => Err("invalid event type int")
        }
    }
    pub fn to_db_type(&self) -> i32 {
        return *self as i32;
    }
    pub fn to_nice_string(&self) -> &str {
        return match self {
            EventType::Practice => "Practice",
            EventType::Qualify => "Qualify",
            EventType::TimeTrial => "Time Trial",
            EventType::Race => "Race",
        };
    }
}