

extern crate dotenv;
use std::time::{SystemTime, UNIX_EPOCH};

use dotenv::dotenv;


use actix_web::{App, Error, FromRequest, HttpRequest, HttpResponse, HttpServer, Responder, web::{self, Data, post, resource}};
use actix_cors::Cors;

use serde::{Deserialize, Serialize};
use std::fs;
use actix_multipart::Multipart;

use futures::{StreamExt, TryStreamExt};
use std::io::Write;



#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
pub mod spaces;
pub mod pois;
pub mod bles;
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
#[derive(Serialize, Deserialize)]
struct File {
    name: String,
    time: u64,
    err: String
}

async fn upload(mut parts: awmp::Parts) -> Result<actix_web::HttpResponse, actix_web::Error> {
    println!("uplkoading {:?}", parts );
    let qs = parts.texts.to_query_string();

    let files = parts
    .files
    .into_inner()
    .into_iter()
    .flat_map(|(name, res_tf)| res_tf.map(|x| (name, x)))
    .map(|(name, tf)| tf.persist_in(dotenv::var("DATASET_FOLDER").expect("Env not set")).map(|f| (name, f)))
    .collect::<Result<Vec<_>, _>>()
    .unwrap_or_default()
    .into_iter()
    .map(|(name, f)| format!("{}: {}", name, f.display()))
    .collect::<Vec<_>>()
    .join(", ");

let body = format!("Text parts: {}, File parts: {}\r\n", &qs, &files);

Ok(HttpResponse::Ok().body(body))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().expect(".env file not found");
   
    
    HttpServer::new( move || {
        App::new()
         .wrap(Cors::permissive())
         .data(awmp::Parts::configure(|cfg| cfg.with_file_limit(1_000_000)))
         .route("/upload", actix_web::web::post().to(upload))

            //.route("/files", web::post().to(upload))
            .route("/welcome", web::get().to(welcome))
            .route("/welcome/{name}", web::get().to(welcome))
            .configure(spaces::init_routes)
            .configure(pois::init_routes)
            .configure(coordinates::init_routes)
            .configure(route_request::init_routes)
            .configure(bles::init_routes)
            .service(websocket::route::websocket)
    })
    .bind("192.168.136.88:8000")?
    .run()
    .await
}