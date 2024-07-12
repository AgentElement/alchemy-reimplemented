use std::collections::{HashMap, HashSet};

use crate::soup::Soup;

use lambda_calculus::Term;

struct Property {
    n: usize,
    rhs: Vec<usize>,
}

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

    fn find_functions_with_property(&self, property: &Property) {}
}
