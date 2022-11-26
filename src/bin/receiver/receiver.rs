use std::{sync::{Arc, Weak}, future::Future, time::Duration, error::Error};
use actix_web::{get, post, web::{self, Bytes}, HttpResponse, HttpServer, App, Responder, FromRequest, dev::{Server, ServerHandle}, body::BoxBody, error};
use flume::{Receiver, r#async};
use tokio::{join, try_join};

struct State{
    i: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("I received a hello world!");
    HttpResponse::Ok().body("Hello world!")
}


async fn upload(req_body: Bytes) -> impl Responder {
    HttpResponse::Ok().body("Received without error!")
   /* let mut data : Vec<u8> = req_body.into_iter().collect(); 
    for u in data {
        print!("{}-", u);
    }

    HttpResponse::Ok().body("Received without error!")*/
}

async fn upload_get()-> impl Responder{

    HttpResponse::Ok().body("This worked!")
}

async fn communication_loop(handle: ServerHandle)-> Result<(), std::io::Error>{
    
    
    tokio::time::sleep(Duration::from_secs(150)).await;
    handle.stop(true).await;
    Ok(())
}

#[actix_web::main] //this could be actix_web::main, as well, but we don't need the additional workers
async fn main() -> std::io::Result<()> {

    let srv =  HttpServer::new(|| {

        let json_cfg = web::JsonConfig::default()
        // limit request payload size
        .limit(2147483648) //#2GB
        // use custom error handler
        .error_handler(|err, _req| {
            error::InternalError::from_response(err, HttpResponse::Conflict().into()).into()
        });

        App::new().app_data(json_cfg)
            .service(hello)
            .service(web::scope("/check").app_data(State{i : 0}).service(web::resource("/upload").route(web::post().to(upload))))
    })
    .bind(("127.0.0.1", 8080)).unwrap();

    let server = srv.run();
    let link = server.handle().clone();
    

    let res = try_join!(server, communication_loop(link));
    res.unwrap();
    println!("Shutting down without encountering any Errors!");
    return Ok(());
}


