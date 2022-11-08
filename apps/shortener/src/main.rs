use actix_web::{get, App, HttpResponse, HttpServer, Responder, HttpRequest, http};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[get("/{slug:.*}")]
async fn redirect(req: HttpRequest) -> impl Responder {
    let slug: String = req.match_info().get("slug").unwrap().parse().unwrap();
    println!("REQUESTED SLUG: ::/{}::", slug);

    if slug == "fail" {
        HttpResponse::NotFound()
        .body(format!("404 NOT FOUND\n    REQUESTED SLUG ::/{}:: DOES NOT EXIST", slug))
    } else {
        HttpResponse::Found()
        .append_header((http::header::LOCATION, "https://github.com/devgar"))
        .finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(redirect)
        })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}