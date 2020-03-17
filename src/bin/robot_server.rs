use actix_web::{get, post, HttpResponse, Result, HttpRequest};
use bytes::{Bytes};
use std::collections::HashMap;
use std::str;
use quick_xml::de::from_str;
use wx_robot::serialize::Xml;
use std::error::Error;



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

async fn modify_menu() -> () {
    // 1. 请求access_token
    // 3. 更新菜单
    let app_id = "";
    let appsecret = "";
    let get_token_url = format!("https://api.weixin.qq.com/cgi-bin/token?grant_type=client_credential&appid={}&secret={}", app_id, appsecret);
    let token_map = reqwest::blocking::get(get_token_url.as_str()).unwrap()
                                            .json::<HashMap<String, String>>().unwrap();
    let token = token_map.get("access_token").unwrap();
    // 2. 删除菜单
    let delete_token_url = format!("https://api.weixin.qq.com/cgi-bin/menu/delete?access_token={}", token);
    let delete_res = reqwest::blocking::get(delete_token_url.as_str()).unwrap()
                                            .json::<HashMap<String, String>>().unwrap();
    let set_menu_url = format!("https://api.weixin.qq.com/cgi-bin/menu/create?access_token={}", token);
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
                    .body("the exact body that is sent")
                    .send()
                    .await;
}


// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//    HttpServer::new(|| 
//     App::new()
//     .service(response_token)
//     .service(response_msg))
//        .bind("127.0.0.1:8080")?
//        .run().await
// }

fn main() {
//     let x = "<xml>
//     <ToUserName><![CDATA[toUser]]></ToUserName>
//     <FromUserName><![CDATA[FromUser]]></FromUserName>
//     <CreateTime>123456789</CreateTime>
//     <MsgType><![CDATA[event]]></MsgType>
//     <Event><![CDATA[subscribe]]></Event>
//   </xml>";
//   println!("{}", wx_robot::serialize::menu.as_str());
//   let xml: Xml = from_str(x).unwrap();
//   xml.response();
  //println!("{}", xml.response());
  env_logger::init();
  let mut res = reqwest::blocking::get("https://www.rust-lang.org/").unwrap();
  res.copy_to(&mut std::io::stdout()).unwrap();
}