// use actix_files as fs; https://github.com/vascokk/fullstack-rust
use actix_web::middleware::Logger;
use actix_web::{
    get, post, put, web, web::scope, App, HttpResponse, HttpServer, Responder, Result,
};
use actix_web_lab::web::spa;
use serde::Serialize;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[derive(Serialize)]
struct MyObj {
    name: String,
}
#[get("/project")]
async fn get_projects() -> Result<impl Responder> {
    let obj = MyObj {
        name: "my name".to_string(),
    };
    Ok(web::Json(obj))
}
#[post("/project")]
async fn create_project() -> Result<impl Responder> {
    let obj = MyObj {
        name: "my name".to_string(),
    };
    Ok(web::Json(obj))
}
#[put("/project")]
async fn edit_project() -> Result<impl Responder> {
    let obj = MyObj {
        name: "my name".to_string(),
    };
    Ok(web::Json(obj))
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
    let server = HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(
                scope("/api")
                    .service(hello_world)
                    .service(get_projects)
                    .service(create_project)
                    .service(edit_project),
            )
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
    .run();
    println!("Server running at 0.0.0.0:8080");
    server.await
}
