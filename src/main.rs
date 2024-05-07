use clap::Parser;
use lambda_calculus::data::num::church::pred;
use lambda_calculus::*;
use rand::{thread_rng, Rng};
use std::io::{self, BufRead, BufReader};

enum Filter {
    Identity,
    Unbound,
}

#[derive(Debug)]
struct FilterSet {
    identity: bool,
    unbound: bool,
}

impl FilterSet {
    fn new() -> Self {
        FilterSet {
            identity: false,
            unbound: false
        }
    }

    fn set(&mut self, f: Filter) {
        match f {
            Filter::Identity => self.identity = true,
            Filter::Unbound => self.unbound = true,
        };
    }
    
    fn unset(&mut self, f: Filter) {
        match f {
            Filter::Identity => self.identity = false,
            Filter::Unbound => self.unbound = false,
        };
    }

    fn object(&mut self, f: Filter) -> Term {
        match f {
            Filter::Identity => parse("\\x.x", Classic).unwrap(),
            Filter::Unbound => parse("\\x.y", Classic).unwrap(),
        }
    }
}


#[derive(Debug)]
struct Soup {
    expressions: Vec<Term>,
    reaction_rules: Vec<Term>,
    discard: bool,
    reduction_limit: usize,
    filter: FilterSet,
}

/// The result of composing a vector `v` of 2-ary lambda expressions with 
/// the expressions A and B.
struct ReactionResult {
    /// Size of each product
    pub sizes: Vec<u32>,

    /// Reduction steps
    pub reductions: Vec<usize>,

    /// Size of A
    pub left_size: u32,

    /// Size of B
    pub right_size: u32,
}


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {

    #[arg(short, long)]
    reduction_cutoff: Option<usize>,
    
    #[arg(short, long)]
    sample_frequency: Option<u32>,
    
}

impl Soup {
    fn new() -> Self {
        Soup {
            expressions: Vec::new(),
            reaction_rules: vec![
                parse("\\x.\\y.x y", Classic).unwrap(),
                parse("\\x.\\y.x", Classic).unwrap(),
                parse("\\x.\\y.y", Classic).unwrap(),
            ],
            discard: true,
            reduction_limit: 100000,
            filter: FilterSet::new(),
        }
    }

    fn set_limit(&mut self, limit: usize) {
        self.reduction_limit = limit;
    }

    fn add_filter(&mut self, filter: Filter) {
        self.filter.set(filter);
    }


    fn perturb(&mut self, expressions: &mut Vec<Term>) {
        self.expressions.append(expressions);
    }

    fn react(&mut self) -> Option<ReactionResult> {
        let mut rng = thread_rng();
        let n_expr = self.expressions.len();

        // Choose two distinct expressions randomly from the soup
        let i = rng.gen_range(0..n_expr);
        let left = &self.expressions.swap_remove(i);
        let left_size = left.max_depth();

        let j = rng.gen_range(0..n_expr - 1);
        let right = &self.expressions.swap_remove(j);
        let right_size = right.max_depth();

        // Record collision information 
        let mut buf = Vec::with_capacity(self.reaction_rules.len());
        let mut reductions = Vec::with_capacity(self.reaction_rules.len());
        let mut sizes = Vec::with_capacity(self.reaction_rules.len());

        // Collide expressions
        for rule in &self.reaction_rules {
            let result = self.collide(rule.clone(), left.clone(), right.clone());
            if let Some((value, n)) = result {
                sizes.push(value.max_depth());
                reductions.push(n);
                buf.push(value);
            } else {
                return None;
            }
        }

        // Add results to soup
        self.expressions.append(&mut buf);

        // Remove additional expressions, if there are more than two rules
        if self.discard {
            for _ in 0..(self.reaction_rules.len() - 2) {
                let k = rng.gen_range(0..self.expressions.len());
                self.expressions.swap_remove(k);
            }
        }

        // Return collision log
        Some(ReactionResult {
            sizes,
            reductions,
            left_size,
            right_size,
        })
    }

    fn simulate_for(&mut self, n: usize) {
        for i in 0..n {
            // print!("reaction {:?}", i);
            println!(
                "reaction {:?} {}",
                i,
                if let Some(result) = self.react() {
                    format!("successful with {} reductions between expressions of sizes {} and {}, and produces an expression of size {}",
                            result.left_size, result.right_size, result.reductions[0], result.sizes[0])
                } else {
                    "failed".to_string()
                }
            )
        }
    }

    fn remove_isomorphic_to(&mut self, copy: Term) -> bool {
        false
    }

    fn collide(&self, rule: Term, left: Term, right: Term) -> Option<(Term, usize)> {
        let mut expr = app!(rule, left, right);
        let n = expr.reduce(HNO, self.reduction_limit);
        if n == self.reduction_limit {
            None
        } else {
            Some((expr, n))
        }
    }
}


fn lambdac_example() {
    let mut expr = app!(pred(), 3.into_church());
    println!("{} order β-reduction steps for PRED 1 are:", NOR);

    println!("{}", expr);
    while expr.reduce(HNO, 1) != 0 {
        println!("{}", expr);
    }
}

fn read_inputs_into_soup() -> Soup {
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
    let mut soup = Soup::new();
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
