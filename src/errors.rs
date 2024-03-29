
use actix_web::middleware::errhandlers::{ErrorHandlerResponse};
use actix_web::{dev, http, Result};


pub fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error!"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

pub fn render_404<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    // println!("{}", "mpte");
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Not Found!"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

