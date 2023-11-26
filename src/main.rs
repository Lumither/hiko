use std::process::exit;

use clap::{arg, Parser};

use hiko::config::Config;
use hiko::{log, run};

/// a simple service watchdog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of config file
    #[arg(short, long, default_value = "./Config.toml")]
    conf_path: String,
}

#[tokio::main]
async fn main() {
    // load config from terminal
    let args = Args::parse();

    // log init
    log::init();
    dbg!(&args.conf_path);

    // load conf
    let cfg = match Config::from(&args.conf_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            log::error!("{}", err);
            exit(1);
        }
    };

    // load mail module

    // axum
    run(args.conf_path.to_owned()).await;
}
