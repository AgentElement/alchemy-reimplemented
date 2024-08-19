use std::collections::{HashMap, HashSet};

use crate::soup::Soup;

use lambda_calculus::Term;

impl Soup {
    // This is expensive, quadratic in the number of expressions. It can
    // probably be written to be faster, but it's not a bottleneck right now.
    pub fn unique_expressions(&self) -> HashSet<Term> {
        HashSet::<Term>::from_iter(self.expressions().cloned())
    }

    pub fn expression_counts(&self) -> HashMap<Term, u32> {
        let mut map = HashMap::<Term, u32>::new();
        for expr in self.expressions().cloned() {
            map.entry(expr).and_modify(|e| *e += 1).or_insert(1);
        }
        map
    }

    pub fn is_dominated_by_k_exprs(&self, _k: usize) -> bool {
        false
    }

    pub fn population_entropy(&self) -> f32 {
        let mut entropy = 0.0;
        let n = self.len() as f32;
        for (_, value) in self.expression_counts().iter() {
            let pi = (*value as f32) / n;
            entropy -= pi * pi.log10();
        }
        entropy
    }

    pub fn jacard_index(&self, other: &Soup) -> f32 {
        let selfcounts = self.expression_counts();
        let othercounts = other.expression_counts();

        let mut intersection = 0;
        for (k, v) in selfcounts {
            if let Some(c) = othercounts.get(&k) {
                intersection += c.min(&v);
            }
        }
        (intersection as f32) / ((self.len() + other.len()) as f32)
    }
}
