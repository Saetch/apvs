use std::{sync::{Arc, Weak}, future::Future, time::Duration, error::Error};
use actix_web::{get, post, web::{self, Bytes, Data}, HttpResponse, HttpServer, App, Responder, FromRequest, dev::{Server, ServerHandle}, body::BoxBody, error, HttpRequest};
use actix_web_lab::web::redirect;
use flume::{Receiver, r#async};
use tokio::{join, try_join, sync::RwLock, time::sleep};

struct State{
    i: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("I received a hello world!");
    HttpResponse::Ok().body("Hello world!")
}


async fn upload(data: Data<RwLock<State>>, req_body: Bytes) -> impl Responder {
    let newi = data.read().await.i +1;

    data.write().await.i = newi;
    println!("{}", newi);
   
    let mut data : Vec<u8> = req_body.into_iter().collect(); 
    for u in data {
        print!("{}-", u);
    }
    println!("Handled upload call!");
    HttpResponse::Ok().body("Received without error! This is amazing!")
}

async fn upload_get(data: Data<RwLock<State>>)-> impl Responder{

    let newi = data.read().await.i +1; 
    data.write().await.i = newi;
    HttpResponse::Ok().body("This worked! ".to_owned()+&newi.to_string())

}

async fn default() -> impl Responder{


    sleep(Duration::from_secs(630)).await;
    HttpResponse::Ok().body("This is the default site!")
}

async fn communication_loop(handle: ServerHandle)-> Result<(), std::io::Error>{
    
    
    tokio::time::sleep(Duration::from_secs(700)).await;
    handle.stop(true).await;
    Ok(())
}

#[tokio::main] //this could be actix_web::main, aswell, but we don't need the additional workers
async fn main() -> std::io::Result<()> {

    let data = Data::new(RwLock::new(State{i: 0 }));

    let srv =  HttpServer::new(move || {

        let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(2147483648) //#2GB
        // use custom error handler
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        });

        App::new().app_data(json_cfg)
            .service(hello)
            .service(web::scope("/check").app_data(Data::clone(&data))
                                .service(web::resource("/upload").route(web::post().to(upload))/*.route(web::get().to(upload_get))*/)
                                .service(web::resource("/").route(web::get().to(default))))
    })
    .client_disconnect_timeout(Duration::from_millis(0))
    .client_request_timeout(Duration::from_millis(10000))
    .bind(("0.0.0.0", 8080)).unwrap();

    let server = srv.run();
    let link = server.handle().clone();
    
    println!("Starting webserver!");
    let res = try_join!(server, communication_loop(link));
    res.unwrap();
    println!("Shutting down without encountering any Errors!");
    return Ok(());
}


