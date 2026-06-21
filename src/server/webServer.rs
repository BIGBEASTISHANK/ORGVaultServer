use crate::{
    security::{self, encryptionHandler},
    server::{self, webServerEndpoints::*},
};
use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, http, web};
use colored::*;
use local_ip_address::local_ip;
use serde_json::json;
use std::{
    collections::HashMap,
    process::{Child, Command, Stdio},
    sync::atomic,
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
    .map_err(|E| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "{0} {1:?}",
                "Error binding server and port | RunWebServerBackend:  ".red(),
                E
            ),
        )
    })?
    .run()
    .await;
}

// Run web frontend server
pub fn RunWebServerFrontend() -> std::io::Result<Child> {
    // Environment variables
    let mut envsHashMap: HashMap<String, String> = HashMap::new();
    envsHashMap.insert(
        "BACKEND_API_URL".to_string(),
        format!("http://{0}:3100", local_ip().unwrap()),
    );
    envsHashMap.insert(
        "KEY_BIN_HASH".to_string(),
        security::encryptionHandler::ConfigEncryptionKeyHash().unwrap(),
    );

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
            .envs(&envsHashMap)
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
                .envs(&envsHashMap)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .map_err(|E| {
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "{0} {1:?}",
                            "Frontend build/start error | RunWebServerFrontend:  ".red(),
                            E
                        ),
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
            .envs(&envsHashMap)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
    }
}

// Configuring API endpoints
fn ConfigureAPIEndpoints(cfg: &mut web::ServiceConfig) {
    cfg.route("/api/backend/ping", web::get().to(HandlePingEndpoint));
    cfg.route(
        "/api/backend/initializedStatus",
        web::get().to(HandleInitializedStatusEndpoint),
    );
    cfg.route(
        "/api/backend/initializeServer",
        web::post().to(HandleInitializeServerEndpoint),
    );
    cfg.route(
        "/api/backend/loginVerification",
        web::post().to(HandleLoginVerificationEndpoint),
    );
    cfg.route(
        "/api/backend/currentAdminDetails",
        web::post().to(HandleCurrentAdminDetailsEndpoint),
    );

    // Developer only endpoints
    cfg.route(
        "/api/developer/seeConfigFile",
        web::get().to(HandleDeveloperSeeConfigFileEndpoint),
    );
}

// Handling developer see config file endpoint
pub async fn HandleDeveloperSeeConfigFileEndpoint() -> HttpResponse {
    if !cfg!(debug_assertions) {
        return HttpResponse::Unauthorized().finish();
    }

    if let Ok(data) = encryptionHandler::DecryptConfigData() {
        return HttpResponse::Ok().json(json!(data));
    }

    if let Err(E) = encryptionHandler::DecryptConfigData() {
        return HttpResponse::InternalServerError().json(json!({"response": E.to_string()}));
    }

    HttpResponse::NotImplemented().finish()
}
