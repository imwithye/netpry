use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use reqwest::{header::HeaderMap, header::HeaderName, header::HeaderValue, Client};
use std::error::Error;
use tokio::task;
use tokio::task::JoinHandle;

async fn proxy(
    req: HttpRequest,
    mut payload: web::Payload,
) -> Result<HttpResponse, Box<dyn Error>> {
    let client = Client::new();

    let uri = format!("https://google.com{}", req.uri());
    let method = req.method().as_str().parse()?;
    let mut forward_req = client.request(method, &uri);
    let mut headers = HeaderMap::new();
    for (name, value) in req.headers().iter() {
        let header_name: HeaderName = name.as_str().parse()?;
        let header_value: HeaderValue = HeaderValue::from_bytes(value.as_bytes())?;
        headers.insert(header_name, header_value);
    }
    if let Some(chunk) = payload.next().await {
        forward_req = forward_req.body(chunk?);
    }

    let resp = forward_req
        .send()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    let status: actix_web::http::StatusCode = resp.status().as_str().parse()?;
    let mut client_resp = HttpResponse::build(status);
    for (name, value) in resp.headers().iter() {
        let header_name: &str = name.as_str();
        let header_value: &[u8] = value.as_bytes();
        client_resp.append_header((header_name, header_value));
    }
    let body = resp
        .bytes()
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(client_resp.body(body))
}

pub fn run_http_server(addr: &'static str) -> JoinHandle<()> {
    task::spawn(async move {
        let http_server = HttpServer::new(|| App::new().default_service(web::get().to(proxy)));
        http_server.bind(addr).unwrap().run().await.unwrap();
    })
}
