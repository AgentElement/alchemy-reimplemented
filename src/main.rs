use clap::Parser;
use lambda_calculus::data::num::church::pred;
use lambda_calculus::*;
use std::io::{self, BufRead, BufReader};

mod soup;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(short, long)]
    reduction_cutoff: Option<usize>,
    
    #[arg(short, long)]
    sample_frequency: Option<u32>,
    
}

fn lambdac_example() {
    let mut expr = app!(pred(), 3.into_church());
    println!("{} order β-reduction steps for PRED 1 are:", NOR);

    println!("{}", expr);
    while expr.reduce(HNO, 1) != 0 {
        println!("{}", expr);
    }
}

fn read_inputs_into_soup() -> soup::Soup {
    let mut expression_strings = Vec::<String>::new();
    let stdin = io::stdin();
    let reader = BufReader::new(stdin.lock());

    for line in reader.lines() {
        match line {
            Ok(line) => expression_strings.push(line),
            Err(_) => break,
        }
    }

    let mut expressions = expression_strings
        .iter()
        .map(|s| parse(s, Classic).unwrap())
        .collect::<Vec<Term>>();
    let mut soup = soup::Soup::new();
    soup.perturb(&mut expressions);
    soup
}

fn main() {
    let cli = Cli::parse();

    let mut soup = read_inputs_into_soup();

    if let Some(cutoff) = cli.reduction_cutoff {
        soup.set_limit(cutoff);
    }

    soup.simulate_for(100000);
    println! {"Terminal soup state:\n{:?}", soup}
}
