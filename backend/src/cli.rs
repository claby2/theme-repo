use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug, Clone, Default)]
pub struct Args {
    /// Path to themes directory
    #[clap(long = "themes", default_value = "./themes")]
    pub themes_path: PathBuf,

    /// Path to templates directory
    #[clap(long = "templates", default_value = "./templates")]
    pub templates_path: PathBuf,
}
