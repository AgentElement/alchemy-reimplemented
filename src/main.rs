use lambda_calculus::data::num::church::pred;
use lambda_calculus::*;
use rand::{thread_rng, Rng};
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
struct Soup {
    expressions: Vec<Term>,
    reaction_rules: Vec<Term>,
    discard: bool,
}

struct ReactionResult {
    pub sizes: Vec<u32>,
    pub reductions: Vec<usize>,
    pub left_size: u32,
    pub right_size: u32,
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
        }
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

        // Collide expressions and add results to soup
        let mut buf = Vec::with_capacity(self.reaction_rules.len());
        let mut reductions = Vec::with_capacity(self.reaction_rules.len());
        let mut sizes = Vec::with_capacity(self.reaction_rules.len());
        for rule in &self.reaction_rules {
            let result = collide(rule.clone(), left.clone(), right.clone());
            if let Some((value, n)) = result {
                sizes.push(value.max_depth());
                reductions.push(n);
                buf.push(value);
            } else {
                return None;
            }
        }

        self.expressions.append(&mut buf);

        // Remove additional expressions, if there are more than two rules
        if self.discard {
            for _ in 0..(self.reaction_rules.len() - 2) {
                let k = rng.gen_range(0..self.expressions.len());
                self.expressions.swap_remove(k);
            }
        }

        Some(ReactionResult {
            sizes,
            reductions,
            left_size,
            right_size,
        })
    }

    fn simulate_for(&mut self, n: usize) {
        for i in 0..n {
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
}

fn collide(rule: Term, left: Term, right: Term) -> Option<(Term, usize)> {
    let mut expr = app!(rule, left, right);
    let limit = 100000;
    let n = expr.reduce(HNO, limit);
    if n == limit {
        None
    } else {
        Some((expr, n))
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

fn main() {
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

    soup.simulate_for(100000);
    println! {"Terminal soup state:\n{:?}", soup}
}
