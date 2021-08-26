
use actix_web::{post, web, HttpResponse};
use crate::error_handler::CustomError;

use super::RouteRequest;



#[post("/startroute")]
async fn create(request: web::Json<RouteRequest>) -> Result<HttpResponse, CustomError> {
    let _response = RouteRequest::handle(request.into_inner());
    Ok(HttpResponse::Ok().json(_response))
}


pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}