
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Response {
    pub code: i32,
    pub content: String,
    pub format: String,
    pub message: String,
    pub success: bool
}

impl Response {
    
    // 获取错误输出对象
    pub fn get_error(msg: &str) -> Self{
        Response {
            code: 4001,
            content: String::new(),
            format: String::from("json"),
            message: String::from(msg),
            success: false
        }
    }

    /// 获取成功的Response
    pub fn get_success(content: &str) -> Self {
        Response {
            code: 0,
            content: content.to_owned(),
            format: String::from("json"),
            message: String::new(),
            success: true
        }
    }
}