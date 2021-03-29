use actix_web::{web, Responder};
use std::sync;
use crate::model;

pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1").route("/login", web::post().to(list_chain)));
}

pub struct Controller {

}



pub async fn list_chain(
    _req: web::Query<model::PageQuery>,
    _ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    "ok"
}

