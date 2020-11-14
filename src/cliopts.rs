use clap::Clap;

/// A command line utility to dump linux system info.
#[derive(Clap)]
#[clap(version = "0.1", author = "Jay Palat <jay@palat.net>")]
pub struct Opts {
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    pub json: i32,
}

