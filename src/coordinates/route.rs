
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::error_handler::CustomError;
use crate::coordinates::{Coordinate, Coordinates};

#[get("/coordinates")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let coordinates = Coordinates::find_all()?;
    Ok(HttpResponse::Ok().json(coordinates))
}
#[get("/coordinates_space/{idspace}")]
async fn find_by_space_id(idspace: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let coordinates = Coordinates::find_by_space_id(idspace.into_inner())?;
    Ok(HttpResponse::Ok().json(coordinates))
}
#[get("/coordinates/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let coordinate = Coordinates::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(coordinate))
}

#[post("/coordinates")]
async fn create(coordinate: web::Json<Coordinate>) -> Result<HttpResponse, CustomError> {
    let coordinate = Coordinates::create(coordinate.into_inner())?;
    Ok(HttpResponse::Ok().json(coordinate))
}

#[put("/coordinates/{id}")]
async fn update(
    id: web::Path<i32>,
    coordinate: web::Json<Coordinate>,
) -> Result<HttpResponse, CustomError> {
    let coordinate = Coordinates::update(id.into_inner(), coordinate.into_inner())?;
    Ok(HttpResponse::Ok().json(coordinate))
}

#[delete("/coordinates/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_coordinate = Coordinates::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_coordinate })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(find_by_space_id);
    config.service(create);
    config.service(update);
    config.service(delete);
}