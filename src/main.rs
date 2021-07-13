extern crate cronjob;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;

use std::path::PathBuf;
use std::{env, thread};

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_static_files::resource_dir;
use cronjob::CronJob;
use diesel::prelude::*;
use dotenv::dotenv;
use human_panic::setup_panic;
use log::{error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::Config;

use schema::speed;

use crate::models::{SpeedInsert, SpeedQuery};
use crate::speed_engine::SpeedEngine;

mod db;
mod errors;
mod models;
mod schema;
mod speed_engine;

embed_migrations!("./migrations");

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_panic!();

    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(Logger::builder().build("speedtest_rs::speedtest", LevelFilter::Warn))
        .logger(Logger::builder().build("speed_monitor", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();
    log4rs::init_config(config).unwrap();

    info!("booting up");

    let connection = db::establish_connection();
    embedded_migrations::run_with_output(&connection, &mut std::io::stdout());

    thread::spawn(move || {
        let mut cron = CronJob::new("speed test cron job", test_and_save);
        cron.seconds("0");
        cron.minutes("0");
        cron.start_job();
    });

    dotenv().ok();

    let web_port = env::var("WEB_PORT").unwrap_or("32001".to_string());
    info!("starting web server at port:{}", web_port);

    HttpServer::new(|| {
        let generated = generate();

        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .route("/test", web::get().to(start_test))
            .route("/health", web::get().to(health))
            .route("/data", web::get().to(data))
            .service(actix_web_static_files::ResourceFiles::new("/", generated))
    })
    .bind((format!("{}:{}", "0.0.0.0", web_port)))?
    .run()
    .await
}

async fn start_test() -> impl Responder {
    test_and_save("from web");
    "Ok"
}

async fn health() -> impl Responder {
    "Server is alive!"
}

async fn data() -> impl Responder {
    use schema::speed::dsl::*;

    let connection = db::establish_connection();
    let results = speed
        .order(id.desc())
        .limit(10)
        .load::<SpeedQuery>(&connection)
        .expect("Error loading speed result");

    web::Json(results)
}

fn test_and_save(name: &str) {
    info!("trigger a test...");

    let connection = db::establish_connection();

    let engine = SpeedEngine {};

    let result = engine.test();

    info!("get result:{:?}", result);

    match result {
        Ok(result) => {
            let new_speed = SpeedInsert {
                download: result.download_band_width as i32,
                upload: result.upload_band_width as i32,
            };

            diesel::insert_into(speed::table)
                .values(&new_speed)
                .execute(&connection)
                .unwrap();

            info!("save data")
        }
        Err(err) => {
            error!("test fail with {:?}", err)
        }
    }
}
