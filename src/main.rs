use actix_web::{App, HttpServer, http, middleware,web};//
use actix_web::middleware::errhandlers::{ErrorHandlers};
use actix_files as afs;
use actix_web::client::Client;

mod config;
mod errors;
mod api;
mod cacher;
use config::{Proxy,Config};
pub static CONFIG_PATH: &str = "config.json";

/// 程序入口方法
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=trace");
    // env_logger::init();
    // 日志系统初始化
    //log4rs::init_file("log.yaml", Default::default()).unwrap();

    // 新建一个服务处理
    let mut sys = actix_rt::System::new("cps-server");

    let cnf = Config::new(CONFIG_PATH);
    // let cacher=Cacher::new(|path|{
    //     Config::new(path)
    // });
    // let cnf=cacher.get_value(CONFIG_PATH);
    println!("cnf={:?}",cnf );
    // let proxy_url = Url::parse(&"http://b2b321.366ec.net").unwrap();
    let addr = format!("0.0.0.0:{}", cnf.server.port);

    HttpServer::new(move || {
        App::new()
           // .wrap(middleware::Logger::default())
            // 启用默认压缩
            // .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new()
                .header("X-Version", "0.1")
                .header("sever", "cps-server")
                // 下面这一句会导致出错
                // .header("content-encoding", "gzip")
            )
            .wrap(
                ErrorHandlers::new()
                    .handler(http::StatusCode::INTERNAL_SERVER_ERROR, errors::render_500)
                    .handler(http::StatusCode::NOT_FOUND, errors::render_404)
            )
            // .configure(express::express_config)
            .configure(|scf: &mut web::ServiceConfig|{
                let conf = Config::new("./config.json");
                for p in conf.proxy {
                    let proxy_path = p.path;
                    let proxy_target = p.target;
                    scf.service(
                            // api 目录代理
                            web::scope(&proxy_path)
                                .data(Client::new())
                                .data(Proxy::new(&proxy_path, proxy_target))
                                .route("*", web::to_async(api::request))
                        );
                }
            })
            .configure(|scf: &mut web::ServiceConfig| {
    
                scf.service(
                    // api 目录代理
                    web::scope("/api")
                        .wrap(
                            middleware::DefaultHeaders::new()
                            .header("X-Version", "0.2")
                            .header("access-control-allow-headers", "*")
                            .header("access-control-allow-methods", "POST, GET")
                            .header("access-control-allow-origin", "*")
                        )
                        .data(Client::new())
                        // 查询
                        .route("", web::get().to_async(api::api_get))
                        .route("", web::post().to_async(api::api_post))
                );
            })
            
            // websocket route
            // .service(web::resource("/ws/").route(web::get().to(ws::ws_index)))
            .service(
                // static files
                afs::Files::new("/", &cnf.server.root_dir).index_file("index.html"),
                
            )
            
            
    })
    .bind(addr)?
    // .workers(1) // 指定启用的工作线程数
    .start();

    sys.run()
}
//  use std::thread;
//  use std::time::Duration;

//  use std::collections::HashMap;
// pub struct Cacher<T>
//      where T: Fn(u32) -> u32
//  {
//      calculation: T,
//      value: Option<u32>,
//  }

//   impl<T> Cacher<T>
//      where T: Fn(u32) -> u32
//  {
//      pub fn new(calculation: T) -> Self {
//          Cacher {
//              calculation,
//              value: None,
//          }
//      }

//      pub fn value(&mut self, arg: u32) -> u32 {
//          match self.value {
//              Some(v) => v,
//              None => {
//                  let v = (self.calculation)(arg);
//                  self.value = Some(v);
//                  v
//              },
//          }
//      }
//  }