use actix_web::dev::ServerHandle;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use futures::stream::StreamExt;
use reqwest::{header::HeaderMap, header::HeaderName, header::HeaderValue, Client};
use std::error::Error;
use std::sync::{Arc, Mutex};
use actix_web::middleware::Logger;
use tauri::{AppHandle, Manager};
use tokio::task;

pub struct ProxyServer {
    app_handle: AppHandle,

    ingress: Option<String>,
    backend: Option<String>,
    server_handle: Option<ServerHandle>,
}

impl ProxyServer {
    pub fn new(app_handle: AppHandle) -> ProxyServer {
        ProxyServer {
            app_handle,

            ingress: None,
            backend: None,
            server_handle: None,
        }
    }

    pub fn is_running(&self) -> bool {
        if self.server_handle.is_some() {
            true
        } else {
            false
        }
    }

    pub fn run(&mut self, ingress: String, backend: String) -> Result<(), String> {
        if self.server_handle.is_some() {
            return Err("Proxy is already running".to_string());
        }

        self.ingress = Some(ingress.clone());
        self.backend = Some(backend.clone());

        let app_handle = Arc::new(Mutex::new(self.app_handle.clone()));
        let backend = Arc::new(Mutex::new(self.backend.as_ref().unwrap().clone()));
        let srv = HttpServer::new(move || {
            let app_handle = app_handle.clone();
            let backend = backend.clone();
            App::new().default_service(web::get().to(move |req, payload| {
                let app_handle = app_handle.clone();
                let backend = backend.clone();
                async move {
                    let app_handle_guard = app_handle.lock().map_err(|err| err.to_string())?;
                    let backend_guard = backend.lock().map_err(|err| err.to_string())?;
                    ProxyServer::proxy(app_handle_guard.clone(), backend_guard.clone(), req, payload).await
                }
            }))
        })
        .bind(ingress.clone())
        .map_err(|err| format!("Invalid ingress address: {}", err.to_string()))?
        .run();
        let srv_handle = srv.handle();
        task::spawn(srv);
        self.server_handle = Some(srv_handle);
        Ok(())
    }

    async fn proxy(
        app_handle: AppHandle,
        backend: String,
        req: HttpRequest,
        mut payload: web::Payload,
    ) -> Result<HttpResponse, Box<dyn Error>> {
        let mut request_details = super::RequestDetails::new();

        let method: reqwest::Method = req.method().as_str().parse()?;
        let uri = format!("{}{}", backend, req.uri());

        request_details.method = method.clone();
        request_details.uri = uri.clone().parse()?;

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

        app_handle
            .emit_all("update", request_details.clone())
            .map_err(|err| err.to_string())?;

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

        app_handle
            .emit_all("update", request_details.clone())
            .map_err(|err| err.to_string())?;

        Ok(client_resp.body(body))
    }
}
