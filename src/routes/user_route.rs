use actix_web::{Responder,HttpResponse,get,web};
use crate::models::user::PublicUsers;
use sqlx::PgPool;

#[get("/users")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = PublicUsers::find_all(db_pool.get_ref()).await;
    match result {
        Ok(public_users) => HttpResponse::Ok().json(public_users),
        _ => HttpResponse::BadRequest().body("Users not found"),
    }
}

#[get("/users/id")]
async fn find() -> impl Responder{
    HttpResponse::Ok().json(
        PublicUsers{
            user_id: 2,
            firstname : "James".to_string(),
            lastname: "Bond".to_string(),
        }
    )
}

pub fn user_routes(config: &mut web::ServiceConfig){
    config.service(find);
    config.service(find_all);
}