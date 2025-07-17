use clap::{Parser, ValueEnum};
use pdf_extract;
use reqwest;
use std::error::Error;

#[derive(Debug, Clone, ValueEnum)]
enum InputKind {
    /// Input from file on disk
    File,
    /// Download input from web address
    Web,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Selected input method
    #[arg(value_enum, default_value_t = InputKind::File, short, long)]
    method: InputKind,

    /// Set byte limit on output
    #[arg(default_value_t = 100_000, short, long)]
    limit: usize,

    /// Ignore byte limit
    #[arg(default_value_t = false, short, long)]
    ignore_limit: bool,

    /// Input source
    input: String,
}

fn download(url: String) -> Result<Vec<u8>, Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let response = client.get(url).send()?;

    if response.status().is_success() {
        Ok(response.bytes()?.to_vec())
    } else {
        Err(format!("Request failed with status: {}", response.status()).into())
    }
}

fn extract(method: InputKind, input: String) -> Result<String, Box<dyn Error>> {
    let ret = match method {
        InputKind::File => pdf_extract::extract_text(input),
        InputKind::Web => pdf_extract::extract_text_from_mem(&download(input)?),
    };

    ret.map_err(|e| e.into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let mut ret = extract(args.method, args.input)?;

    if args.ignore_limit {
        println!("{}", ret);
    } else {
        ret.truncate(args.limit);
        println!("{}", ret);
    }

    Ok(())
}
