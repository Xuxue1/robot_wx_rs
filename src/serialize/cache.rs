use std::collections::HashSet;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;
use super::static_ob::read_file;


pub struct Cache {
    sub_ids: HashSet<String>,
    store: String,
}

impl Cache {
    pub fn new(store: &str) -> Cache {
        let s = read_file(store);
        let ids: HashSet<&str> = s.split("\n").collect();
        let mut set = HashSet::new();
        ids.iter().for_each(|x| {
            set.insert(x.to_string());
        });
        Cache {
            sub_ids: set,
            store: store.to_string(),
        }
    }

    pub fn exist(&mut self, id: &String) -> bool {
        if self.sub_ids.contains(id) {
            true
        } else {
            // 文件写入
            let path = Path::new(self.store.as_str());
            let mut file = match OpenOptions::new().append(true).open(path) {
                Ok(file) => file,
                Err(_) => panic!("couldn't open {}", self.store.as_str()),
            };
            match file.write_all(format!("{}\n", id).as_bytes()) {
                Ok(_) => (),
                Err(e) => panic!("Failed write! {:?}", e),
            };
            self.sub_ids.insert(id.to_owned());
            false
        }
    }
}