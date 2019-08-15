
use actix_web::client::Client;
use actix_web::{web, Error, HttpResponse, HttpRequest};
use futures::Future;
use url::Url;
use crate::config::{Proxy, Config};

/// 给定代理配置
pub fn proxy_config(scf: &mut web::ServiceConfig) {
    
    let conf = Config::new("./config.json");
    for p in conf.proxy {
        let proxy_path = p.path;
        let proxy_target = p.target;
        scf.service(
                // api 目录代理
                web::scope(&proxy_path)
                    .data(Client::new())
                    .data(Proxy::new(&proxy_path, proxy_target))
                    .route("*", web::to_async(request))
            );
    }
}


/// 代理请求处理
pub fn request(
    req: HttpRequest,
    pr: web::Data<Proxy>,
    payload: web::Payload,
    client: web::Data<Client>
) -> impl Future<Item = HttpResponse, Error = Error> {

    // 代理请求目标主机集群
    let targets = &pr.target;
    // 获取第一个目标主机
    let target_url = &targets[0];
    // 创建一个可变的url地址
    let mut new_url = Url::parse(&target_url.url).unwrap();
    
    let url_path = req.uri().path();    
    let start_index = pr.path.len();
    
    // 设置请求的路径,并去掉代理目录前缀
    new_url.set_path(&url_path[start_index..]);
    new_url.set_query(req.uri().query());

    // println!("{:?}", new_url);
    // 从源请求中把头部信息设置到客户端对象上
    let proxy_req = client
        .request_from(new_url.as_str(), &req.head())
        .no_decompress();
    // 绑定远程ip实现获取到准确ip
    let proxy_req = if let Some(addr) = req.head().peer_addr {
        proxy_req.header("x-forwarded-for", format!("{}", addr.ip()))
    } else {
        proxy_req
    };

    // proxy_req.

    // 代理请求数据并返回前端
    proxy_req
        .send_stream(payload)//发送源请求的源数据
        .map_err(Error::from)
        .map(|res| {
            let mut client_resp = HttpResponse::build(res.status());
            for (header_name, header_value) in res
                .headers()
                .iter()
                .filter(|(h, _)| *h != "connection" && *h != "content-length")
            {
                client_resp.header(header_name.clone(), header_value.clone());
            }
            client_resp.streaming(res)
        })

}
