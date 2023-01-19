use std::{sync::RwLock, time::Duration};

use actix_web::{web::{Data, self}, HttpServer, error, HttpResponse, App, Responder, get};


///// TODO!
struct State{
    i: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("I received a hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[tokio::main] //this could be actix_web::main, aswell, but we don't need the additional workers
async fn main() -> std::io::Result<()> {



let data = Data::new(RwLock::new(State{i: 0 }));

    let srv =  HttpServer::new(move || {

        let json_cfg = actix_web::web::JsonConfig::default()
        // limit request payload size
        .limit(2147483648) //#2GB
        // use custom error handler
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        });

        App::new().app_data(json_cfg)
            .service(hello)
            .service(web::scope("/execute").app_data(Data::clone(&data))
                                .service(web::resource("/entrypoint").route(web::post().to(upload))/*.route(web::get().to(upload_get))*/)
                                .service(web::resource("/").route(web::get().to(default)))
                                .service(web::resource("uploadUncompiled").route(web::post().to(upload_uncompiled)))    
                            )
    })
    .client_disconnect_timeout(Duration::from_millis(0))
    .client_request_timeout(Duration::from_millis(10000))
    .bind(("0.0.0.0", 8080)).unwrap();



    Ok(())
}