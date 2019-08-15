use actix_web::{App, HttpServer, http, middleware,web, Error, HttpResponse, HttpRequest};//
use actix_web::middleware::errhandlers::{ErrorHandlers};
use actix_files as afs;
use actix_web::client::Client;
use futures::Future;
use url::Url;
use crate::config::{Proxy, Config};
mod proxy;
mod config;
mod errors;
mod api;

pub static CONFIG_PATH: &str = "config.json";

/// 程序入口方法
fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=trace");
    // env_logger::init();
    // 日志系统初始化
    //log4rs::init_file("log.yaml", Default::default()).unwrap();

    // 新建一个服务处理
    let sys = actix_rt::System::new("cps-server");

    let cnf = config::Config::new(CONFIG_PATH);
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
                                .route("*", web::to_async(proxy::request))
                        );
                }
            })
            .configure(api::api_service_config)
            
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
    // HttpServer::new(|| {
    //     App::new()
    //         // enable logger
    //         .wrap(middleware::Logger::default())
    //         // 启用默认压缩
    //         .wrap(middleware::Compress::default())
    //         .configure(proxy::proxy_config)
    //         .configure(express::express_config)
    //         .service(
    //             // static files
    //             fs::Files::new("/", &cnf.server.root_dir).index_file("index.html"),
    //         )
    //         // .service(web::resource("/").to(index))
    //         // async handler
    //         .service(
    //             web::resource("/async/{name}").route(web::get().to_async(index_async)),
    //         )
    //         .route(r"/a/{name}", web::get().to(index_json))
    //         // // async handler
    //         // .service(
    //         //     web::resource("/async-body/{name}")
    //         //         .route(web::get().to(index_async_body)),
    //         // )
    // })
    // .bind("127.0.0.1:8080")?
    // .run()
}
