use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn say_hello() -> impl Responder {
    HttpResponse::Ok().body("Michael Hello World")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there! This is manual hello")
}

async fn index() -> impl Responder {
    "michael index"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer::new(|| {
    //     App::new()
    //         .service(say_hello)
    //         .service(echo)
    //         .route("/manual_route", web::get().to(manual_hello))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await
    HttpServer::new(|| {
        App::new().service(
            web::scope("michael")
                .service(say_hello)
                .service(echo)
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
