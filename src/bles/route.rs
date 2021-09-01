
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use crate::error_handler::CustomError;
use crate::bles::{Ble,Bles};

#[get("/bles")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let bles = Bles::find_all()?;
    Ok(HttpResponse::Ok().json(bles))
}
#[get("/bles_space/{idspace}")]
async fn find_by_space_id(idspace: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let bles = Bles::find_by_space_id(idspace.into_inner())?;
    Ok(HttpResponse::Ok().json(bles))
}

#[get("/bles/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let ble = Bles::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(ble))
}

#[post("/bles")]
async fn create(ble: web::Json<Ble>) -> Result<HttpResponse, CustomError> {
    let ble = Bles::create(ble.into_inner())?;
    Ok(HttpResponse::Ok().json(ble))
}

#[put("/bles/{id}")]
async fn update(
    id: web::Path<i32>,
    ble: web::Json<Ble>,
) -> Result<HttpResponse, CustomError> {
    let ble = Bles::update(id.into_inner(), ble.into_inner())?;
    Ok(HttpResponse::Ok().json(ble))
}

#[delete("/bles/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_ble = Bles::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_ble })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(find_by_space_id);
    config.service(create);
    config.service(update);
    config.service(delete);
}