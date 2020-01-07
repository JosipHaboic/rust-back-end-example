#![allow(dead_code)]
use crate::api::responses::error::ErrorResponse;
use crate::api::responses::user::UsersResponse;
use crate::core::traits::base::Gateway;
use crate::core::traits::data_source::TableGateway;
use crate::core::types::sqlite3::{Params, Value};
use crate::gateways::user::UserTableGateway;
use crate::inputs::user::UserInput;
use crate::store::state::AppState;
use actix_web::{web, HttpResponse};
use rs_uuid::uuid16;
use serde_json::json;
use std::error::Error;

pub fn get_user_list(data: web::Data<AppState>) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    if let Ok(users) = user_gateway.find(None) {
        HttpResponse::Ok().json(UsersResponse {
            status: 200,
            message: &format!("{} items found", users.len()),
            result: users,
        })
    } else {
        HttpResponse::Ok().json(ErrorResponse {
            status: 400,
            message: "Not found",
        })
    }
}

pub fn get_user(
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    match user_gateway.find(Some(&path.0)) {
        Ok(user) => HttpResponse::Ok().json(UsersResponse {
            status: 200,
            message: &format!("{} items found", 1),
            result: user,
        }),
        Err(error) => HttpResponse::Ok().json(ErrorResponse {
            status: 400,
            message: error.description(),
        }),
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

    match user_gateway.insert(&p) {
        Ok(_) => HttpResponse::Ok().json(json!(
            {
                "status": 200,
                "message": "User inserted"
            }
        )),
        Err(error) => HttpResponse::Ok().json(ErrorResponse {
            status: 400,
            message: error.description(),
        }),
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

    match user_gateway.update(&p) {
        Ok(_) => HttpResponse::Ok().json(json!(
        {
            "status": 200,
            "message": "User updated"
        })),
        Err(error) => HttpResponse::Ok().json(ErrorResponse {
            status: 400,
            message: error.description(),
        }),
    }
}

pub fn delete_user(
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let connection = data.db.connection.lock().unwrap();
    let user_gateway = UserTableGateway::init(&connection);

    match user_gateway.delete(&path.0) {
        Ok(_) => HttpResponse::Ok().json(json!(
        {
            "status": 200,
            "message": "User deleted"
        })),
        Err(error) => HttpResponse::Ok().json(ErrorResponse {
            status: 400,
            message: error.description(),
        }),
    }
}

// !TODO: Do the test for the handlers: https://actix.rs/docs/testing/