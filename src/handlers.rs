use actix_web::{get, post, web::Json, web::Path};

#[get("/")]
async fn index() -> Json<String> {
    Json("Hello World".to_string())
}

#[post("/data")]
async fn echo(req_body: Json<String>) -> Json<String> {
    req_body
}

#[get("/hello/{name}")]
async fn hello(name: Path<String>) -> Json<String> {
    Json(format!("Hello {name}"))
}
