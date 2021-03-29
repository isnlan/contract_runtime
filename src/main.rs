use crate::config::Config;
use actix_web::{middleware, App, HttpServer};
use envconfig::Envconfig;
use std::sync;

#[macro_use]
extern crate log;
#[macro_use]
extern crate anyhow;

mod config;
mod contract;
mod handler;
mod model;
mod repl;
mod service;
mod utils;

pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    // 设置日志打印格式
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} {} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                buf.default_styled_level(record.level()),
                record.module_path().unwrap_or("<unnamed>"),
                &record.args()
            )
        })
        .init();
    info!("env_logger initialized.");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logger();

    let config = Config::init_from_env().unwrap();
    info!("{:?}", config);

    let ctrl = handler::Controller {};
    let ctrl = sync::Arc::new(ctrl);
    HttpServer::new(move || {
        App::new()
            .data(ctrl.clone())
            .wrap(middleware::Logger::default())
            .configure(handler::app_config)
    })
    .bind(config.server_address)?
    .run()
    .await
}
