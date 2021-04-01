use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;

use walkdir::WalkDir;
use zip::write::FileOptions;
use zip::CompressionMethod;
use zip::ZipWriter;

fn main() -> Result<(), Box<dyn Error>> {
    // rerun when resources changed
    println!("cargo:rerun-if-changed=resources");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("resources.zip");
    eprintln!("writing to {:?}", dest_path);
    let dest = File::create(dest_path)?;

    let cargo_dir = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let src_path = Path::new(&cargo_dir).join("resources");

    zip_dir(&src_path, dest, FileOptions::default().compression_method(CompressionMethod::Bzip2))?;

    Ok(())
}

// https://github.com/zip-rs/zip/blob/master/examples/write_dir.rs
pub fn zip_dir<T: Write + Seek>(
    src: &Path,
    dest: T,
    options: FileOptions,
) -> Result<(), Box<dyn Error>> {
    let mut zip = ZipWriter::new(dest);

    let mut buffer = Vec::new();
    for entry in WalkDir::new(src) {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(src)?;

        // Write file or directory explicitly
        if path.is_file() {
            eprintln!("adding file {:?} as {:?} ...", path, name);
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;

            f.read_to_end(&mut buffer)?;
            zip.write_all(&*buffer)?;
            buffer.clear();
        } else if name.as_os_str().is_empty() {
            // Only if not root! Avoids path spec / warning
            // and mapname conversion failed error on unzip
            eprintln!("adding dir {:?} as {:?} ...", path, name);
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;
    Ok(())
}
