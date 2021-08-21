
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::error_handler::CustomError;
use crate::spaces::{Space, Spaces};

#[get("/spaces")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let spaces = Spaces::find_all()?;
    Ok(HttpResponse::Ok().json(spaces))
}

#[get("/spaces/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let space = Spaces::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(space))
}

#[post("/spaces")]
async fn create(space: web::Json<Space>) -> Result<HttpResponse, CustomError> {
    let space = Spaces::create(space.into_inner())?;
    Ok(HttpResponse::Ok().json(space))
}

#[put("/spaces/{id}")]
async fn update(
    id: web::Path<i32>,
    space: web::Json<Space>,
) -> Result<HttpResponse, CustomError> {
    let space = Spaces::update(id.into_inner(), space.into_inner())?;
    Ok(HttpResponse::Ok().json(space))
}

#[delete("/spaces/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_space = Spaces::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_space })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}