use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub username: String,
    pub email: String,
    pub cellphone: String
}