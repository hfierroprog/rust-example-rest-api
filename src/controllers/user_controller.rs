use actix_web::{Responder, web, get, post, put, delete, HttpResponse};
use web::Data;
use crate::dto::create_user::CreateUser;
use crate::dto::info::Info;
use crate::dto::update_user::UpdateUser;
use crate::state::State;
use super::super::structs::user::User;

#[get("/user")]
async fn get_users(data: Data<State>) -> impl Responder {
    let users = data.users.lock().expect("bad state");

    web::Json(users.clone())
}

#[get("/user/{id}")]
async fn get_user_by_id(data: Data<State>, info: web::Path<Info>) -> impl Responder {
    let mut users = data.users.lock().expect("bad state");

    let found_user = users
        .iter_mut()
        .find(|user| user.id == info.id);

    if let Some(found_user) = found_user {
        HttpResponse::Ok().json(found_user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[post("/user")]
async fn create_user(dto: web::Json<CreateUser>, data: Data<State>) -> impl Responder {
    let mut users = data.users.lock().expect("bad state");
    let mut id: usize = 1;

    if users.len() > 0 {
        id = users[users.len() - 1].id.clone() + 1;
    }

    let new_user: User = User {
        id,
        username: dto.username.clone(),
        email: dto.email.clone(),
        cellphone: dto.cellphone.clone()
    };

    users.push(new_user.clone());

    HttpResponse::Ok().json(web::Json(new_user))
}

#[put("/user")]
async fn update_user(dto: web::Json<UpdateUser>, data: Data<State>) -> impl Responder {
    let mut users = data.users.lock().expect("bad state");

    let found_user = users
        .iter_mut()
        .find(|user| user.id == dto.id);

    if let Some(found_user) = found_user {
        found_user.username = dto.username.clone();
        found_user.email = dto.email.clone();
        found_user.cellphone = dto.cellphone.clone();

        HttpResponse::Ok().json(web::Json(found_user))
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

#[delete("/user/{id}")]
async fn delete_user(data: Data<State>, info: web::Path<Info>) -> impl Responder {
    let mut users = data.users.lock().expect("bad state");

    let found_index = users
        .iter_mut()
        .enumerate()
        .find(|(_index, user)| user.id == info.id)
        .map(|(index, _user)| index);

    if let Some(found_index) = found_index {
        users.remove(found_index);
        HttpResponse::Ok().body("User deleted successful")
    }else {
        HttpResponse::NotFound().body("User not found")
    }
}

// this function could be located in a different module
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_users);
    cfg.service(get_user_by_id);
    cfg.service(create_user);
    cfg.service(update_user);
    cfg.service(delete_user);
}