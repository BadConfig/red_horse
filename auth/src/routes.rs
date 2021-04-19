use actix_web::{
    web, http, dev, guard, 
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder,
};
use serde::Deserialize;
use crate::db::Auth;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
use crate::error::ApiError;

#[derive(Deserialize,Debug)]
pub struct UserData {
    login: String,
    auth_type: String,
}
use crate::db::AuthSecret;

pub async fn send_jwt(
    query: web::Json<UserData>,
    conn: web::Data<DbPool>, 
    secret: web::Data<AuthSecret>,
    _req: HttpRequest
) -> Result<HttpResponse,ApiError> {
    let conn = conn.get()?;
    let r = HttpResponse::Ok()
        .json(json!({"jwt": Auth::get(
            &query.login,
            &query.auth_type,
            &conn)
                .await?
                .get_jwt(&secret)
                .await?}));
    Ok(r)
}

#[derive(Deserialize,Debug)]
pub struct NewUser {
    login: String,
    auth_type: String,
    roles: Vec<String>,
}

pub async fn create_user(
    form: web::Json<NewUser>,
    conn: web::Data<DbPool>, 
    _req: HttpRequest
) -> Result<HttpResponse,ApiError> {
    let conn = conn.get()?;
    println!("roles: {:?}",form.roles);
    Auth::new(
        &form.login, 
        &form.auth_type, 
        &form.roles, 
        &conn)
    .await?;
    Ok(HttpResponse::Ok().json(json!({"code":200})))
}
