use actix_web::{App, HttpServer};

pub mod models;
pub mod routes;
fn publish_url() -> String{
    println!("This app is served from http://127.0.0.1:8000");
    String::from("127.0.0.1:8000")
}
#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        App::new()
            // .route("/",web::get().to(welcome))
            // .route("/{name}",web::get().to(welcome))
            .configure(routes::user_routes)
    })
        .bind(publish_url())?
        .run()
        .await
}
