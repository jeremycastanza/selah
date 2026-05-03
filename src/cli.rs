use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "selah", about = "A terminal Bible reader", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short = 'n', long = "no-banner")]
    pub no_banner: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Random {
        #[arg(short, long, default_value = "KJV")]
        translation: String,
    },
}
