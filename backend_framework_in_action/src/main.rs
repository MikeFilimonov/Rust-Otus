mod actions;
mod config;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::*;
use actix_web::{web, web::Data, App, HttpServer};
use dotenv::dotenv;
use tokio_postgres::NoTls;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    //loading envs
    dotenv().ok();
    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => panic!("Failed to read the config because of {e}"),
    };

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .route("/howdy", web::get().to(handlers::howdy))
            .route("/rooms{_:/?}", web::get().to(get_room_list))
            .route("/rooms{_:/?}", web::post().to(add_room))
            .route("/rooms/{room_id}{_:/?}", web::delete().to(remove_room))
            .route(
                "/rooms/{room_id}/devices{_:/?}",
                web::get().to(get_device_list),
            )
            .route("/rooms/{room_id}/devices{_:/?}", web::post().to(add_device))
            .route(
                "/rooms/{room_id}/devices/{device_id}{_:/?}",
                web::delete().to(remove_device),
            )
            .route("/full_report{_:/?}", web::get().to(get_full_report))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {}
