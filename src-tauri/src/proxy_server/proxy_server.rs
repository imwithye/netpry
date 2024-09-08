use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use reqwest::{header::HeaderMap, header::HeaderName, header::HeaderValue, Client};
use std::error::Error;
use std::sync::{Arc, RwLock};
use tauri::AppHandle;

pub struct ProxyServer {
    store: Arc<RwLock<super::Store>>,
}

impl ProxyServer {
    pub fn new(app_handle: AppHandle) -> ProxyServer {
        ProxyServer {
            store: Arc::new(RwLock::new(super::Store::new(app_handle))),
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.store
            .write()
            .map_err(|_| "Failed to acquire write lock")?
            .run()?;

        let addr = "".to_string();
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
        let uri = format!("https://stage.akuity.io{}", req.uri());

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
            let body = chunk?;
            request_details.request_body = Some(body.to_vec());
            forward_req = forward_req.body(body);
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

        let status_code = resp.status();
        request_details.status_code = Some(status_code.clone());

        let status_code: actix_web::http::StatusCode = status_code.as_str().parse()?;
        let mut client_resp = HttpResponse::build(status_code);
        let mut response_headers = HeaderMap::new();
        for (name, value) in resp.headers().iter() {
            let header_name: HeaderName = name.as_str().parse()?;
            let header_value: HeaderValue = HeaderValue::from_bytes(value.as_bytes())?;
            response_headers.insert(header_name.clone(), header_value.clone());

            let header_name: actix_web::http::header::HeaderName = name.as_str().parse()?;
            let header_value: actix_web::http::header::HeaderValue =
                actix_web::http::header::HeaderValue::from_bytes(value.as_bytes())?;
            client_resp.append_header((header_name, header_value));
        }
        request_details.response_headers = Some(response_headers);

        let body = resp
            .bytes()
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        request_details.response_body = Some(body.to_vec());
        request_details.end_time = Some(time::OffsetDateTime::now_utc());

        store
            .read()
            .map_err(|_| "Failed to acquire read lock")?
            .send(request_details.clone())
            .await;

        Ok(client_resp.body(body))
    }
}
