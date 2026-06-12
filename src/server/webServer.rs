use crate::server;
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, http, web};
use colored::*;
use local_ip_address::local_ip;
use serde::Deserialize;
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
    .bind((server::SERVER_ADDRESS, server::WEB_SERVER_BACKEND_PORT))
    .map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{} {:?}", "Error binding server and port:".red(), e),
        )
    })?
    .run()
    .await;
}

// Run web frontend server
pub fn RunWebServerFrontend() -> std::io::Result<Child> {
    // Checking if node_modules exists
    if !std::path::Path::new(&*crate::WEB_FRONTEND_DATA_FILE)
        .join("node_modules")
        .exists()
    {
        // Log
        println!(
            "\n{0} were not installed. {1}----------",
            "node_modules/".yellow(),
            "Installing node_modules...\n".green()
        );
        // Installing node_modules
        Command::new("yarn")
            .args(&["install"])
            .current_dir(&*crate::WEB_FRONTEND_DATA_FILE)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()?;
    }

    if cfg!(debug_assertions) {
        // Log
        println!(
            "\n{0}----------",
            "Running frontend in debug mode...\n".green()
        );

        // Running in debug mode
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
            // Log
            println!("\n{0}----------", "Building frontend...\n".green());

            // Building frontend
            Command::new("yarn")
                .args(&["--silent", "build"])
                .current_dir(&*crate::WEB_FRONTEND_DATA_FILE)
                .env(
                    "NEXT_PUBLIC_API_URL",
                    format!("http://{0}:3100", local_ip().unwrap()),
                )
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .map_err(|e| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("{} {:?}", "Frontend build/start error:".red(), e),
                    )
                })?;
        }

        // Log
        println!(
            "\n{0}----------",
            "Running frontend in release mode...\n".green()
        );

        // Running frontend in release mode
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
    cfg.route(
        "/api/initializeServer",
        web::post().to(HandleInitializeServerEndpoint),
    );
}

// Handling ping endpoint
async fn HandlePingEndpoint() -> HttpResponse {
    let START = Instant::now();
    let RESPONSE_TIME_MS = START.elapsed().as_millis();

    HttpResponse::Ok().json(json!({
        "message": "pong",
        "RESPONSE_TIME_MS": RESPONSE_TIME_MS
    }))
}

// Handling initialized status endpoint
async fn HandleInitializedStatusEndpoint() -> HttpResponse {
    if crate::isInitialized.load(atomic::Ordering::SeqCst) {
        return HttpResponse::Ok().finish();
    } else {
        return HttpResponse::NotImplemented().finish();
    }
}

// Handling initialize server endpoint
#[derive(Deserialize)]
struct InitializeRequest {
    adminMacAddress: String,
    username: String,
    password: String,
}

async fn HandleInitializeServerEndpoint(req: web::Json<InitializeRequest>) -> HttpResponse {
    // Safely extract headers without unwrapping
    let MAC_ADDRESS = &req.adminMacAddress;
    let USERNAME = &req.username;
    let PASSWORD = &req.password;

    println!("macAddress: {:?}", MAC_ADDRESS);
    println!("username: {:?}", USERNAME);
    println!("password: {:?}", PASSWORD);

    // Initialize config file
    match server::InitializeConfigFile(MAC_ADDRESS.clone(), USERNAME.clone(), PASSWORD.clone()) {
        Ok(_) => return HttpResponse::Ok().finish(),
        Err(e) => {
            println!("{0} {1}", "Error: ".red(), e);
            return HttpResponse::InternalServerError().finish();
        }
    };
}
