use std::sync::Mutex;
mod app;
mod test;

use actix_web::{get, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

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

    // // readable state
    // HttpServer::new(|| {
    //     App::new()
    //         .app_data(web::Data::new(AppState {
    //             app_name: "michael's app".into(),
    //             version: 1,
    //         }))
    //         .service(
    //             web::scope("michael")
    //                 .service(say_hello)
    //                 .service(echo)
    //                 .route("/index.html", web::get().to(index))
    //                 .service(get_app_info),
    //         )
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    // // // shared mutable state
    // // 1. created _outside_ HttpServer::new closure
    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });

    // // 2. move counter into the closure
    // HttpServer::new(move || {
    //     App::new()
    //         // register the created data
    //         .app_data(counter.clone())
    //         .service(increase)
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    // multiple scopes
    // HttpServer::new(|| {
    //     App::new()
    //         .service(web::scope("/abc").service(s11).service(s12))
    //         .service(web::scope("/efg").service(s21).service(s22))
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    // configure
    // HttpServer::new(|| {
    //     App::new()
    //         .configure(app::config)
    //         .service(web::scope("/api").configure(test::scoped_config))
    //         .route(
    //             "/",
    //             web::get().to(|| async { HttpResponse::Ok().body("/") }),
    //         )
    // })
    // .bind(("127.0.0.1", 8080))?
    // .run()
    // .await

    HttpServer::new(|| App::new().service(info))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[derive(Deserialize)]
struct Info {
    age: u32,
    name: String,
    b: bool,
}

// http://127.0.0.1:8080/michael?age=1024&name=12a3&b=false
#[get("/michael")]
async fn info(info: web::Query<Info>) -> String {
    format!("{} {} {}", info.name, info.age, info.b)
}

// non-type-safe alternative, it's also possible to query
// #[get("/michael/{age}/{name}")]
// async fn info11(req: HttpRequest) -> Result<String> {
//     let age = req.match_info().get("age").unwrap().parse::<u8>().unwrap();
//     let name = req
//         .match_info()
//         .get("name")
//         .unwrap()
//         .parse::<String>()
//         .unwrap();

//     Ok(format!("name:{name} age:{age}"))
// }
// //  to extract path information to a type that implements the Deserialize trait from serde
// #[derive(Deserialize)]
// struct Info {
//     name: String,
//     age: u8,
// }

// #[get("/michael/{age}/{name}")]
// async fn info(path: web::Path<Info>) -> Result<String> {
//     Ok(format!("{}--{}", path.name, path.age))
// }

// path
// #[get("michael/{user_id}/{name}/{b}")]
// async fn info(path: web::Path<(u32, String, bool)>) -> Result<String> {
//     let (a, b, c) = path.into_inner();
//     Ok(format!("==={a} {b} {c}==="))
// }

// #[get("/s11")]
// async fn s11() -> String {
//     "s11".into()
// }

// #[get("/s12")]
// async fn s12() -> String {
//     "s12".into()
// }

// #[get("/s21")]
// async fn s21() -> String {
//     "s21".into()
// }

// #[get("/s22")]
// async fn s22() -> String {
//     "s22".into()
// }

// // shared mutable state
// struct AppStateWithCounter {
//     counter: Mutex<u32>,
// }

// #[get("/inc_counter")]
// async fn increase(data: web::Data<AppStateWithCounter>) -> String {
//     let mut counter = data.counter.lock().unwrap();
//     *counter += 1;
//     format!("counter now: {counter}")
// }

// state
// struct AppState {
//     app_name: String,
//     version: u8,
// }

// #[get("/get_app_info")]
// async fn get_app_info(data: web::Data<AppState>) -> String {
//     let app_state = &data;
//     format!(
//         "app name: {}, app version: {}",
//         app_state.app_name, app_state.version
//     )
// }
