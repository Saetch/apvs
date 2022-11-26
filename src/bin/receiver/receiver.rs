use std::{sync::{Arc, Weak}, future::Future, time::Duration};
use actix_web::{error, get, post, web::{self, Bytes}, HttpResponse, HttpServer, App, Responder, FromRequest, dev::{Server, ServerHandle}, body::BoxBody};
use flume::Receiver;
use tokio::join;


#[get("/")]
async fn hello() -> impl Responder {
    println!("I received a hello world!");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/check")]
async fn echo(req_body: Bytes) -> impl Responder {
    let mut data : Vec<u8> = req_body.into_iter().collect(); 
    for u in data {
        print!("{}-", u);
    }

    HttpResponse::Ok().body("Received without error!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


async fn communication_loop(handle: ServerHandle){
    
    
    tokio::time::sleep(Duration::from_secs(15)).await;
    handle.stop(true).await;
    ()
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
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080)).unwrap();

   // let (sender, receiver) = flume::unbounded();
    let server = srv.run();
    let link = server.handle().clone();
    
    println!("!SD");

    //let arc2_s = arc_server.clone();
    let res = join!(server, communication_loop(link));
    res.0.unwrap();
    //server.await.unwrap();
    println!("fasfafasf");
    //server.handle().stop(false);
    //sender.send(server);
    return Ok(());
}


