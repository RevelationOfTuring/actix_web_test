use actix_web::{web, HttpResponse};

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            // get请求
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            // head请求
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}
