use super::xml::*;
use super::static_ob::*;
use config::Config;
use config::File;
use log::info;


fn request_token(app_id: &str, appsecret: &str) -> String {
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

fn delete_menu(token: &str) -> bool {
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

fn create_menu(token: &str) -> bool {
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

pub fn modify_menu() -> bool {
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

#[cfg(test)]
mod tests {
    use crate::serialize::menu::modify_menu;

    #[test]
    fn test_modify_meu() {
        modify_menu();
    }
}