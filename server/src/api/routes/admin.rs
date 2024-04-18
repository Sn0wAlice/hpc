use actix_web::{web::Data, CustomizeResponder, HttpResponse, Responder};
use serde_json::Value;
use crate::socket::TcpServer;


pub async fn admin(path:&str, parsed_json: Value, data_tcp_stream:Data<TcpServer>) -> CustomizeResponder<HttpResponse> {


    if path == "admin/status" {
        return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"online\"}").customize();
    } else if path == "admin/clients" {
        let clients = data_tcp_stream.get_clients_str();
        let resonse = format!("{{\"error\": false, \"clients\": {:?}}}", &clients);
        return HttpResponse::Ok().content_type("application/json").body(resonse).customize();
    }



    return HttpResponse::Ok().content_type("application/json").body("{\"status\": \"ok\"}").customize();
}