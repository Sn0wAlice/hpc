use actix_web::{web, Scope, post, HttpResponse, Responder, App};
use serde_json::Value;
use crate::socket::TcpServer;
use crate::socket::create_server;
use futures::StreamExt;

use crate::api::routes::*;

const MAX_SIZE: usize = 262_144;

#[post("/{path:.*}")]
async fn handler(path: web::Path<String>, mut payload: web::Payload, socket_server: web::Data<TcpServer>) -> impl Responder {
    println!("Received request: {}", path.to_string());

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = match chunk {
            Ok(chunk) => chunk,
            Err(_) => {
                return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"error\"}").customize();
            }
        };
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"error\"}").customize();
        }
        body.extend_from_slice(&chunk);
    }

    // Get the expected data
    let str_data = std::str::from_utf8(&body).expect("Invalid UTF-8");
    let parsed_json: Value = serde_json::from_str(str_data).expect("Failed to parse JSON");
    println!("Body {:?}", parsed_json);
    
    let clients = socket_server.get_clients(); // Retrieve the client list
    println!("TCP client {:?}", clients);


    match path.clone().as_str() {
        "" => HttpResponse::Ok().content_type("application/json").body("{\"status\": \"OK\"}").customize(),

        path if path.starts_with("admin/") => {
            admin::admin(path, parsed_json, socket_server).await
        }

        path if path.starts_with("clients/") => {
            clients::clients(path, parsed_json, socket_server).await
        }
        
        _ => {
            println!("404 Not Found: {}", path.to_string());
            HttpResponse::Ok().content_type("application/json").body("{\"error\": \"path not found\"}").customize()
        }
    }
}




pub fn init_api() -> Scope {
    let socket_server = create_server().unwrap_or_else(|err| {
        panic!("Failed to create socket server: {}", err);
    });

    socket_server.clone().start().unwrap_or_else(|err| {
        panic!("Failed to start socket server: {}", err);
    });

    web::scope("/api").service(handler).app_data(web::Data::new(socket_server))
}
