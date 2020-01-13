use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use actix_cors::Cors;
use env_logger;
use listenfd::ListenFd;
use log::info;
use rusqlite::Connection;
use std::include_str;

mod api;
mod core;
mod gateways;
mod handlers;
mod inputs;
mod models;
mod responders;
mod store;
mod transaction_scripts;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let address: &str = "127.0.0.1";
    let port: &'static str = option_env!("port").unwrap_or("8080");
    let api_version: u8 = 1;

    println!("Starting application on: http://{}:{}", address, port);
    info!(
        "[server] starting application server at: http://{}:{}",
        address, port
    );

    let connection = Connection::open("./database.db").unwrap();
    connection
        .execute_batch(include_str!("./sql/user/__create__.sql"))
        .unwrap();

    connection.close().unwrap();

    let mut server = HttpServer::new(move || {
        // move state into the closure
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .wrap(middleware::Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::Compress::default())
            .wrap(Cors::default())
            // .app_data(web::Data::new(store::state::AppState::new(api_version)))
            .app_data(web::Data::new(store::state::AppState::new(api_version)))
            .service(
                web::scope(&format!("api/v{}", api_version))
                    .service(
                        web::resource("/users/")
                            .name("users")
                            .route(web::get().to(handlers::user::get_user_list))
                            .route(web::post().to(handlers::user::create_user)),
                    )
                    .service(
                        web::resource("/users/{id}")
                            .name("user")
                            .route(web::get().to(handlers::user::get_user))
                            .route(web::put().to(handlers::user::update_user))
                            .route(web::delete().to(handlers::user::delete_user)),
                    ),
            )
            .service(
                fs::Files::new("/", "../front-end-react/build")
                    .index_file("index.html"),
            )
    });

    let mut listenfd = ListenFd::from_env();

    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(format!("{}:{}", address, port)).unwrap()
    };

    server.run().await
}
