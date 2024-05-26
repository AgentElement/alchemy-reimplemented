use clap::Parser;
use lambda_calculus::*;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Write};

mod analysis;
mod config;
mod generators;
mod soup;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    reduction_cutoff: Option<usize>,

    #[arg(short, long)]
    sample_frequency: Option<u32>,

    #[arg(short, long)]
    config_file: Option<String>,

    #[arg(short, long)]
    make_default_config: bool,
}

fn read_inputs_into_soup(cfg: &config::Config) -> soup::Soup {
    let mut expression_strings = Vec::<String>::new();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        match line {
            Ok(line) => expression_strings.push(line),
            Err(_) => break,
        }
    }

    let expressions = expression_strings
        .iter()
        .map(|s| parse(s, Classic).unwrap());
    let mut soup = soup::Soup::from_config(cfg);
    soup.perturb(expressions);
    soup
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    if cli.make_default_config {
        let config_path = "config.json";
        let mut config_file = File::create(config_path)?;
        config_file.write_all(config::Config::new().to_config_str().as_bytes())?;
        return Ok(());
    }

    let config = if let Some(filename) = cli.config_file {
        let contents = read_to_string(filename)?;
        config::Config::from_config_str(&contents)
    } else {
        config::Config::new()
    };

    let mut soup = read_inputs_into_soup(&config);

    soup.set_limit(if let Some(cutoff) = cli.reduction_cutoff {
        cutoff
    } else {
        config.reduction_cutoff
    });

    soup.simulate_for(config.run_limit);
    println! {"Terminal soup state:\n{:?}", soup};
    Ok(())
}
