use clap::Parser;

// ref.: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
#[derive(Parser)]
#[command(name = "Search")]
#[command(author = "n4zz4r1 <nazzari_red@pm.me>")]
#[command(author, version, about)]
pub struct Cli {
    /// Directory only.
    #[arg(short = 'd', default_value_t = false)]
    pub only_dir: bool,
    /// Deep
    #[arg(long = "deep", default_value_t = 0)]
    pub deep: i32,
    /// Name with wildcard auto
    pub name: String,
    /// Search Path
    #[arg( default_value = ".")]
    pub path: String,
}
