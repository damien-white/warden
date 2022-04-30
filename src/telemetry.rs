use std::fs::OpenOptions;
use std::{fs::File, io, path::Path, sync::Arc};

use tracing_subscriber::{filter::Directive, filter::LevelFilter, prelude::*, EnvFilter, Registry};

// TODO: Use `DEFAULT_DIRECTIVE` in initial `EnvFilter` builder
const DEFAULT_DIRECTIVE: &str = "warden=trace";

pub fn init_logging() -> io::Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(Directive::from(LevelFilter::DEBUG))
        .from_env()
        .expect("tracing directive(s) should be valid");

    // Create human-readable console logger
    let console_logger = tracing_subscriber::fmt::layer().pretty();

    let service_name = parse_service_name(DEFAULT_DIRECTIVE);
    let log_file = create_log_file(service_name)?;

    let file_logger = tracing_subscriber::fmt::layer()
        .with_writer(Arc::new(log_file))
        .json();

    Registry::default()
        .with(filter)
        .with(console_logger)
        .with(file_logger)
        .init();

    Ok(())
}

fn parse_service_name(target: &str) -> &str {
    match target.chars().position(|ch| ch == '=') {
        Some(i) => &target[..i],
        None => &target[..target.len()],
    }
}

fn create_log_file<P: AsRef<Path>>(path: P) -> io::Result<File> {
    let base_dir = std::env::current_dir()?.join("logs");
    let mut path = base_dir.join(path.as_ref());
    path.set_extension("json");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path)?;

    Ok(file)
}
