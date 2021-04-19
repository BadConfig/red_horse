use actix_web::{
    web, http, dev, guard, 
    App, HttpResponse, client::Client,
    HttpServer,
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use actix_web::middleware::Logger;
use auth::{db::Auth, reverse_proxy::{
    forward,
    pass_addr,
}};
use auth::routes::{
    create_user,
    send_jwt, 
};
use auth::db::AuthSecret;
use auth::auth::{
   EngineAuth, 
};
use serde_json::json;
extern crate env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let secret = std::sync::Arc::new(AuthSecret([0u8;32]));
    let jwt_guard = EngineAuth::new(secret.clone());
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let client = std::sync::Arc::new(Client::new());
    println!("starting server...");
    HttpServer::new(move || {
        App::new()
            //.data(client.clone())
            .data(pool.clone())
            .data(secret.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/internal")
                    .route("/auth", web::get().to(send_jwt))
                    .route("/auth", web::put().to(create_user))
                    .route("/auth", web::post().to(||HttpResponse::Ok().json(json!({"of":"ok"}))))
            )
            .service(
                web::resource("/private/{role}/{microservice_name}")
                    .guard(jwt_guard.role("admin"))
                    .default_service(web::route().to(forward))
            )
    })
    .bind("0.0.0.0:8088")?
    .system_exit()
    .run()
    .await
}
