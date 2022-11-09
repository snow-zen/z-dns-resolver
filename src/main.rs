use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// Specify domain
    #[arg(short, long)]
    domain: String,
}

fn main() {
    let args: Cli = Cli::parse();
    println!("{}", args.domain)
}
