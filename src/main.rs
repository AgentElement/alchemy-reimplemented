use lambda_calculus::data::num::church::pred;
use lambda_calculus::*;
use rand::{seq::IteratorRandom, thread_rng, Rng};
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

    fn react(&mut self) {
        let mut rng = thread_rng();

        let n_expr = self.expressions.len();

        // Choose two distinct expressions randomly from the soup
        let i = rng.gen_range(0..n_expr);
        let left = &self.expressions.remove(i);
        
        let j = rng.gen_range(0..n_expr - 1);
        let right = &self.expressions.remove(j);

        println!("left: {:?}", left);
        println!("right: {:?}", right);

        // Collide expressions and add results to soup
        for rule in &self.reaction_rules {
            let result = collide(rule.clone(), left.clone(), right.clone());
            println!("result: {:?}", result);
            self.expressions.push(result);
        }

        // Remove additional expressions, if there are more than two rules 
        if self.discard {
            for _ in 0..(self.reaction_rules.len() - 2) {
                let k = rng.gen_range(0..self.expressions.len());
                self.expressions.remove(k);
                println!("removed: {:?}", right);
            }
        }
    }

    fn simulate_for(&mut self, n: usize) {
        for _ in 0..n {
            self.react();
            // self.expressions.iter().filter(|e| is_identity(e))
        }
    }
}

fn collide(rule: Term, left: Term, right: Term) -> Term {
    let mut expr = app!(rule, left, right);
    expr.reduce(HNO, 100000);
    expr
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

    println!{"Soup begins in state:\n{:?}", soup}
    soup.simulate_for(100);
    println!{"Terminal soup state:\n{:?}", soup}
}
