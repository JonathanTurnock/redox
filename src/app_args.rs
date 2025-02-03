use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AppArgs {
    /// Command to execute
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Serve the application
    Serve {
        /// Port to run the server on
        #[arg(short, long)]
        port: Option<u16>,
    },

    /// Run the application
    Run {
        /// Entrypoint to run
        entrypoint: String,
    },

    /// Run tests
    Test {
        /// Include tests to run as a regex pattern
        include: String,
    },
}