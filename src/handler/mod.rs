use crate::{model, service};
use actix_web::{web, Responder};
use std::sync;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .route("/help", web::get().to(help))
            .route("/build", web::post().to(build))
            .route("/setup", web::post().to(setup)),
    );
}

pub struct Controller {
    svc: service::Service,
}

impl Controller {
    pub fn new(svc: service::Service) -> Controller {
        Controller { svc }
    }
}

pub async fn help() -> impl Responder {
    "欢迎使用 help"
}

pub async fn build(
    req: web::Json<model::Contract>,
    ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let c = req.into_inner();
    ctrl.svc.build(&c.contract_type, &c.path)?;
    model::Response::ok("build success!").json()
}

pub async fn setup(
    _req: web::Json<model::Contract>,
    _ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    "ok"
}
