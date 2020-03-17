use serde::{Deserialize, Serialize};
use quick_xml::de::from_str;
use std::option::Option;
use std::result::Result;
use quick_xml::de::DeError;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use serde_json;
use actix_web::{HttpResponse};


fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("couldn't open {}", file_path),
    };
    let mut s = String::new();
    file.read_to_string(&mut s);
    s
}

pub struct Cache {
    sub_ids: HashSet<String>,
    s: String,
    lock: Mutex<usize>,
    store: String,
}

impl Cache {
    pub fn new(store: &str) -> Cache {
        let s = read_file(store);
        let ids: HashSet<&str> = s.split(",").collect();
        let mut set = HashSet::new();
        ids.iter().for_each(|x| {
            set.insert(x.to_string());
        });
        Cache {
            sub_ids: set,
            s: s,
            lock: Mutex::new(0),
            store: store.to_string(),
        }
    }

    pub fn exist(&mut self, id: &String) -> bool {
        self.lock.lock().unwrap();
        if self.sub_ids.contains(id) {
            true
        } else {
            self.s.push_str(",");
            self.s.push_str(id);
            // 文件写入
            let path = Path::new(self.store.as_str());
            let mut file = match File::open(&path) {
                Ok(file) => file,
                Err(_) => panic!("couldn't open {}", self.store.as_str()),
            };
            file.write(self.s.as_bytes());
            false
        }
    }
}


#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Xml {
    pub ToUserName: String,      //
    pub FromUserName: String,    // 消息的来源
    pub CreateTime: u64,         // 消息的创建时间
    pub MsgType: String,         // 公共的字段 msg类型


    pub Content: Option<String>, // 普通消息 内容是发送的内容
    pub MsgId: Option<String>,   // 普通消息 内容是msgId

    pub Event: Option<String>,    // 菜单点击消息
    pub EventKey: Option<String>, // 菜单点击事件的key
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConfigIteam {
    pub key: String,
    pub response: String,
}

lazy_static! {
    
    pub static ref keyword_res: HashMap<String, String> = {
        let data = read_file("/Users/candy/code/actix_test/robot_wx/src/resource/msg_response.json");
        let p : Vec<ConfigIteam> = serde_json::from_str(data.as_str()).unwrap();
        p.iter().map(|x| (x.key.to_owned(), x.response.to_owned())).collect()
    };

    pub static ref menue_res: HashMap<String, String> = {
        let data = read_file("/Users/candy/code/actix_test/robot_wx/src/resource/meu_response.json");
        let p : Vec<ConfigIteam> = serde_json::from_str(data.as_str()).unwrap();
        p.iter().map(|x| (x.key.to_owned(), x.response.to_owned())).collect()
    };

    pub static ref subscribe_res: String = read_file("/Users/candy/code/actix_test/robot_wx/src/resource/subscribe.conf");

    pub static ref menu: String =  read_file("/Users/candy/code/actix_test/robot_wx/src/resource/meu.json");
}

impl Xml {

    pub fn new(xml: &str) -> Option<Xml> {
        let res: Result<Xml, DeError> = from_str(xml);
        match res {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }

    pub fn is_subscribe(&self) -> bool {
        if !self.MsgType.eq("event") { return false; }
        match &self.Event {
            Some(v) => v.eq("subscribe"),
            None => false,
        }
    }

    pub fn is_common(&self) -> bool {
        self.MsgType.eq("text")
    }

    pub fn is_menu_click(&self) -> bool {
        if !self.MsgType.eq("event") { return false; }
        match &self.Event {
            Some(v) => v.eq("CLICK"),
            None => false,
        }
    }

    pub fn response(&self, op: fn(&str) -> actix_web::Result<HttpResponse>)
                                                 -> actix_web::Result<HttpResponse> {
        if self.is_subscribe() { //返回订阅公众号的默认提醒消息
            op(subscribe_res.as_str())
        } else if self.is_menu_click() { // 返回菜单点击的消息
            match &self.EventKey {
                Some(v) => { 
                    op("hello")
                 },
                None => { 
                    op("word")
                },
            }
        } else {
            op("hello")
        }
    }


}

