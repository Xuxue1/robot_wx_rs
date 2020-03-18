use quick_xml::de::{from_str, DeError};
use serde::{Deserialize, Serialize};
use std::option::Option;
use std::result::Result;
use std::time::{SystemTime, UNIX_EPOCH};
use super::static_ob::*;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ConfigIteam {
    pub key: String,
    pub response: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DeleteMeuResponse {
    pub errcode: u32,
    pub errmsg: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SetMeuResponse {
    pub errcode: u32,
    pub errmsg: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Xml {
    #[serde(rename = "ToUserName", default)]
    pub to_user_name: String, // 消息的目的地
    #[serde(rename = "FromUserName", default)]
    pub from_user_name: String, // 消息的来源
    #[serde(rename = "CreateTime", default)]
    pub create_time: u64, // 消息的创建时间
    #[serde(rename = "MsgType", default)]
    pub msg_type: String, // 公共的字段 msg类型
    #[serde(rename = "Content", default)]
    pub content: Option<String>, // 普通消息 内容是发送的内容
    #[serde(rename = "MsgId", default)]
    pub msg_id: Option<String>, // 普通消息 内容是msgId
    #[serde(rename = "Event", default)]
    pub event: Option<String>, // 菜单点击消息
    #[serde(rename = "EventKey", default)]
    pub event_key: Option<String>, // 菜单点击事件的key
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
        if !self.msg_type.eq("event") {
            return false;
        }
        match &self.event {
            Some(v) => v.eq("subscribe"),
            None => false,
        }
    }

    pub fn is_common(&self) -> bool {
        self.msg_type.eq("text")
    }

    pub fn is_menu_click(&self) -> bool {
        if !self.msg_type.eq("event") {
            return false;
        }
        match &self.event {
            Some(v) => v.eq("CLICK"),
            None => false,
        }
    }

    pub fn subscribe_res(&self) -> String {
        let now = SystemTime::now();
        format!(
            r#"<xml>
                <ToUserName><![CDATA[{}]]></ToUserName>
                <FromUserName><![CDATA[{}]]></FromUserName>
                <CreateTime>{}</CreateTime>
                <MsgType><![CDATA[text]]></MsgType>
               <Content><![CDATA[{}]]></Content>
             </xml>"#,
                self.from_user_name,
                self.to_user_name,
                now.elapsed().unwrap().as_secs(),
                SUBSCRIBE_RES.as_str())
    }

    pub fn menu_click(&self) -> String {
        let event_key_ref = self.event_key.as_ref();
        if event_key_ref.is_some() {
            let key = event_key_ref.unwrap();
            let msg = MENUE_RES.get(key).unwrap();
            let now = SystemTime::now();
            format!(
                r#"<xml>
                        <ToUserName><![CDATA[{}]]></ToUserName>
                        <FromUserName><![CDATA[{}]]></FromUserName>
                        <CreateTime>{}</CreateTime>
                        <MsgType><![CDATA[text]]></MsgType>
                        <Content><![CDATA[{}]]></Content>
                  </xml>"#,
                self.from_user_name,
                self.to_user_name,
                now.elapsed().unwrap().as_secs(),
                msg
            )
        } else {
            "".to_string()
        }
    }

    pub fn now(&self) -> u64 {
        let now = SystemTime::now();
        now.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }

    pub fn pic_and_word_msg(&self) -> String {
        format!(
            r#"
            <xml>
                <ToUserName><![CDATA[{}]]></ToUserName>
                <FromUserName><![CDATA[{}]]></FromUserName>
                <CreateTime>{}</CreateTime>
                <MsgType><![CDATA[news]]></MsgType>
                <ArticleCount>1</ArticleCount>
                <Articles>
                    <item>
                    <Title><![CDATA[（2.12 更新）知乎热门：免费大学教材 PDF 哪里找？]]></Title>
                    <Description><![CDATA[快开学了，各种资料答案走起~]]></Description>
                    <PicUrl><![CDATA[http://mmbiz.qpic.cn/mmbiz_jpg/TS8rulAuOdKFq1HV4FkXkWE731UKq922h3Qqt0UpIWyr6rDauyfOwOu9JVPOjzXp3I0vZmLsrrgBuvNcbK0TFg/0?wx_fmt=jpeg]]></PicUrl>
                    <Url><![CDATA[https://mp.weixin.qq.com/s/J95beHEQHAwQ6U2l8XwfHA]]></Url>
                    </item>
                </Articles>
            </xml>"#, self.from_user_name, self.to_user_name, self.now())
    }

    pub fn word_msg(&self, word: &str) -> String {
        format!(
            r#"
                <xml>
                    <ToUserName><![CDATA[{}]]></ToUserName>
                    <FromUserName><![CDATA[{}]]></FromUserName>
                    <CreateTime>{}</CreateTime>
                    <MsgType><![CDATA[text]]></MsgType>
                    <Content><![CDATA[{}]]></Content>
                </xml>
            "#,
            self.from_user_name,
            self.to_user_name,
            self.now(),
            word)
    }

    pub fn keyword_res(&self) -> String {
        let content_ref = self.content.as_ref();
        if content_ref.is_some() {
            let content = self.content.as_ref();
            let matchs: Option<&str> = KEYWORD_RES
                .iter()
                .find(|(_, v)| v.0.is_match(content.unwrap().as_str()))
                .map(|(_, v)| v.1.as_str());
            if matchs.is_some() {
                let res_content = matchs.unwrap();
                if res_content.eq("https://mp.weixin.qq.com/s/T7ol72aZHHrXRluWXm5srA") {
                    return self.pic_and_word_msg()
                } else {
                    return self.word_msg(res_content)
                }
            } else {
                let mut cache = CACHE.lock().unwrap();
                if !cache.exist(&self.from_user_name) {
                    return self.word_msg("需要教材电子版的话，看看咱们链接里的方法，能不能找到呀");
                }
            }
        }
        String::from("")
    }

    pub fn response<T>(&self, op: fn(&str) -> T) -> T {
        if self.is_subscribe() {
            //返回订阅公众号的默认提醒消息
            op(self.subscribe_res().as_str())
        } else if self.is_menu_click() {
            // 返回菜单点击的消息
            op(self.menu_click().as_str())
        } else {
            //普通消息
            op(self.keyword_res().as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::serialize::xml::Xml;

    #[test]
    fn test_subscribe() {
        let xml_str = r#"<xml><ToUserName><![CDATA[gh_5d0bbcfa8ae9]]></ToUserName>
        <FromUserName><![CDATA[o2_HDv95IYQmdT0NTRb0SldRGHso]]></FromUserName>
        <CreateTime>1584513594</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[subscribe]]></Event>
        <EventKey><![CDATA[]]></EventKey>
        </xml>"#;
        let xml: Xml = Xml::new(xml_str).unwrap();
        let s: String = xml.response(|x| {
            String::from(x)
        });
        println!("{}", s);
    }

    #[test]
    fn test_menu() {
        let xml_str = r#"
        <xml><ToUserName><![CDATA[gh_5d0bbcfa8ae9]]></ToUserName>
        <FromUserName><![CDATA[o2_HDv2b0MWJHmskA6GRFeAkjKiA]]></FromUserName>
        <CreateTime>1583245678</CreateTime>
        <MsgType><![CDATA[event]]></MsgType>
        <Event><![CDATA[CLICK]]></Event>
        <EventKey><![CDATA[V1001]]></EventKey>
        </xml>
        "#;
        let xml: Xml = Xml::new(xml_str).unwrap();
        let s: String = xml.response(|x| {
            String::from(x)
        });
        println!("{}", s);
    }

    #[test]
    fn test_keyword() {
        let xml_str = r#"
        <xml><ToUserName><![CDATA[gh_5d0bbcfa8ae9]]></ToUserName>
        <FromUserName><![CDATA[o2_HDv8IxHgKd4_mFo_BLjHjaCqE]]></FromUserName>
        <CreateTime>1583245742</CreateTime>
        <MsgType><![CDATA[text]]></MsgType>
        <Content><![CDATA[asasasasa]]></Content>
        <MsgId>22666629054217964</MsgId>
        </xml>
        "#;
        let xml: Xml = Xml::new(xml_str).unwrap();
        let s: String = xml.response(|x| {
            String::from(x)
        });
        println!("{}", s);
    }



}