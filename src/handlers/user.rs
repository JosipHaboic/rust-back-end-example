use actix_web::{web, HttpResponse};
use rs_uuid::uuid16;
use std::include_str;
use crate::store::state::AppState;
use crate::core::types::sqlite3::{Params, Value};
use crate::core::traits::data_source::TableGateway;
// get list of users
// GET
// /users
pub fn get_user_list(data: web::Data<AppState>) -> HttpResponse {
    let user_gateway = data.db.user_table_gateway;
	user_gateway.create_table();

	let mut p = Params::new();
	p.insert("uuid", Value::Text(uuid16()));
	p.insert("username", Value::Text("Joespeh".to_owned()));
	p.insert("password", Value::Text("1q2w3e4r5t".to_owned()));
	user_gateway.insert(&p);

	p = Params::new();
	p.insert("uuid", Value::Text(uuid16()));
	p.insert("username", Value::Text("Maja".to_owned()));
	p.insert("password", Value::Text("dosssd3e4r5t".to_owned()));
	user_gateway.insert(&p);

	let mut body = String::new();
	let users = user_gateway.find_all();

	for user in users[0] {
		body.push_str(&format!(
			include_str!("../templates/user/model.html"),
			uuid = user.uuid,
			username = user.username,
			password = user.password,
			created_at = user.created_at
		))
	}

	HttpResponse::Ok().content_type("text/html").body(body)
}

// get user by id
// GET
// /users/{id}

// update
// UPDATE
// /users/{id}/?username="username"

// delete
// DELETE
// /users/{id}
