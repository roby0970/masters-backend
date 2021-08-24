
use actix_web::{middleware, App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web, Error};
use actix_cors::Cors;
use actix::prelude::*;
use actix_web_actors::ws;
use serde::{Serialize, Deserialize};




#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
pub mod spaces;
pub mod pois;
pub mod coordinates;
pub mod db;
pub mod error_handler;
pub mod schema;
const PYEXE: &str = "C:/Users/Robi/AppData/Local/Programs/Python/Python38/python.exe";
const PYSCRIPT: &str  = "c:/diplomski/blefingerprinting android/convert data to fluter class/train.py";

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}


struct MyWs;

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


async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWs {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
         .wrap(Cors::permissive())
         .route("/ws/", web::get().to(websocket))
            .route("/welcome", web::get().to(welcome))
            .route("/welcome/{name}", web::get().to(welcome))
            .configure(spaces::init_routes)
            .configure(pois::init_routes)
            .configure(coordinates::init_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
/*#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .service(hello)
    })
    .bind("192.168.0.18:8081")?
    
    .run()
    .await
}
#[derive(Debug, Deserialize, Serialize)]
struct Req {
    pub keys : String,
    pub rssi : String,
}
#[post("/")]
async fn hello(post: web::Json<Req>) -> impl Responder {
    use std::process::Command;
    
    let output = Command::new("cmd")
    .args(&["/C", &PYEXE, &PYSCRIPT, &post.keys, &post.rssi])
    .output()
    .expect("failed to execute process");

    HttpResponse::Ok().body(output.stdout)
}*/
