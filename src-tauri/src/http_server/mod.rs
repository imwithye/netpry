use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use tokio::task;
use tokio::task::JoinHandle;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub fn run_http_server(addr: &'static str) -> JoinHandle<()> {
    task::spawn(async move {
        let http_server = HttpServer::new(|| App::new().service(hello));
        http_server.bind(addr).unwrap().run().await.unwrap();
    })
}
