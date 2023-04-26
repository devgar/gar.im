use axum::{
    extract::Path,
    response::{Redirect},
    routing::get,
    Router,
};

async fn index() -> &'static str {
    "Hello, world!"
}

async fn app() -> &'static str {
    "Hello, client!"
}

// TODO Change to not found
fn local(slug: String) -> String {
    format!("_/{slug}")
}

// TODO Change to shortcut
fn github<'a>(slug: String) -> String {
    format!("https://github.com/devgar/{slug}")
}

async fn path(Path(slug): Path<String>) -> Redirect {
    Redirect::permanent((
        // TODO Get from DB must be an Option or Result
        if slug == "" {
            local(slug)
        } else {
            github(slug)
        }
    ).as_str())
}


#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/", get(index))
        .route("/_/*slug", get(app))
        .route("/+/*slug", get(app))
        .route("/-/*slug", get(app))
        .route("/./*slug", get(app))
        .route("/*slug", get(path));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}