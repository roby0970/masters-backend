
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
use crate::error_handler::CustomError;
use crate::pois::{Poi, Pois};

#[get("/pois")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let pois = Pois::find_all()?;
    Ok(HttpResponse::Ok().json(pois))
}
#[get("/pois_space/{idspace}")]
async fn find_by_space_id(idspace: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let pois = Pois::find_by_space_id(idspace.into_inner())?;
    Ok(HttpResponse::Ok().json(pois))
}

#[get("/pois/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let poi = Pois::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(poi))
}

#[post("/pois")]
async fn create(poi: web::Json<Poi>) -> Result<HttpResponse, CustomError> {
    let poi = Pois::create(poi.into_inner())?;
    Ok(HttpResponse::Ok().json(poi))
}

#[put("/pois/{id}")]
async fn update(
    id: web::Path<i32>,
    poi: web::Json<Poi>,
) -> Result<HttpResponse, CustomError> {
    let poi = Pois::update(id.into_inner(), poi.into_inner())?;
    Ok(HttpResponse::Ok().json(poi))
}

#[delete("/pois/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_poi = Pois::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_poi })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(find_by_space_id);
    config.service(create);
    config.service(update);
    config.service(delete);
}