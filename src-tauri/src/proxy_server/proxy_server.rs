use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use reqwest::{header::HeaderMap, header::HeaderName, header::HeaderValue, Client};
use std::error::Error;
use std::sync::{Arc, RwLock};
use tauri::AppHandle;

pub struct ProxyServer {
    addr: String,
    store: Arc<RwLock<super::Store>>,
}

impl ProxyServer {
    pub fn new(addr: &str, app_handle: AppHandle) -> ProxyServer {
        ProxyServer {
            addr: addr.to_string(),
            store: Arc::new(RwLock::new(super::Store::new(app_handle))),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.store
            .write()
            .map_err(|_| "Failed to acquire write lock")?
            .run()?;

        let addr = self.addr.clone();
        let store = Arc::clone(&self.store);
        let http_server = HttpServer::new(move || {
            let store = Arc::clone(&store);
            App::new().default_service(web::get().to(move |req, payload| {
                let store = Arc::clone(&store);
                async move { ProxyServer::proxy(store, req, payload).await }
            }))
        });
        let http_server = http_server
            .bind(addr)
            .map_err(|_| "Failed to bind address")?;
        Ok(http_server
            .run()
            .await
            .map_err(|_| "Failed to run http server")?)
    }

    async fn proxy(
        store: Arc<RwLock<super::Store>>,
        req: HttpRequest,
        mut payload: web::Payload,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let mut request_details = super::RequestDetails::new();

        let method: reqwest::Method = req.method().as_str().parse()?;
        let uri = format!("https://google.com{}", req.uri());

        request_details.method = method.clone();
        request_details.uri = uri.clone().parse().unwrap();

        let client = Client::new();
        let mut forward_req = client.request(method.clone(), &uri);
        let mut headers = HeaderMap::new();
        for (name, value) in req.headers().iter() {
            let header_name: HeaderName = name.as_str().parse()?;
            let header_value: HeaderValue = HeaderValue::from_bytes(value.as_bytes())?;
            headers.insert(header_name.clone(), header_value.clone());
            request_details
                .request_headers
                .insert(header_name.clone(), header_value.clone());
        }
        if let Some(chunk) = payload.next().await {
            forward_req = forward_req.body(chunk?);
        }

        store
            .read()
            .map_err(|_| "Failed to acquire read lock")?
            .send(request_details.clone())
            .await;

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
}
