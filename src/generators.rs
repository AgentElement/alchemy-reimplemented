use lambda_calculus::Term;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

struct LTree {
    n: u32,
    left: Option<Box<LTree>>,
    right: Option<Box<LTree>>,
}

impl LTree {
    fn new(n: u32) -> LTree {
        LTree {
            n,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, n: u32) {
        let child = LTree::new(n);
        match (&mut self.left, &mut self.right, n <= self.n) {
            (None, _, true) => self.left = Some(Box::new(child)),
            (_, None, false) => self.right = Some(Box::new(child)),
            (Some(t), _, true) | (_, Some(t), false) => t.insert(n),
        };
    }

    fn to_lambda_h(&self, freevar_p: u32, depth: usize) -> Term {
        match (&self.left, &self.right) {
            (None, None) => Term::Var(depth),
            (Some(t), None) | (None, Some(t)) => {
                Term::Abs(Box::new(t.to_lambda_h(freevar_p, depth + 1)))
            }
            (Some(l), Some(r)) => {
                let left = l.to_lambda_h(freevar_p, depth);
                let right = r.to_lambda_h(freevar_p, depth);
                Term::App(Box::new((left, right)))
            }
        }
    }

    pub fn to_lambda(&self, freevars_count: u32) -> Term {
        self.to_lambda_h(freevars_count, 0)
    }
}

struct BtreeGen {
    n_nodes: u32,
}

pub fn btree(n: u32, freevars_count: u32) -> Option<Term> {
    assert!(
        n > 0,
        "btree generator does not produce zero-sized expressions."
    );
    let mut rng = thread_rng();
    let mut permutation = (0..n).collect::<Vec<u32>>();
    permutation.shuffle(&mut rng);
    let mut tree = LTree::new(permutation[0]);
    permutation.iter().skip(1).for_each(|i| tree.insert(*i));
    Some(tree.to_lambda(freevars_count))
}

pub fn fontana(
    abs_range: (f64, f64),
    app_range: (f64, f64),
    depth_cutoff: u32,
    freevars_count: u32,
) -> Option<Term> {
    None
}
