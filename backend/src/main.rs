use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix_web::{
    post,
    web::{Data, Path},
    App, HttpResponse, HttpServer, Responder,
};

#[post("/runners/{client_id}")]
async fn echo(
    req_body: String,
    client_id: Path<String>,
    runners: Data<Arc<RwLock<HashMap<String, String>>>>,
) -> impl Responder {
    runners
        .write()
        .unwrap()
        .insert(client_id.into_inner(), "url".to_string());
    println!("runners {:?}", runners.read().unwrap());
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let runners: Arc<RwLock<HashMap<String, String>>> = Arc::new(RwLock::new(HashMap::new()));
    HttpServer::new(move || {
        App::new()
            .service(echo)
            .app_data(Data::new(runners.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
