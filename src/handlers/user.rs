#![allow(dead_code)]
use crate::core::traits::data_source::TableGateway;
use crate::core::types::sqlite3::{Params, Value};
use crate::store::state::AppState;
use actix_web::{web, HttpResponse};
use rs_uuid::uuid16;
use std::include_str;

pub fn get_user_list(data: web::Data<AppState>) -> HttpResponse {
    let user_gateway = &data.db.user_table_gateway;
    user_gateway.create_table();

    let mut p = Params::new();
    p.insert("uuid".to_owned(), Value::Text(uuid16()));
    p.insert("username".to_owned(), Value::Text("Joespeh".to_owned()));
    p.insert("password".to_owned(), Value::Text("1q2w3e4r5t".to_owned()));
    user_gateway.insert(&p);

    p = Params::new();
    p.insert("uuid".to_owned(), Value::Text(uuid16()));
    p.insert("username".to_owned(), Value::Text("Maja".to_owned()));
    p.insert(
        "password".to_owned(),
        Value::Text("dosssd3e4r5t".to_owned()),
    );
    user_gateway.insert(&p);

    let mut body = String::new();

    match user_gateway.find_all() {
        Some(users) => {
            for user in users {
                body.push_str(&format!(
                    include_str!("../templates/user/model.html"),
                    uuid = user.uuid,
                    username = user.username,
                    password = user.password,
                    inserted_at = match user.inserted_at {
                        Some(i) => i,
                        None => "".to_owned(),
                    }
                ))
            }

            let html = format!(
                include_str!("../templates/user/users.html"),
                body = body
            );

            HttpResponse::Ok().content_type("text/html").body(html)
        }
        None => {
            let html = format!(
                include_str!("../templates/user/users.html"),
                body = "No users found".to_owned()
            );

            HttpResponse::Ok().content_type("text/html").body(html)
        }
    }
}
