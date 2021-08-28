
use std::borrow::Borrow;

use actix_web::{ App, HttpRequest, HttpResponse, HttpServer, Responder, web, Error};
use actix_cors::Cors;
use actix::{dev::MessageResponse, prelude::*};
use actix_web_actors::ws;




#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
pub mod spaces;
pub mod pois;
pub mod route_request;
pub mod coordinates;
pub mod db;
pub mod astar;
pub mod websocket;
pub mod python_knn;
pub mod error_handler;
pub mod schema;



async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


pub struct MyWs;

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {
    
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(&text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

   
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    

    
    
    HttpServer::new( move || {
        App::new()
         .wrap(Cors::permissive())
         //.route("/ws/", web::get().to(websocket))
            .route("/welcome", web::get().to(welcome))
            .route("/welcome/{name}", web::get().to(welcome))
            .configure(spaces::init_routes)
            .configure(pois::init_routes)
            .configure(coordinates::init_routes)
            .configure(route_request::init_routes)
            .service(websocket::route::websocket)
    })
    .bind("192.168.36.88:8000")?
    .run()
    .await
}