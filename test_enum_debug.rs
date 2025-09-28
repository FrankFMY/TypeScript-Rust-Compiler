use std::collections::HashMap;



#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue
}

pub const Pending: &str = "pending";
pub const Approved: &str = "approved";
pub const Rejected: &str = "rejected";


