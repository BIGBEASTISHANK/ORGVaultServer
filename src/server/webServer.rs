use crate::server;
use std::io;
use actix_files::NamedFile;
use actix_web::HttpRequest;
use actix_web::{App, HttpServer, web};

// Home web page
pub async fn WebHome(_req: HttpRequest) -> io::Result<NamedFile> {
    return Ok(NamedFile::open("static/index.html")?);
}

// APIEndpoint config function
fn APIEndpointConfig(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(WebHome));
}

// Run web server
pub async fn WebServerRunner() -> Result<(), std::io::Error> {
    return HttpServer::new(|| App::new().configure(APIEndpointConfig))
        .bind((server::SERVER_ADDRESS, server::SERVER_PORT))?
        .run()
        .await
}