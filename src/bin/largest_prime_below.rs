use std::process;
use clap::Parser;
use prime_utils::largest_prime_below;

#[derive(Parser)]
#[command(
    name = "largest-prime",
    about = "Finds the largest prime below the given integer (unsigned 64 bit)",
    version
)]

struct Cli {
    /// Number to search below
    number: u64,
}


fn main() {
    let cli = Cli::parse();

    match largest_prime_below(cli.number) {
        Some(p) => {
            println!("{}", p);
        }
        None => {
            eprintln!("There is no prime less than {}", cli.number);
            process::exit(1);
        }
    }
}
