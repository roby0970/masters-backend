
use std::borrow::Borrow;

use actix::{Addr, Recipient, SystemService};
use actix_web::{App, HttpRequest, HttpResponse, post, web};
use serde_json;
use crate::websocket::session::{Message, SendMessage};
use uuid::Uuid;
use crate::{error_handler::CustomError, websocket::session::WebSocketServer};

use super::{CoordinateRequest, RouteRequest};



#[post("/startroute")]
async fn create(request: web::Json<RouteRequest>) -> Result<HttpResponse, CustomError> {
    let _response = RouteRequest::handle(request.into_inner());
    Ok(HttpResponse::Ok().json(_response))
}


#[post("/coordinate")]
async fn coordinate(request: web::Json<CoordinateRequest> ) -> Result<HttpResponse, CustomError> {
    println!("Req");
    let _response = RouteRequest::handleCoordinates(request.into_inner());
    let msg = SendMessage{id: 1, name: String::from("a"), content:serde_json::to_string(&_response).unwrap()};
    WebSocketServer::from_registry().do_send(msg);
    Ok(HttpResponse::Ok().json(_response))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
    config.service(coordinate);
}