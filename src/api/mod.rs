

// use actix_web::client::Client;
use actix_web::{web, middleware,Error, HttpResponse, HttpRequest};
use futures::{future::ok, Future, Stream};
//use super::response::Response;
mod response;
mod parameter;
use response::Response;
use parameter::Parameter;
use actix_web::client::Client;



/// api接口配置
pub fn api_service_config(scf: &mut web::ServiceConfig) {
    
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
            .route("", web::get().to_async(api_get))
            .route("", web::post().to_async(api_post))
    );
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
