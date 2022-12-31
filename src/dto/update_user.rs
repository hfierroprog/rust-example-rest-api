use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct UpdateUser {
    pub id: usize,
    pub username: String,
    pub email: String,
    pub cellphone: String
}