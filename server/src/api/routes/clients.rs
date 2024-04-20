use actix_web::{web::Data, CustomizeResponder, HttpResponse, Responder};
use serde_json::Value;
use crate::socket::TcpServer;


pub async fn clients(path:&str, parsed_json: Value, data_tcp_stream:Data<TcpServer>) -> CustomizeResponder<HttpResponse> {


    if path == "clients/exec" {
        return execute(parsed_json, data_tcp_stream).await;
    }


    return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"ok\"}").customize();
}



pub async fn execute(parsed_json: Value, data_tcp_stream:Data<TcpServer>) -> CustomizeResponder<HttpResponse> {

    // get client uuid un body: client_uuid:
    let client_uuid = match parsed_json["client_uuid"].as_str() {
        Some(uuid) => uuid,
        None => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"true\", \"error_msg\": \"client_uuid is required\"}").customize();
        }
    };

    // conver client uuid to Uuid
    let client_uuid_validated = match client_uuid.parse() {
        Ok(uuid) => uuid,
        Err(_) => {
            return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"true\", \"error_msg\": \"client_uuid is not valid\"}").customize();
        }

    };

    let send = data_tcp_stream.send_message(client_uuid_validated, format!("{{\"execute\": \"{}\"}}", parsed_json["execute"].as_str().unwrap()));

    if !send {
        return HttpResponse::Ok().content_type("application/json").body("{\"error\": \"true\", \"error_msg\": \"client_uuid not found\"}").customize();
    }

    return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"ok\"}").customize();
}
