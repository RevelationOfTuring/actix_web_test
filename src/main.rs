use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Michael Hello World")
}

#[post("/echo")]
async fn echo(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}
