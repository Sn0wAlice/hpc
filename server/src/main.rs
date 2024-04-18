extern crate server;
use std::fs;
use actix_cors::Cors;
use actix_web::{HttpServer,App};
use server::api;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // Read ascii art in utils/ascii.art
    let ascii_art = include_str!("../utils/ascii.art");
    println!("{}", ascii_art);
    
    let port: u16 = 15001;
    HttpServer::new(|| {
        let cors = Cors::default().allow_any_origin().allow_any_method().allow_any_header();
        App::new().wrap(cors).service(api::init::init_api())
    })
    .bind(("0.0.0.0",port))?
    .workers(1)
    .run().await
}
