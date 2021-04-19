use actix_web::{
    web, http, dev, guard, 
    App, HttpResponse, client::Client,
    HttpServer,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("starting server...");
    HttpServer::new(move || {
        App::new()
            .service(
                web::resource("/private/admin/productsbackend")
                    .route(web::get().to(||HttpResponse::Ok().json("")))
            )
    })
    .bind("0.0.0.0:8088")?
    .system_exit()
    .run()
    .await
}
