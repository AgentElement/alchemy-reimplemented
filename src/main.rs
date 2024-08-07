use clap::Parser;
use generators::BTreeGen;
use lambda_calculus::*;
use std::fs::{read_to_string, File};
use std::io::{self, BufRead, BufReader, Write};

/// Simulation analysis
mod analysis;

/// Global configuration
mod config;

/// Random expression generators
mod generators;

/// Main AlChemy simulation module
mod soup;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Fail a reaction if it takes more than `reduction_cutoff` steps to reduce
    #[arg(short = 'f', long)]
    reduction_cutoff: Option<usize>,

    /// When set, generate a tape that snapshots the state of the reactor every `polling_interval`
    /// reactions
    #[arg(short, long)]
    polling_interval: Option<usize>,

    /// Explicit path to configuration file
    #[arg(short, long)]
    config_file: Option<String>,

    /// Dump out the current config and exit
    #[arg(long)]
    dump_config: bool,

    /// Make a default config file in the current directory and exit
    #[arg(short, long)]
    make_default_config: bool,

    /// Number of reactions to run before printing out final soup
    #[arg(short, long)]
    run_limit: Option<usize>,

    /// Log each reaction
    #[arg(long)]
    log: bool,

    #[arg(long)]
    generate: Option<u32>,
}

/// Read lambda expressions from stdin and create a new soup from the global configuration
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

    if cli.dump_config {
        println!("{}", config.to_config_str());
        return Ok(());
    }

    if let Some(n) = cli.generate {
        let mut gen = BTreeGen::new();
        for _ in 0..n {
            println!("{:?}", gen.generate())
        }
        return Ok(());
    }

    let mut soup = read_inputs_into_soup(&config);

    soup.set_limit(if let Some(cutoff) = cli.reduction_cutoff {
        cutoff
    } else {
        config.reduction_cutoff
    });

    let limit = if let Some(run_limit) = cli.run_limit {
        run_limit
    } else {
        config.run_limit
    };

    let log = cli.log || config.print_reaction_results;

    if let Some(polling_interval) = cli.polling_interval {
        let tape = soup.simulate_and_record(limit, polling_interval, log);
        for soup in tape.history() {
            println!("{}", soup.population_entropy());
        }
    } else {
        soup.simulate_for(limit, log);
        soup.print();
    }

    Ok(())
}
