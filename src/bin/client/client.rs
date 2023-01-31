use std::{io::{Write, Read}, path::Path, fs::{File}, env::{ var}};

use actix_web::web::Bytes;
use walkdir::WalkDir;
use zip::write::FileOptions;

#[tokio::main]
pub async fn main() -> std::io::Result<()>{
    let args :Vec<_> = std::env::args().collect();
    let mut src_dir = "";
    let mut _entrypoint = "";
    let mut ct = 0;
    for arg in args.iter(){
        if !arg.starts_with("--") {
            ct=ct+1;
        }
        match ct {
            2 => src_dir = &arg,
            3 => _entrypoint = &arg,
            _default => (),
        }
    }
    let path = std::path::Path::new("tmp_zipped_src.zip");
   

    if !Path::new(src_dir).is_dir() {

        //This is for handling a single image to create a container based on that, but the upload takes too long and thus this was abandoned
        println!("Specifying single image: {}", Path::new(src_dir).to_str().unwrap());
        #[allow(unused_variables)]
        let src_dfile = src_dir;
        let output_file = std::fs::File::create(&path).unwrap();
        let _zip = zip::ZipWriter::new(output_file);
        let _opts = FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o777);
        let _buffer: Vec<Bytes> = Vec::new();

        todo!()

    }
    else{

        
        let output_file = std::fs::File::create(&path).unwrap();
        let walkdir = WalkDir::new(src_dir);
        let iterator = walkdir.into_iter();




        let mut zip = zip::ZipWriter::new(output_file);

        let opts = FileOptions::default().compression_method(zip::CompressionMethod::Deflated).unix_permissions(0o777);
        let mut buffer = Vec::new();
        for f in iterator {
            let in_dir_object = f.expect("could not iterate through directory!");
            let p = in_dir_object.path();
            let name = p.strip_prefix(Path::new(&src_dir)).unwrap();

            if p.is_file(){
                println!("Debug: adding file {:?} as {:?} ...", p, name );
                #[allow(deprecated)]
                zip.start_file_from_path(name, opts).unwrap();
                let mut f = File::open(p).unwrap();

                f.read_to_end(&mut buffer).unwrap();
                zip.write_all(&*buffer).unwrap();
                buffer.clear();
            }   else if !name.as_os_str().is_empty(){
                println!("Debug: adding dir {:?} as {:?} ...", p, name);
                #[allow(deprecated)]
                zip.add_directory_from_path(name, opts).unwrap();
            }
        }

        zip.finish().unwrap();
    }
    let client = reqwest::Client::new();
    
    let host = var("receiver").unwrap_or("localhost".to_string());
    let port = std::env::var("receiver_port").unwrap_or("8080".to_string());
    let payload = tokio::fs::read("tmp_zipped_src.zip").await.unwrap();
    let path = std::env::var("receiver_path").unwrap_or("/check/upload".to_string());
    

    let complete_target ="http://".to_owned()+ &host + ":" +&port + &path;

    println!("Sending request to: {}", complete_target);
    let resu = client.post(complete_target).body(payload).send().await;
    println!("Result: {:?}", resu.unwrap());

    
    return Ok(());
}