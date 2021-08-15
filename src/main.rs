use actix_web::{App, HttpServer};
use std::env;
use sqlx::Pool;
use sqlx::PgPool;
use dotenv::dotenv;
use listenfd::ListenFd;

pub mod models;
pub mod routes;


fn publish_url() -> String{
    println!("This app is served from http://127.0.0.1:8000");
    String::from("127.0.0.1:8000")
}
#[actix_rt::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    let mut listenfd = ListenFd::from_env();
    let database_url = env::var("DATABASE_URL").expect("Error in getting database url");

    let db_pool = PgPool::builder()
        .max_size(5) // maximum number of connections in the pool
        .build(&database_url).await.expect("Eror Connecting");
    let mut server = HttpServer::new(move || {
        App::new()
            // .route("/",web::get().to(welcome))
            // .route("/{name}",web::get().to(welcome))
            .data(db_pool.clone())
            .configure(routes::user_routes)
    });
    server = match listenfd.take_tcp_listener( 0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            server.bind(publish_url())?
        }
    };

    println!("Starting server");
    server.run().await?;

    Ok(())
}
