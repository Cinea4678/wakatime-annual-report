use std::fs::File;
use std::io::Write;
use anyhow::Result;
use clap::Parser;

use wakatime_annual_report::{get_analyses_report_data, read_json_from_file};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file name
    #[arg(short, long)]
    input: String,
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    let file_path = r#"/Users/cineazhan/Downloads/wakatime-cinea123163.com-ae526249eb5349709adfaa168f50c84f(1).json"#;

    println!("Loading data from json...");
    let data = read_json_from_file(file_path)?;

    println!("Analysing data...");
    let data = get_analyses_report_data(data, 2023, 8.0, 900.0);

    serde_json::to_writer(File::create("./output.json")?, &data)?;

    println!("Finished...");

    Ok(())
}
