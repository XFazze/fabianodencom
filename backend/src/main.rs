// use actix_files as fs; https://github.com/vascokk/fullstack-rust
use actix_web::middleware::Logger;
use actix_web::{get, web::scope, App, HttpResponse, HttpServer, Responder};
use actix_web_lab::web::spa;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(scope("/api").service(home))
            // .service(fs::Files::new("/", "./dist").index_file("./dist/index.html")) https://github.com/vascokk/fullstack-rust
            .service(
                spa()
                    .index_file("./dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
