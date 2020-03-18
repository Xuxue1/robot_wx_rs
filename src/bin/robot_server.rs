use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Result};
use bytes::Bytes;
use env_logger;
use std::collections::HashMap;
use std::env;
use std::str;
use std::thread;
use wx_robot::serialize::xml::*;
use wx_robot::serialize::static_ob::*;
use wx_robot::serialize::menu::*;
use log::info;

#[get("/api/response")]
async fn response_token(req: HttpRequest) -> Result<HttpResponse> {
    let query_string: Vec<&str> = req.query_string().split(|x| x == '=' || x == '&').collect();
    let mut query_string_map = HashMap::new();
    let mut index = 0;
    while index + 2 <= query_string.len() {
        query_string_map.insert(query_string[index], query_string[index + 1]);
        index += 2;
    }
    let s: &str = match query_string_map.get("echostr") {
        Some(v) => v,
        None => "",
    };
    Ok(HttpResponse::Ok()
        .content_type("text/plain")
        .body(format!("{}", s)))
}

#[post("/api/response")]
async fn response_msg(body: Bytes) -> Result<HttpResponse> {
    let xml_str = match str::from_utf8(body.as_ref()) {
        Ok(v) => v,
        Err(_) => "<xml></xml>",
    };
    info!("Recive:\n {}", xml_str);
    let xml = Xml::new(xml_str).unwrap();
    xml.response(|x| {
        info!("Response:\n {}", x);
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("{}", x)))
    })
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info,info");
    env_logger::init();
    let host: String = CONFIG.get("host").unwrap();
    let port: u32 = CONFIG.get("port").unwrap();
    thread::spawn(move || {
        modify_menu();
    });
    HttpServer::new(|| App::new().service(response_token).service(response_msg))
        .bind(format!("{}:{}", host, port))?
        .run()
        .await
}
