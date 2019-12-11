use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
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

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 8080;
    const API_VERSION: u8 = 1;

    println!("Starting application on: http://{}:{}", ADDRESS, PORT);
    info!(
        "[server] starting application server at: http://{}:{}",
        ADDRESS, PORT
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
            .wrap(middleware::Compress::default())
            .register_data(web::Data::new(store::state::AppState::new(API_VERSION)))
            .service(
                web::scope(&format!("api/v{}", API_VERSION))
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
                // static files
                // fs::Files::new("/", "../front-end/build").index_file("index.html"),
                fs::Files::new("/", "./front-end/temp").index_file("index.html"),
            )
    });

    let mut listenfd = ListenFd::from_env();

    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(format!("{}:{}", ADDRESS, PORT)).unwrap()
    };

    server.run().unwrap()
}
