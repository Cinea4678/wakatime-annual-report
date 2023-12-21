use std::fs::File;
use flate2::Compression;
use flate2::write::GzEncoder;

fn main() {
    let file = File::create("web_dist.tar").unwrap();
    let enc = GzEncoder::new(file, Compression::default());
    let mut tar_file = tar::Builder::new(enc);

    tar_file.append_dir_all("dist","./webpage/dist").unwrap();
}
