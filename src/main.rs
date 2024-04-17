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

    fn react(&mut self) -> Option<Vec<usize>> {
        let mut rng = thread_rng();

        let n_expr = self.expressions.len();

        // Choose two distinct expressions randomly from the soup
        let i = rng.gen_range(0..n_expr);
        let left = &self.expressions.swap_remove(i);

        let j = rng.gen_range(0..n_expr - 1);
        let right = &self.expressions.swap_remove(j);

        // println!("left: {:?}", left);
        // println!("right: {:?}", right);

        // Collide expressions and add results to soup
        let mut buf = Vec::with_capacity(self.reaction_rules.len());
        let mut collisions = Vec::with_capacity(self.reaction_rules.len());
        for rule in &self.reaction_rules {
            let result = collide(rule.clone(), left.clone(), right.clone());
            if let Some((value, n)) = result {
                buf.push(value);
                collisions.push(n);
            } else {
                return None;
            }
            // println!("result: {:?}", result);
        }
        self.expressions.append(&mut buf);

        // Remove additional expressions, if there are more than two rules
        if self.discard {
            for _ in 0..(self.reaction_rules.len() - 2) {
                let k = rng.gen_range(0..self.expressions.len());
                self.expressions.swap_remove(k);
                // println!("removed: {:?}", right);
            }
        }

        Some(collisions)
    }

    fn simulate_for(&mut self, n: usize) {
        for i in 0..n {
            println!(
                "reaction {:?} {}",
                i,
                if self.react().is_some() { "successful" } else { "failed" }
            )
            // self.expressions.iter().filter(|e| is_identity(e))
        }
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

    soup.simulate_for(10000);
    println! {"Terminal soup state:\n{:?}", soup}
}
