use std::sync::{Arc, Mutex};
use actix_web::{web, App, HttpServer};
use controllers::user_controller::config;
use test_api::controllers;
use test_api::state::State;
use test_api::structs::user::User;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let stack: Vec<User> = vec![];
    let users= Arc::new(Mutex::new(stack));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                users: users.clone()
            }))
            .configure(config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}