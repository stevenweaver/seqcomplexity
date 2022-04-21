mod process;

use clap::{App, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("seqcomplexity")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Steven Weaver")
        .about("Calculates Per-Read and Total Sequence Complexity from FastQ file.")
        .arg(
            Arg::new("fastq")
                .help("The input FASTQ file (gzip acceptable).")
                .takes_value(true)
                .required(true)
                .short('q')
                .long("fastq"),
        )
        .get_matches();

    crate::process::process(matches.value_of("fastq").unwrap())
}
