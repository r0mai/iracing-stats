#[derive(Copy, Clone)]
pub enum EventType {
    Practice = 2,
    Qualify = 3,
    TimeTrial = 4,
    Race = 5
}

impl EventType {
    pub fn to_db_type(&self) -> i32 {
        return *self as i32;
    }
}