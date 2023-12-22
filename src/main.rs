mod host;

use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use anyhow::Result;
use clap::Parser;
use colored::Colorize;

use wakatime_annual_report::{get_analyses_report_data, read_json_from_file};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long, required = true)]
    input: String,

    /// Don't serve by the program
    #[arg(long = "no-serve")]
    no_serve: bool,

    /// The year to generate the report
    #[arg(long)]
    year: Option<i32>,

    /// Your time zone, default as 8 (UTC+8).
    #[arg(long = "time-zone", default_value = "8.0")]
    time_zone: f64,

    /// Timeout of the WakaTime's time calculating. See WakaTime's FAQ for more info.
    #[arg(long = "timeout", short, default_value = "900.0")]
    timeout: f64,

    /// The host address of the web server
    #[arg(long, default_value = "127.0.0.1")]
    host: String,

    /// The host port of the web server
    #[arg(short, long, default_value = "8031")]
    port: u16,
}

fn main() -> Result<()> {
    let args = Args::parse();

    pretty_env_logger::init();

    if !args.no_serve {
        host::host_init();
    }

    println!("{}", "✨ Please wait... We are reading the file...".yellow());
    let data = read_json_from_file(&args.input)?;

    let year: i32;
    if args.year.is_some() {
        year = args.year.unwrap();
    } else {
        let years = data.get_available_years();
        if years.len() == 1 {
            year = years[0];
        } else {
            println!("{} {}", "🗓 We have found different years in your record:".blue(), years.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(", "));
            loop {
                print!("   Please choose one: ");
                stdout().flush()?;
                let mut line = String::new();
                if let Ok(n) = stdin().read_line(&mut line) {
                    if n > 0 {
                        if let Ok(y) = i32::from_str(line.trim()) {
                            if years.contains(&y) {
                                year = y;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", format!("🔍 Analysing your {}...", year).green());

    println!("   {} If you are not in China, remember to configure the time zone in the arguments!", "Tips".green());

    let data = get_analyses_report_data(data, year, args.time_zone, args.timeout);

    if args.no_serve {
        serde_json::to_writer(File::create("./output.json")?, &data)?;
        println!("{} {} {}", "😄 The".green(), "output.json".blue(), "is successfully generated.".green())
    } else {
        println!("{}", "😄 The analysis is successful and we are starting the server for you...".green());
        println!("{} {}", "🌍 Open in your browser: ".green(), format!("http://localhost:{}", args.port).blue());
        println!("{}", "   And click Ctrl+C to exit.".green());

        host::host(&args.host, args.port, data)?;
    }

    Ok(())
}
