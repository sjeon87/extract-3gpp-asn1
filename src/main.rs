use clap::Parser;
use extract_3gpp_asn1::extract_asn1_blocks;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The file to process
    #[arg(value_name = "FILE")]
    path: std::path::PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let content = std::fs::read_to_string(cli.path).expect("could not read file");
    let extracted = extract_asn1_blocks(&content);
    println!("{}", extracted);
}