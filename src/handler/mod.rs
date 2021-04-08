use crate::{model, service};
use actix_web::{web, Responder};
use std::sync;
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use error::*;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .route("/command", web::post().to(command)),
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

pub async fn command(
    mut payload:  Multipart,
    ctrl: web::Data<sync::Arc<Controller>>, ) -> impl Responder {
    let mut name = None;
    let mut contract_type = None;
    let mut env = None;
    let mut command = None;
    let mut path = "fs";
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().ok_or_else(||anyhow!("parse error"))?;
        let param_name = match content_type.get_name() {
            Some(v) => v,
            None => continue,
        };

        match param_name {
            "name" => {
                let v = read_field(field).await?;
                name = Some(String::from_utf8(v)?);
            },
            "contract_type" => {
                let v = read_field(field).await?;
                contract_type = Some(String::from_utf8(v)?);
            },
            "env" => {
                let v = read_field(field).await?;
                env = Some(String::from_utf8(v)?);
            },
            "command" => {
                let v = read_field(field).await?;
                command = Some(String::from_utf8(v)?);
            },
            "file" => {
                let fname = match content_type.get_filename() {
                    Some(v) => v,
                    None => continue,
                };
                info!("file name: {}", fname);
            },
            _ => continue,
        };
    }

    let command = command.ok_or_else(||anyhow!("command line is null"))?;
    info!("command: {}, contract name: {:?},contract type: {:?}, env: {:?}", command, name, contract_type, env);
    model::Response::ok("success!").json()
}

async fn read_field(mut field: Field) -> Result<Vec<u8>> {
    let mut content: Vec<u8> = vec![];
    while let Some(chunk) = field.next().await {
        let mut data = chunk?;
        content.append(&mut data.to_vec());
    }
    Ok(content)
}
