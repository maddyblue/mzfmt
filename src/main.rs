use std::io::{self, Read};

use clap::AppSettings;
use mz_sql_pretty::pretty_str;
use structopt::StructOpt;

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
    println!("{}", pretty_str(&buffer, opt.width).unwrap());
}
