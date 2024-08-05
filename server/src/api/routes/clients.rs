use actix_web::{web::Data, CustomizeResponder, HttpResponse, Responder};
use serde_json::{json, Value};
use crate::socket::TcpServer;


pub async fn clients(path:&str, parsed_json: Value, data_tcp_stream:Data<TcpServer>) -> CustomizeResponder<HttpResponse> {


    if path == "clients/list" {
        return clients_list(path, parsed_json, data_tcp_stream).await;
    }


    return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"ok\"}").customize();
}

async fn clients_list(path:&str, parsed_json: Value, data_tcp_stream:Data<TcpServer>) -> CustomizeResponder<HttpResponse> {
    // get a pretty list of connected clients
    let clients = data_tcp_stream.get_clients_str();

    let response_json = json!({
        "status": "ok",
        "clients": clients
    });

    return HttpResponse::Ok().content_type("application/json").body(response_json.to_string()).customize();
}