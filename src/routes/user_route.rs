use actix_web::{Responder,HttpResponse,get,web};
use crate::models::user::PublicUsers;

#[get("/users")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(vec![
        PublicUsers {
            user_id: 1,
            first_name : "Ola".to_string(),
            last_name: "John Ajiboye".to_string(),
        },
        PublicUsers {
            user_id: 2,
            first_name : "James".to_string(),
            last_name: "Bond".to_string(),
        },

    ])
}

#[get("/users/id")]
async fn find() -> impl Responder{
    HttpResponse::Ok().json(
        PublicUsers{
            user_id: 2,
            first_name : "James".to_string(),
            last_name: "Bond".to_string(),
        }
    )
}

pub fn user_routes(config: &mut web::ServiceConfig){
    config.service(find);
    config.service(find_all);
}