use hotwatch::{Event, EventKind, Hotwatch};
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};

static LOG_FILE: &str = "./hotwatch.log";
static CONFIG: &str = "./config.json";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    files_to_watch: HashSet<String>,
}

fn setup_logger() -> Result<(), log::SetLoggerError> {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    let mut builder = env_logger::Builder::from_env(env);
    let file = File::create(LOG_FILE).unwrap();
    builder.target(env_logger::Target::Pipe(Box::new(file)));
    builder.init();
    Ok(())
}

fn load_config(config_name: &str) -> Result<HashSet<String>, io::Error> {
    let file = match File::open(config_name) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open config file '{}': {}", config_name, e);
            return Err(e);
        }
    };
    let reader = BufReader::new(file);

    let config: Config = serde_json::from_reader(reader)?;
    Ok(config.files_to_watch)
}

fn del_path(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
        info!("Directory deleted successfully!");
    } else if path.is_file() {
        fs::remove_file(path)?;
        info!("File deleted successfully!");
    } else {
        warn!("The provided path is neither a file nor a directory");
    }
    Ok(())
}

fn is_file_in_config(config: &HashSet<String>, file_name: &str) -> bool {
    config.contains(file_name)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logger().expect("Failed to initialize logger");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path-to-watch>", args[0]);
        error!("No args!");
        std::process::exit(1);
    }

    let config = load_config(CONFIG).expect("Failed to load config");

    let config = Arc::new(Mutex::new(config));
    let path_to_watch = &args[1];
    info!("Watching path: {}", path_to_watch);

    let mut hotwatch = Hotwatch::new()?;
    let (tx, rx) = channel();

    hotwatch.watch(path_to_watch, {
        let config = config.clone();
        move |event: Event| {
            let config = config.lock().unwrap();
            match event.kind {
                EventKind::Modify(_) => info!("File has been modified: {:?}", event.paths[0]),
                EventKind::Create(_) => {
                    info!("File has been created: {:?}", event.paths[0]);
                    if let Some(file_name) = event.paths[0].file_name().and_then(|f| f.to_str()) {
                        if is_file_in_config(&config, file_name) {
                            info!("File '{:?}' found in config!", event.paths[0]);
                            let path = event.paths[0].to_path_buf();
                            if let Err(e) = del_path(&path) {
                                error!("Error deleting path: {:?}", e);
                            }
                        }
                    }
                }
                _ => {}
            }
            tx.send(()).unwrap();
        }
    })?;

    loop {
        rx.recv()?;
    }
}
