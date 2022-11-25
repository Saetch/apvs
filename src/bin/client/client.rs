use std::{io::{Write, Read, Seek}, path::Path, default, fs::File};

use walkdir::{WalkDir, DirEntry};
use zip::{write::FileOptions, result::ZipError};

pub fn main(){
    let args :Vec<_> = std::env::args().collect();
    let mut src_dir = "";
    let mut entrypoint = "";
    let mut ct = 0;
    //handle arguments and options
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
    

    if !Path::new(src_dir).is_dir() {
        println!("source specified is not a directory!: {}", Path::new(src_dir).to_str().unwrap());
        return;
    }

    zip_directory(src_dir, "temp_zipped_src.zip", zip::CompressionMethod::Deflated).unwrap();
    
    
}

fn zip_dir<T>(
    it: &mut dyn Iterator<Item = DirEntry>,
    prefix: &str,
    writer: T,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()>
where
    T: Write + Seek,
{
    let mut zip = zip::ZipWriter::new(writer);
    let options = FileOptions::default()
        .compression_method(method)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();
    for entry in it {
        let path = entry.path();
        let name = path.strip_prefix(Path::new(prefix)).unwrap();

        if path.is_file() {
            println!("adding file {:?} as {:?} ...", path, name);
            #[allow(deprecated)]
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            println!("adding dir {:?} as {:?} ...", path, name);
            #[allow(deprecated)] //this is the reason for every file/subfolder, the whole write action is redone. After add_directory, the ZipWriter cannot add anything to the file
            zip.add_directory_from_path(name, options)?;
        }
    }
    zip.finish()?;
    Result::Ok(())
}

fn zip_directory(
    src_dir: &str,
    dst_file: &str,
    method: zip::CompressionMethod,
) -> zip::result::ZipResult<()> {
    if !Path::new(src_dir).is_dir() {
        return Err(ZipError::FileNotFound);
    }

    let path = Path::new(dst_file);
    let file = File::create(path).unwrap();

    let walkdir = WalkDir::new(src_dir);
    let it = walkdir.into_iter();

    zip_dir(&mut it.filter_map(|e| e.ok()), src_dir, file, method)?;

    Ok(())
}