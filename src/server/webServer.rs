use crate::server;
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, http, web};
use local_ip_address::local_ip;
use serde_json::json;
use std::{
    process::{Child, Command, Stdio},
    sync::atomic,
    time::Instant,
};

// Run web backend server
pub async fn RunWebServerBackend() -> Result<(), std::io::Error> {
    return HttpServer::new(|| {
        // CORS middleware
        let CORS = Cors::default()
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
        App::new().wrap(CORS).configure(ConfigureAPIEndpoints)
    })
    .bind((server::SERVER_ADDRESS, server::WEB_SERVER_BACKEND_PORT))?
    .run()
    .await;
}

// Run web frontend server
pub fn RunWebServerFrontend() -> std::io::Result<Child> {
    if cfg!(debug_assertions) {
        Command::new("yarn")
            .args(&["dev", "-p", &server::WEB_SERVER_FRONTEND_PORT.to_string()])
            .current_dir(&*crate::WEB_FRONTEND_DATA_FILE)
            .env(
                "NEXT_PUBLIC_API_URL",
                format!("http://{0}:3100", local_ip().unwrap()),
            )
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    } else {
        if crate::rebuildFrontendStatus.load(atomic::Ordering::SeqCst) {
            Command::new("yarn")
                .args(&["--silent", "build"])
                .current_dir(&*crate::WEB_FRONTEND_DATA_FILE)
                .env(
                    "NEXT_PUBLIC_API_URL",
                    format!("http://{0}:3100", local_ip().unwrap()),
                )
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()?;
        }

        Command::new("yarn")
            .args(&[
                "--silent",
                "start",
                "-p",
                &server::WEB_SERVER_FRONTEND_PORT.to_string(),
            ])
            .current_dir(&*crate::WEB_FRONTEND_DATA_FILE)
            .env(
                "NEXT_PUBLIC_API_URL",
                format!("http://{0}:3100", local_ip().unwrap()),
            )
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    }
}

// Configuring API endpoints
fn ConfigureAPIEndpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/ping", web::get().to(HandlePingEndpoint));
    cfg.route(
        "/api/initializedStatus",
        web::get().to(HandleInitializedStatusEndpoint),
    );
}

// Handeling ping endpoint
async fn HandlePingEndpoint() -> HttpResponse {
    let start = Instant::now();
    let response_time_ms = start.elapsed().as_millis();

    HttpResponse::Ok().json(json!({
        "message": "pong",
        "response_time_ms": response_time_ms
    }))
}

// Handeling initialized status endpoint
async fn HandleInitializedStatusEndpoint() -> HttpResponse {
    println!("isInitialized: {:?}", crate::isInitialized.load(atomic::Ordering::SeqCst));
    if crate::isInitialized.load(atomic::Ordering::SeqCst) {
        return HttpResponse::Ok().finish();
    } else {
        return HttpResponse::NotImplemented().finish();
    }
}
