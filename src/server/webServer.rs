use crate::server;
use actix_cors::Cors;
use actix_web::{App, HttpServer, http, web};
use actix_web::{HttpRequest, HttpResponse};
use serde_json::json;
use std::process::{Child, Command, Stdio};
use std::sync::atomic;

// Variables
pub const WEB_SERVER_BACKEND_PORT: u16 = 3100;
pub const WEB_SERVER_FRONTEND_PORT: u16 = 3000;

// Run web backend server
pub async fn WebServerBackendRunner() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
        // CORS middleware
        let cors = Cors::default()
            .allowed_origin_fn(|ORIGIN, _req_head| {
                if let Ok(ORIGIN_STR) = ORIGIN.to_str() {
                    if ORIGIN_STR.starts_with("http://192.168.") {
                        return true;
                    }
                }
                false
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        // Configure API endpoint
        App::new().wrap(cors).configure(APIEndpointConfig)
    })
    .bind((server::SERVER_ADDRESS, WEB_SERVER_BACKEND_PORT))?
    .run()
    .await;
}

// Run web frontend server
pub fn WebServerFrontendRunner() -> std::io::Result<Child> {
    if cfg!(debug_assertions) {
        Command::new("yarn")
            .args(&["dev", "-p", &WEB_SERVER_FRONTEND_PORT.to_string()])
            .current_dir(crate::WEB_FRONTEND_DATA_FILE)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    } else {
        if crate::REBUILD_FRONTEND.load(atomic::Ordering::SeqCst) {
            Command::new("yarn")
                .args(&["--silent", "build"])
                .current_dir(crate::WEB_FRONTEND_DATA_FILE)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()?;
        }

        Command::new("yarn")
            .args(&[
                "--silent",
                "start",
                "-p",
                &WEB_SERVER_FRONTEND_PORT.to_string(),
            ])
            .current_dir(crate::WEB_FRONTEND_DATA_FILE)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    }
}

// APIEndpoint config function
fn APIEndpointConfig(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/test", web::get().to(APIEndpointTest));
}

// APIEndpoint test function
async fn APIEndpointTest(_req: HttpRequest) -> HttpResponse {
    println!("API ENDPOINT CALLED");
    HttpResponse::Ok().json(json!({ "message": "Hello World" }))
}
