use actix_web::{web, App, HttpResponse, HttpServer};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            // get请求
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            // head请求
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}