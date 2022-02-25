use messages_actix::MessageApp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = MessageApp::new(8080);
    app.run().await
}
