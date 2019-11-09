#![allow(dead_code)]
use crate::core::traits::base::Gateway;
use crate::core::traits::data_source::TableGateway;
use crate::core::types::sqlite3::{Params, Value};
use crate::gateways::user::UserTableGateway;
use crate::inputs::user::UserInput;
use crate::store::state::AppState;
use actix_web::{web, HttpResponse};
use rs_uuid::uuid16;

pub fn get_user_list(data: web::Data<AppState>) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    match user_gateway.find_all() {
        Some(users) => HttpResponse::Ok().json(users),
        None => HttpResponse::Ok().json(()),
    }
}

pub fn get_user(
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    match user_gateway.find(&path.0) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::Ok().json(()),
    }
}

pub fn create_user(
    form: web::Form<UserInput>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    let mut p = Params::new();
    p.insert("uuid".to_owned(), Value::Text(uuid16()));
    p.insert("username".to_owned(), Value::Text(form.username.clone()));
    p.insert("password".to_owned(), Value::Text(form.password.clone()));

    if user_gateway.insert(&p) {
        HttpResponse::Ok().json(
            r#"{
                status: \"Ok\",
                message: \"user is inserted\"
            }"#)
    } else {
        HttpResponse::Ok().json(
            r#"{
                status: \"Error\",
                message: \"user is not inserted\"
            }"#,
        )
    }
}

pub fn update_user(
    path: web::Path<(String,)>,
    form: web::Form<UserInput>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    let mut p = Params::new();
    p.insert("uuid".to_owned(), Value::Text(path.0.clone()));
    p.insert("username".to_owned(), Value::Text(form.username.clone()));
    p.insert("password".to_owned(), Value::Text(form.password.clone()));

    if user_gateway.update(&p) {
        HttpResponse::Ok().json(
            r#"{
                status: \"Ok\",
                message: \"user is updated\"
            }"#
        )
    } else {
        HttpResponse::Ok().json(
            r#"{
                status: \"Error\",
                message: \"user is not updated\"
            }"#
        )
    }
}

pub fn delete_user(
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    if user_gateway.delete(&path.0) {
        HttpResponse::Ok().json(
            r#"{
                status: \"Ok\",
                message: \"user is deleted\"
            }"#
        )
    } else {
        HttpResponse::Ok().json(
            r#"{
                status: \"Error\",
                message: \"user is not deleted\"
            }"#
        )
    }
}
