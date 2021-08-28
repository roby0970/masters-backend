use actix_web::HttpRequest;
use actix_web::web::{Data, Payload};
use actix_web::{get, HttpResponse};
use actix_web_actors::ws;
use super::session::WebSocketSession;

#[get("/ws/")]
pub async fn websocket(
    request: HttpRequest,
    stream: Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let response = ws::start(WebSocketSession::new(), &request, stream);
    response
}