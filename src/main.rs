use std::process::{exit, Command};

use clap::ArgAction;
use clap::{arg, Parser};

use hiko::config::Config;
use hiko::run;

/// a simple service watchdog
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path of config file
    #[arg(short, long, default_value = "./Config.toml")]
    config_path: String,

    /// Only test config file
    #[arg(long = "test", short = 't', action = ArgAction::SetTrue)]
    is_test_config_mod: bool,
}

#[tokio::main]
async fn main() {
    dbg!({
        Command::new("pwd").spawn().expect("failed to exec pwd");
    });

    // load config from terminal
    let args = Args::parse();
    dbg!(&args.config_path);

    if args.is_test_config_mod {
        match Config::from(&args.config_path) {
            Ok(_) => {
                println!("Test Passed!");
                exit(0);
            }
            Err(err) => {
                println!("{}", err);
                exit(1);
            }
        };
    }

    run(args.config_path.to_owned()).await;
}
