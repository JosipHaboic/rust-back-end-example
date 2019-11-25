#![allow(dead_code)]
use crate::core::traits::base::Gateway;
use crate::core::traits::data_source::TableGateway;
use crate::core::types::sqlite3::{Params, Value};
use crate::gateways::user::UserTableGateway;
use crate::inputs::user::UserInput;
use crate::api::responses::error::ErrorResponse;
use crate::api::responses::user::UsersResponse;
use crate::store::state::AppState;
use actix_web::{web, HttpResponse};
use serde_json::json;
use rs_uuid::uuid16;


pub fn get_user_list<'a>(data: web::Data<AppState>) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    if let Some(users) = user_gateway.find(None) {
        HttpResponse::Ok().json(UsersResponse {
            status: 200,
            message: &format!("{} items found", users.len()),
            result: users
        })
    } else {
        HttpResponse::Ok().json(
            ErrorResponse
            {
                status: 400,
                message: "Not found"
            }
        )
    }
}

pub fn get_user(
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    match user_gateway.find(Some(&path.0)) {
        Some(user) => HttpResponse::Ok().json(UsersResponse {
            status: 200,
            message: &format!("{} items found", 1),
            result: user
        }),
        None => HttpResponse::Ok().json(ErrorResponse
            {
                status: 400,
                message: "Not found"
            }
        )
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
        HttpResponse::Ok().json(json!(
            {
                "status": 200,
                "message": "User inserted"
            }))
    } else {
        HttpResponse::Ok().json(ErrorResponse
            {
                status: 400,
                message: "User not inserted"
            }
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
        HttpResponse::Ok().json(json!(
            {
                "status": 200,
                "message": "User updated"
            }))
    } else {
        HttpResponse::Ok().json(ErrorResponse
            {
                status: 400,
                message: "User is not updated"
            }
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
        HttpResponse::Ok().json(json!(
            {
                "status": 200,
                "message": "User deleted"
            }))
    } else {
        HttpResponse::Ok().json(ErrorResponse
            {
                status: 400,
                message: "User was not deleted"
            }
        )
    }
}
