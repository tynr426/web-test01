

// use actix_web::client::Client;
use actix_web::{web,Error, HttpResponse, HttpRequest};
use futures::{future::ok, Future, Stream};
//use super::response::Response;
mod response;
mod parameter;
use response::Response;
use parameter::Parameter;
use actix_web::client::Client;
use url::Url;
use crate::config::{Proxy};


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

// 处理get方式的请求
pub fn api_get(
    req: HttpRequest
) -> Box<dyn Future<Item = HttpResponse, Error = Error>> {
    println!("{:?}", req);
    
    // let api_res = distribute("ss", &client);

    Box::new(ok::<_, Error>(
        HttpResponse::Ok()
        .content_type("application/json")
        .body("api_res"),
    ))
}

// 处理post方式请求
pub fn api_post(req: HttpRequest,
    paylod: web::Payload
) -> impl Future<Item = HttpResponse, Error = Error> {
    println!("post {:?}", req);
    
    // req.body().from_err
    
    paylod.map_err(Error::from)
        .fold(web::BytesMut::new(), move |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, Error>(body)
         })
         .and_then(|body| {
            let x_str = std::str::from_utf8(&body);
            let p = Parameter::new(x_str.unwrap());
            let res = if p.verify_sign("react_secret") {
                distribute(p)
            }
            else{
                Response::get_error("签名错误")
            };

            // println!("{:?}", p.verify_sign());
            // let cli = client.clone();
            // let api_res = distribute("post", &cli);
            Ok(
                HttpResponse::Ok()
                .content_type("application/json")
                .json(res),
            )
         })
    
    
    

    // let api_res = distribute("post", &client);

    // Box::new(ok::<_, Error>(
    //     HttpResponse::Ok()
    //     .content_type("application/json")
    //     .json(api_res),
    // ))

    // Box::new(
    //     paylod.map_err(Error::from)
    //     .fold(web::BytesMut::new(), move |mut body, chunk| {
    //         body.extend_from_slice(&chunk);
    //         Ok::<_, Error>(body)
    //      })
    //      .and_then(|body| {
    //          format!("Body {:?}!", body);
    //          Ok(HttpResponse::Ok().finish())
    //      })
    // )
}

/// 接口分发
fn distribute(param: Parameter) -> Response {
    match param.method.as_str() {
        "ip.get" => {
            Response::get_success("ssd")
        },
        "sms.send" => {
             Response::get_success("ok")

        }
        _ => {
            Response::get_error(&format!("请求参数错误,未找到{}接口方法。", param.method))
        }
    }
}
