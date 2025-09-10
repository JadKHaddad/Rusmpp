use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(
        long,
        env = "RUSMPPS_CONFIG_FILE",
        default_value = "rusmpps-config.yaml"
    )]
    /// Config file: The path to the configuration file
    pub config_file: PathBuf,
}
