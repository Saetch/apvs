use std::{ time::Duration,  path::PathBuf, io::Cursor, process::Command};
use actix_web::{get, web::{self, Bytes, Data}, HttpResponse, HttpServer, App, Responder, error};
use tokio::{ sync::RwLock};


use bollard::{image::BuildImageOptions, container::{CreateContainerOptions, Config, StartContainerOptions}};
use bollard::Docker;

use std::collections::HashMap;

use futures_util::stream::StreamExt;

use std::io::prelude::*;
use std::fs::File;
use tar::Builder;
use rand::Rng;

extern crate tar;

struct State{
    i: u16,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("I received a hello world!");
    HttpResponse::Ok().body("Hello world!")
}

async fn upload(data: Data<RwLock<State>>, req_body: Bytes) -> impl Responder{
    let mut rng = rand::thread_rng();
    let newi = data.read().await.i +1;
    let id = rng.gen::<i64>().to_string();
    data.write().await.i = newi;
    println!("Received execute command: {}", newi);
   
    let data : Vec<u8> = req_body.into_iter().collect(); 
    for u in data.iter() {
        print!("{}-", u);
    }

    let target_dir = PathBuf::from("execute");
    zip_extract::extract(Cursor::new(data), &target_dir, true).unwrap();


    let file = File::create("executorBuildContext.tar").unwrap();
    let mut a = Builder::new(file);

    a.append_file("Dockerfile", &mut File::open("execute/Dockerfile").unwrap()).unwrap();

    let app_f = File::open("execute/app.py");
    if app_f.is_ok(){
        a.append_file("app.py", &mut app_f.unwrap()).unwrap()
    }

    let app_f = File::open("execute/requirements.txt");
    if app_f.is_ok(){
        a.append_file("requirements.txt", &mut app_f.unwrap()).unwrap()
    }

    a.finish().unwrap();


    let mut buf = Vec::new();
    File::open("executorBuildContext.tar").unwrap().read_to_end(&mut buf).unwrap();
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut build_image_args = HashMap::new();
    build_image_args.insert("dummy", "value");

    let mut build_image_labels = HashMap::new();
    build_image_labels.insert("maintainer", "somemaintainer");

    let build_image_options = BuildImageOptions {
        t: "image".to_owned()+&newi.to_string()+&id,
        ..Default::default()
        
    };

    let mut image_build_stream = docker.build_image(build_image_options, None, Some(buf.into()));

    while let Some(msg) = image_build_stream.next().await {
        println!("Message: {:?}", msg);
    }


    println!(" \n Starting container! ");
    let name_c = "run".to_owned() + &newi.to_string()+&id;
    let options = Some(CreateContainerOptions{
        name: &name_c,
    });

    let config = Config{
        image: Some("image".to_owned() + &newi.to_string()+&id),
        ..Default::default()
    };

    docker.create_container(options, config).await.unwrap();

    docker.start_container(&name_c,  None::<StartContainerOptions<String>>).await.unwrap();


    let _c = Command::new("sh").arg("-c").arg("rm -rf execute executorBuildContext.tar").spawn().unwrap();
    HttpResponse::Ok().body("Received without error! This is amazing!")
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
                                .service(web::resource("/upload").route(web::post().to(upload)))
                            )
    })

    .client_disconnect_timeout(Duration::from_millis(0))
    .client_request_timeout(Duration::from_millis(10000))
    .bind(("0.0.0.0", 8080)).unwrap();


    println!("Starting webserver!");
    srv.run().await.unwrap();
    
 


    Ok(())
}