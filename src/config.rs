// #[deny(dead_code)]

use serde_derive::Deserialize;
use std::io::prelude::*;
use std::fs::File;

// 服务器信息配置
#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    // 运行端口
    pub port: i32,

    // 站点根目录
    pub root_dir: String,

    // 静态文件目录
    pub static_dir: Vec<String>,

    // 静态文件扩展类型
    pub static_ext: Vec<String>
}

// 代理信息配置
#[derive(Debug, Clone, Deserialize)]
pub struct Proxy {
    // 代理路径
    pub path: String,
    // 代理目录
    pub target: Vec<ProxyTarget>
}

/// 代理类功能实现
impl Proxy{

    /// 新建一个代理类对象
    pub fn new(p: &str, t: Vec<ProxyTarget>) -> Self {
        Proxy{
            path: String::from(p),
            target: t
        }
    }
}

/// 代理目标
#[derive(Debug, Clone, Deserialize)]
pub struct ProxyTarget {
    // 代理目标地址
    pub url: String,
    // 代理权重
    pub weights: i32
}

// 系统配置信息
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    // 配置标题
    pub title: String,
    // 服务器信息配置
    pub server: Server,
    // 代理配置
    pub proxy: Vec<Proxy>,

    pub markdown: Option<Markdown>
}

#[derive(Debug, Clone, Deserialize)]
pub struct Markdown {

    pub dir: Option<String>,
}

// 配置扩展
impl Config {

    // 创建Config并加载相应信息
    pub fn new(conf_path: &str) -> Self {
        // println!("{}", conf_path);

        // 判断传入的conf_path是否为空,如果为空就给定默认值 
        let file_path = if conf_path.len() == 0 {
            "config.toml"
        } else {
            conf_path
        };

        // 打开文件
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception: {}", file_path, e)
        };

        // 读取文件到字符串变量
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("Error Reading file:{}", e)
        };
       
        // println!("file path {:?}", file_path);

        // if file_path.ends_with(".json") {

        // }
        // else {
        //     // 使用toml载加配置信息到结构体中
        //     let tc: Config = toml::from_str(&str_val).unwrap();
        //         Self {
        //         title: tc.title,
        //         server: tc.server,
        //         proxy: tc.proxy
        //     }
        // }
        let jc: Config = serde_json::from_str(&str_val).unwrap();
        jc
    }
}