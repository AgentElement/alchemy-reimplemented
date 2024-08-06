use lambda_calculus::Term::{self, Abs};
use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

struct BTree {
    n: u32,
    left: Option<Box<BTree>>,
    right: Option<Box<BTree>>,
    var: Option<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum Standardization {
    Prefix,
    Postfix,
    None,
}

pub fn standardize(term: Term, std: Standardization) -> Term {
    term
}

impl BTree {
    fn new(n: u32) -> BTree {
        BTree {
            n,
            left: None,
            right: None,
            var: None,
        }
    }

    fn insert(&mut self, n: u32) {
        let child = BTree::new(n);
        match (&mut self.left, &mut self.right, n <= self.n) {
            (None, _, true) => self.left = Some(Box::new(child)),
            (_, None, false) => self.right = Some(Box::new(child)),
            (Some(t), _, true) | (_, Some(t), false) => t.insert(n),
        };
    }

    fn to_lambda_h(
        &self,
        rng: &mut ChaCha8Rng,
        freevar_p: f64,
        max_free_vars: u32,
        depth: u32,
    ) -> Term {
        match (&self.left, &self.right) {
            (None, None) => {
                let var = if rng.gen_bool(freevar_p) || depth == 0 {
                    depth + rng.gen_range(1..=max_free_vars)
                } else {
                    rng.gen_range(1..=depth)
                };
                Term::Var(var as usize)
            }
            (Some(t), None) | (None, Some(t)) => Term::Abs(Box::new(t.to_lambda_h(
                rng,
                freevar_p,
                max_free_vars,
                depth + 1,
            ))),
            (Some(l), Some(r)) => {
                let left = l.to_lambda_h(rng, freevar_p, max_free_vars, depth);
                let right = r.to_lambda_h(rng, freevar_p, max_free_vars, depth);
                Term::App(Box::new((left, right)))
            }
        }
    }

    fn to_lambda(&self, rng: &mut ChaCha8Rng, freevar_p: f64, max_free_vars: u32) -> Term {
        self.to_lambda_h(rng, freevar_p, max_free_vars, 0)
    }
}

pub struct BTreeGen {
    n: u32,
    freevar_p: f64,
    max_free_vars: u32,
    std: Standardization,

    seed: [u8; 32],
    rng: ChaCha8Rng,
}

impl BTreeGen {
    pub fn new() -> BTreeGen {
        let seed = [0; 32];
        BTreeGen::from_seed(seed)
    }

    pub fn from_seed(seed: [u8; 32]) -> BTreeGen {
        let rng = ChaCha8Rng::from_seed(seed);
        BTreeGen {
            n: 20,
            freevar_p: 0.2,
            max_free_vars: 6,
            std: Standardization::None,
            seed,
            rng,
        }
    }

    pub fn generate(&mut self) -> Term {
        let n = self.n;
        assert!(
            n > 0,
            "btree generator does not produce zero-sized expressions."
        );
        let mut permutation = (0..n).collect::<Vec<u32>>();
        permutation.shuffle(&mut self.rng);
        let mut tree = BTree::new(permutation[0]);
        permutation.iter().skip(1).for_each(|i| tree.insert(*i));
        let lambda = tree.to_lambda(&mut self.rng, self.freevar_p, self.max_free_vars);
        BTreeGen::prefix_standardize(lambda)
    }

    fn postfix_standardize(t: Term) {
    }

    fn prefix_standardize(mut t: Term) -> Term {
        // This is horrible
        while t.has_free_variables() {
            t = Abs(Box::new(t))
        }
        t
    }
}

pub struct FontanaGen {
    abs_range: (f64, f64),
    app_range: (f64, f64),
    depth_cutoff: u32,
    freevars_count: u32,

    seed: [u8; 32],
    rng: ChaCha8Rng,
}

impl FontanaGen {
    pub fn generate(&self) -> Option<Term> {
        None
    }

    pub fn set_seed(&mut self, seed: [u8; 32]) {
        self.seed = seed;
    }
}
