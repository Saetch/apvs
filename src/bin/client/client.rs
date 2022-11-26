use std::{io::{Write, Read}, path::Path, default, fs::File};

use walkdir::WalkDir;
use zip::write::FileOptions;

pub fn main(){
    let args :Vec<_> = std::env::args().collect();
    let mut src_dir = "";
    let mut entrypoint = "";
    let mut ct = 0;
    for arg in args.iter(){
        if !arg.starts_with("--") {
            ct=ct+1;
        }
        match ct {
            2 => src_dir = &arg,
            3 => entrypoint = &arg,
            _default => (),
        }
    }
    let path = std::path::Path::new("tmp_zipped_src.zip");
   

    if !Path::new(src_dir).is_dir() {
        println!("source specified is not a directory!: {}", Path::new(src_dir).to_str().unwrap());
        return;
    }
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