use crate::repl::CommandType;
use crate::{model, service};
use actix_multipart::{Field, Multipart};
use actix_web::{web, Responder};
use error::*;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use std::sync;

pub fn app_config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1").route("/command", web::post().to(command)));
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
    mut payload: Multipart,
    _ctrl: web::Data<sync::Arc<Controller>>,
) -> impl Responder {
    let mut name = None;
    let mut contract_type = None;
    let mut env = None;
    let mut command = None;
    let _path = "fs";
    while let Ok(Some(field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| anyhow!("parse error"))?;
        let param_name = match content_type.get_name() {
            Some(v) => v,
            None => continue,
        };

        match param_name {
            "name" => {
                let v = read_field(field).await?;
                name = Some(String::from_utf8(v)?);
            }
            "contract_type" => {
                let v = read_field(field).await?;
                contract_type = Some(String::from_utf8(v)?);
            }
            "env" => {
                let v = read_field(field).await?;
                env = Some(String::from_utf8(v)?);
            }
            "command" => {
                let v = read_field(field).await?;
                command = Some(String::from_utf8(v)?);
            }
            "file" => {
                let fname = match content_type.get_filename() {
                    Some(v) => v,
                    None => continue,
                };
                let v = read_field(field).await?;
                let hash = utils::hash::compute_sha256(&v);
                let hash = utils::hash::hex_to_string(&hash);
                utils::pack::unpack(&v, Some("/home/snlan/yy"))?;
                info!("unpack file {} success, hash: {}", fname, hash);
            }
            _ => continue,
        };
    }

    let command = command.ok_or_else(|| anyhow!("command line is null"))?;
    info!(
        "command: {}, contract name: {:?},contract type: {:?}, env: {:?}",
        command, name, contract_type, env
    );

    let output = match CommandType::new(&command)? {
        CommandType::Help => {
            r#"Welcome to use the blockchain smart contract runtime, we will provide you
               with the complete contract test、running and call contract, you can use the commands below:
                    help
                    build
                    setup
            "#.to_string()
        }
        CommandType::Build => {
            "build".to_string()
        }
        CommandType::Setup => {
            "setup".to_string()
        }
        CommandType::Command(c) => {
            c
        }
        CommandType::Unknown(c) => {
            format!("unknown command {}, please use `help`", c)
        }
    };

    model::Response::ok(output).json()
}

async fn read_field(mut field: Field) -> Result<Vec<u8>> {
    let mut content: Vec<u8> = vec![];
    while let Some(chunk) = field.next().await {
        let data = chunk?;
        content.append(&mut data.to_vec());
    }
    Ok(content)
}

async fn write_file(mut field: Field, name: &str) -> Result<String> {
    let filepath = format!("{}", sanitize_filename::sanitize(name));
    let path = filepath.clone();
    let mut f = web::block(move || std::fs::File::create(path)).await?;

    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|e| anyhow!("multipart error: {:}", e))?;
        f = web::block(move || f.write_all(&data).map(|_| f))
            .await
            .map_err(|e| anyhow!("write file error: {:?}", e))?;
    }

    f.flush()?;
    info!("success write file: {:?}", filepath);

    Ok(filepath)
}
