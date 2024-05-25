
pub enum DriverId {
    Name(String),
    CustId(i64)
}

impl DriverId {
    pub fn from_params(driver_name: Option<String>, cust_id: Option<i64>) -> Option<Self> {
        if let Some(cust_id) = cust_id {
            return Some(DriverId::CustId(cust_id));
        }
        if let Some(driver_name) = driver_name {
            return Some(DriverId::Name(driver_name));
        }
        return None;
    }
}
