use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use listenfd::ListenFd;
use log::info;

mod core;
mod gateways;
mod handlers;
mod models;
mod responders;
mod store;
mod utils;

fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    const ADDRESS: &str = "127.0.0.1";
    const PORT: u16 = 8080;

    println!("Starting application on: http://{}:{}", ADDRESS, PORT);
    info!(
        "[server] starting application server at: http://{}:{}",
        ADDRESS, PORT
    );

    let mut server = HttpServer::new(move || {
        // move state into the closure
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .register_data(web::Data::new(store::state::AppState::new()))
            // .service(web::scope("/api").route("/", web::get().to(handlers::api::index)))
            .service(
                // static files
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
        // .service(
        //     web::scope("/async")
        //     .route("/", web::to_async(handlers::message::index))
        // )
    });

    let mut listenfd = ListenFd::from_env();

    server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        server.bind(format!("{}:{}", ADDRESS, PORT)).unwrap()
    };

    server.run().unwrap()
}
