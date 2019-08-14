use serde::{Deserialize, Serialize};
use std::collections::{HashMap, BTreeMap};

/// 接口请求参数
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Parameter {

    // 请求的方法
    pub method: String,
    // 请求时间戳
    pub timestamp: String,
    // 应用Id
    pub app_id: String,
    // 签名数据
    pub sign: String,
    // 版本
    pub version: String,

    pub properties: Option<HashMap<String, String>>,

    origin: HashMap<String, String>,

    keys: BTreeMap<String, String>
}

/// 获取给定对象的值
fn get_map_value(keys_map: BTreeMap<String, String>, values_map: HashMap<String, String>, key: &str) -> String {
    let lower_key = key.to_lowercase();
    match keys_map.get(&lower_key) {
        Some(key) => {
            match values_map.get(key) {
                Some(val) => val.to_owned(),
                None => String::new()
            }
        },
        None => String::new()
    }
}

impl Parameter {

    pub fn new(form_str: &str) -> Self {
        let mut value_maps = HashMap::new();
        let mut key_maps = BTreeMap::new();

        for xs in String::from(form_str).split("&") {
            let xxs: Vec<&str> = xs.split("=").collect();
            value_maps.insert(xxs[0].to_owned(), xxs[1].to_owned());
            key_maps.insert(xxs[0].to_lowercase(), xxs[0].to_owned());
        }

        let mut ps = value_maps.clone();
        ps.remove("method");
        ps.remove("timestamp");
        ps.remove("appid");
        ps.remove("sign");
        ps.remove("version");

        Parameter {
            method: get_map_value(key_maps.clone(), value_maps.clone(), "method"),
            timestamp: get_map_value(key_maps.clone(), value_maps.clone(), "timestamp"),
            app_id: get_map_value(key_maps.clone(), value_maps.clone(), "appid"),
            sign: get_map_value(key_maps.clone(), value_maps.clone(), "sign"),
            version: get_map_value(key_maps.clone(), value_maps.clone(), "version"),
            properties: Some(ps),
            origin: value_maps,
            keys: key_maps
        }
    }

    pub fn get_value(&self, key: &str) -> String {
        let keys = self.keys.clone();
        let values = self.origin.clone();
        get_map_value(keys, values, key)
    }

    /// 统一判断签名信息
    pub fn verify_sign(&self, secret: &str) -> bool{
        let mut sign_strs: Vec<String> = vec![];
        let mut sign = "";
        let key_maps = &self.keys;
        let maps = &self.origin;
        for (key, value) in key_maps {
            let skey = value;
            let val = maps.get(skey);
            if key == "sign" {
                sign = val.unwrap();
            }
            else{
                let mut st = String::new();
                st.push_str(skey);
                st.push_str("=");
                st.push_str(val.unwrap());
                sign_strs.push(st);
            }
        }

        let mut x_sign: String = sign_strs.join("&");
        x_sign.push_str("&secret=");
        x_sign.push_str(secret);

        // println!("{}", x_sign);
        let digest = md5::compute(x_sign.as_str().as_bytes());
        let new_sign = format!("{:x}", digest);
        // println!("new: {:?}, old: {:?}", new_sign, sign);
        new_sign == sign
    }
}