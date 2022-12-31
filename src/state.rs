use std::sync::{Mutex, Arc};
use crate::structs::user::User;

pub struct State {
    pub users: Arc<Mutex<Vec<User>>>
}