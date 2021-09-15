use std::io::{self, Read};

use clap::AppSettings;
use structopt::StructOpt;

use mzfmt::pretty_str;

/// Reads SQL from stdin, formats at specified width, and outputs to stdout.
#[derive(StructOpt)]
#[structopt(settings = &[AppSettings::UnifiedHelpMessage], usage = "mzfmt [OPTIONS]")]
struct Opt {
    /// Target output width
    #[structopt(short, long, default_value = "60")]
    width: usize,
}

fn main() {
    let opt = Opt::from_args();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{}", pretty_str("select 1,2", opt.width).unwrap());
}
