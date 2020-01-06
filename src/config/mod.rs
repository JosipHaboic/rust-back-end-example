#[allow(dead_code, unused_imports)]
// This is just an example file
use actix_web::{web, App, HttpResponse, HttpServer};

// this function could be located in different module
fn scoped_config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/test")
			.route(web::get().to(|| HttpResponse::Ok().body("test")))
			.route(web::head().to(|| HttpResponse::MethodNotAllowed())),
	);
}

// this function could be located in different module
fn config(cfg: &mut web::ServiceConfig) {
	cfg.service(
		web::resource("/app")
			.route(web::get().to(|| HttpResponse::Ok().body("app")))
			.route(web::head().to(|| HttpResponse::MethodNotAllowed())),
	);
}

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .configure(config)
//             .service(web::scope("/api").configure(scoped_config))
//             .route("/", web::get().to(|| HttpResponse::Ok().body("/")))
//     })
//     .bind("127.0.0.1:8088")?
//     .run()
//     .await
// }
