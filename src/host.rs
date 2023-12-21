use std::fs;
use std::fs::File;
use dirs;
use flate2::read::GzDecoder;
use tar::Archive;
use anyhow::Result;
use wakatime_annual_report::report::NormalReport;

/// 我们的网页
const WEB_DIST_TAR: &[u8] = include_bytes!("../web_dist.tar");

pub fn host_init() {
    let mut data_dir = dirs::data_dir().unwrap();
    data_dir.push("cinea/wakatime-annual-report");
    fs::create_dir_all(data_dir.as_path()).unwrap();

    // 解压 & 写入打包完成的前端
    let tar = GzDecoder::new(WEB_DIST_TAR);
    let mut a = Archive::new(tar);
    a.unpack(data_dir.as_path()).unwrap();
}

pub fn host(host: &str, port: u16, report: NormalReport) -> Result<()> {
    let mut dist_dir = dirs::data_dir().unwrap();
    dist_dir.push("cinea/wakatime-annual-report/dist");

    let mut json_path = dist_dir.clone();
    json_path.push("output.json");
    serde_json::to_writer(File::create(json_path.as_path())?, &report)?;

    let server = file_serve::ServerBuilder::new(dist_dir).hostname(host).port(port).build();
    server.serve()?;

    Ok(())
}
