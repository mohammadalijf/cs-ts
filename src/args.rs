use crate::region::Region;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// API key of target stack
    #[arg(short = 'k', long)]
    pub api_key: String,

    /// Access Token of target stack
    #[arg(short = 't', long)]
    pub access_token: String,

    /// Optional, The path that cli should dump the result. if none is passed it will print into std
    #[arg(short = 'o', long)]
    pub output: Option<String>,

    /// Optional, The region for api call
    #[arg(short = 'r', long, default_value = "europe")]
    pub region: Region,

    #[arg(long)]
    pub prefix: Option<String>,

    #[arg(long)]
    pub postfix: Option<String>,
}
