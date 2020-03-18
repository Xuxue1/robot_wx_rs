use config::Config;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Mutex;
use super::cache::Cache;
use super::xml::*;


pub fn read_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => panic!("couldn't open {}", file_path),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => s,
        Err(e) => panic!("Failed read file! {:?}", e),
    }
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let mut settings = Config::default();
        settings
            .merge(config::File::with_name("conf/wx_robot.toml"))
            .unwrap();
        settings
    };
    pub static ref KEYWORD_RES: HashMap<String, (Regex, String)> = {
        let keyword_res_path: String = CONFIG.get("keyword_res").unwrap();
        let data = read_file(keyword_res_path.as_str());
        let p: Vec<ConfigIteam> = serde_json::from_str(data.as_str()).unwrap();
        p.iter()
            .map(|x| {
                let pattern = x.key.to_string();
                let regex = Regex::new(pattern.as_str()).unwrap();
                (
                    pattern.to_owned(),
                    (regex.to_owned(), x.response.to_owned()),
                )
            })
            .collect()
    };
    pub static ref MENUE_RES: HashMap<String, String> = {
        let menue_res_path: String = CONFIG.get("menue_res").unwrap();
        let data = read_file(menue_res_path.as_str());
        let p: Vec<ConfigIteam> = serde_json::from_str(data.as_str()).unwrap();
        p.iter()
            .map(|x| (x.key.to_owned(), x.response.to_owned()))
            .collect()
    };
    pub static ref SUBSCRIBE_RES: String = {
        let subscrib_path: String = CONFIG.get("subscrib_res").unwrap();
        read_file(subscrib_path.as_str())
    };
    pub static ref MENU: String = {
        let menu_path: String = CONFIG.get("menu").unwrap();
        read_file(menu_path.as_str())
    };
    pub static ref CACHE: Mutex<Cache> = {
        let cache_path: String = CONFIG.get("cache").unwrap();
        Mutex::new(Cache::new(cache_path.as_str()))
    };
}