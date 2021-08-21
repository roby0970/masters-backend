use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, post, web};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
pub mod spaces;
pub mod db;
pub mod error_handler;
pub mod schema;
const PYEXE: &str = "C:/Users/Robi/AppData/Local/Programs/Python/Python38/python.exe";
const PYSCRIPT: &str  = "c:/diplomski/blefingerprinting android/convert data to fluter class/train.py";

async fn welcome(request: HttpRequest) -> impl Responder {
    let name = request.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
         .wrap(Cors::permissive())
            .route("/welcome", web::get().to(welcome))
            .route("/welcome/{name}", web::get().to(welcome))
            .configure(spaces::init_routes)
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
