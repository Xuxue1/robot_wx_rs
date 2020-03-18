use actix_web::{get, post, App, HttpRequest, HttpResponse, HttpServer, Result};
use bytes::Bytes;
use config::Config;
use config::File;
use env_logger;
use log::info;
use std::collections::HashMap;
use std::env;
use std::str;
use std::thread;
use wx_robot::serialize::xml::CONFIG;
use wx_robot::serialize::xml::MENU;
use wx_robot::serialize::xml::{DeleteMeuResponse, SetMeuResponse, TokenResponse, Xml};

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
    let xml = Xml::new(xml_str).unwrap();
    xml.response(|x| {
        Ok(HttpResponse::Ok()
            .content_type("text/plain")
            .body(format!("{}", x)))
    })
}

pub fn request_token(app_id: &str, appsecret: &str) -> String {
    let client = reqwest::blocking::Client::new();
    let get_token_url = format!(
        "https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}",
        app_id, appsecret
    );
    let token_res: TokenResponse = client
        .get(get_token_url.as_str())
        .send()
        .unwrap()
        .json()
        .unwrap();
    token_res.access_token
}

pub fn delete_menu(token: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let delete_token_url = format!(
        "https://api.weixin.qq.com/cgi-bin/menu/delete?access_token={}",
        token
    );
    let delete_res: DeleteMeuResponse = client
        .get(delete_token_url.as_str())
        .send()
        .unwrap()
        .json()
        .unwrap();
    delete_res.errcode == 0
}

pub fn create_menu(token: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let set_menu_url = format!(
        "https://api.weixin.qq.com/cgi-bin/menu/create?access_token={}",
        token
    );
    let set_menu_res: SetMeuResponse = client
        .post(set_menu_url.as_str())
        .body(MENU.as_str())
        .send()
        .unwrap()
        .json()
        .unwrap();
    set_menu_res.errcode == 0
}

fn modify_menu() -> bool {
    let mut settings = Config::default();
    settings
        .merge(File::with_name("conf/wx_robot.toml"))
        .unwrap();
    let app_id: String = settings.get("app_id").unwrap();
    let appsecret: String = settings.get("appsecret").unwrap();
    let token = request_token(app_id.as_str(), appsecret.as_str());
    info!("Request token: {} success.", token);
    delete_menu(token.as_str());
    info!("Delete menu success.");
    create_menu(token.as_str());
    info!("Create menue success.");
    true
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

#[cfg(test)]
mod tests {
    use crate::create_menu;
    use crate::delete_menu;
    use crate::modify_menu;
    use crate::request_token;
    use config::Config;
    use config::File;

    #[test]
    fn test_config() {
        let mut settings = Config::default();
        settings
            .merge(File::with_name("conf/wx_robot.toml"))
            .unwrap();
        let d: bool = settings.get("debug").unwrap();
        let path: String = settings.get("path").unwrap();
        assert_eq!(d, false);
        assert_eq!(path, "hahahaha");
        println!("{}", d);
        println!("{}", path);
    }

    #[test]
    fn test_request_token() {
        let mut settings = Config::default();
        settings
            .merge(File::with_name("conf/wx_robot.toml"))
            .unwrap();
        let app_id: String = settings.get("app_id").unwrap();
        let appsecret: String = settings.get("appsecret").unwrap();
        let token = request_token(app_id.as_str(), appsecret.as_str());
        println!("token: {}", token);
    }

    #[test]
    fn test_delete_meu() {
        let mut settings = Config::default();
        settings
            .merge(File::with_name("conf/wx_robot.toml"))
            .unwrap();
        let app_id: String = settings.get("app_id").unwrap();
        let appsecret: String = settings.get("appsecret").unwrap();
        let token = request_token(app_id.as_str(), appsecret.as_str());
        assert_eq!(delete_menu(token.as_str()), true);
    }

    #[test]
    fn test_set_meu() {
        let mut settings = Config::default();
        settings
            .merge(File::with_name("conf/wx_robot.toml"))
            .unwrap();
        let app_id: String = settings.get("app_id").unwrap();
        let appsecret: String = settings.get("appsecret").unwrap();
        let token = request_token(app_id.as_str(), appsecret.as_str());
        assert_eq!(delete_menu(token.as_str()), true);
        assert_eq!(create_menu(token.as_str()), true);
    }

    #[test]
    fn test_modify_meu() {
        modify_menu();
    }
}
