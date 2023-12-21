use std::fs;
use dirs;
use flate2::read::GzDecoder;
use tar::Archive;

/// 我们的网页
const WEB_DIST_TAR: &[u8] = include_bytes!("../web_dist.tar");

pub fn host_init(){
    let mut temp_dir = dirs::data_dir().unwrap();
    temp_dir.push("cinea/wakatime-annual-report");
    fs::create_dir_all(temp_dir.as_path()).unwrap();

    // 解压 & 写入打包完成的前端
    let tar = GzDecoder::new(WEB_DIST_TAR);
    let mut a = Archive::new(tar);
    a.unpack(temp_dir.as_path()).unwrap();
}

pub fn host(host:)
