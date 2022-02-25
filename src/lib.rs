use actix_web::{get, web, App, HttpRequest, HttpServer, Result};
use serde::Serialize;

#[derive(Serialize)]
struct IndexResponse {
    message: String,
}

#[get("/")]
async fn index(req: HttpRequest) -> Result<web::Json<IndexResponse>> {
    let hello = req
        .headers()
        .get("hello")
        .and_then(|v| v.to_str().ok())
        .unwrap_or_else(|| "world");

    Ok(web::Json(IndexResponse {
        message: hello.to_owned(),
    }))
}

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }
    pub async fn run(&self) -> std::io::Result<()> {
        println!("Starting http server: 127.0.0.1:{}", self.port);
        HttpServer::new(|| App::new().service(index))
            .bind(("127.0.0.1", self.port))?
            .run()
            .await
    }
}
