


use actix::{ SystemService};
use actix_web::{HttpResponse, post, web};
use serde_json;
use crate::websocket::session::{ SendMessage};

use crate::{error_handler::CustomError, websocket::session::WebSocketServer};

use super::{CoordinateRequest, CoordinateWSResponse, RouteRequest};



#[post("/startroute")]
async fn create(request: web::Json<RouteRequest>) -> Result<HttpResponse, CustomError> {
    let name = request.id.clone();
    let request = request.into_inner();
    let _response_coords = RouteRequest::handle_coordinates(CoordinateRequest{idspace: request.space, name: request.id.clone(), source: request.source.clone()});
    let _response_coords_with_name = CoordinateWSResponse {
        x: _response_coords.x,
        y: _response_coords.y,
        name: name
    };
    let msg = SendMessage{id: 2, name: String::from("Server"), content:serde_json::to_string(&_response_coords_with_name).unwrap()};
    WebSocketServer::from_registry().do_send(msg);
    let _response = RouteRequest::handle(request);
    Ok(HttpResponse::Ok().json(_response))
}


#[post("/coordinate")]
async fn coordinate(request: web::Json<CoordinateRequest> ) -> Result<HttpResponse, CustomError> {
    let name = request.name.clone();
    let _response = RouteRequest::handle_coordinates(request.into_inner());
    let _response_with_name = CoordinateWSResponse {
        x: _response.x,
        y: _response.y,
        name: name
    };
    let msg = SendMessage{id: 1, name: String::from("Server"), content:serde_json::to_string(&_response_with_name).unwrap()};
    WebSocketServer::from_registry().do_send(msg);
    Ok(HttpResponse::Ok().json(_response))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
    config.service(coordinate);
}