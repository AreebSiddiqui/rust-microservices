use actix_web:: {web, App, HttpResponse, HttpServer, Responder};
use std::io;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

async fn index2 () -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}

use actix_web::get;
#[get("/hello")]
async fn index3() -> impl Responder {
    HttpResponse::Ok().body("Hey there")
}

#[actix_rt::main]
async fn main () -> io::Result<()>  {
    HttpServer::new(||{
        App::new()
        .route("/",web::get().to(index))
        .route("/again",web::get().to(index2))
        .service(index3)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}

