use lambda_calculus::Term;
use rand::{seq::SliceRandom, SeedableRng};
use rand_chacha::ChaCha8Rng;

struct LTree {
    n: u32,
    left: Option<Box<LTree>>,
    right: Option<Box<LTree>>,
    var: Option<usize>,
}

#[derive(Copy, Clone, Debug)]
pub enum Standardization {
    Prefix,
    Postfix,
}

impl LTree {
    fn new(n: u32) -> LTree {
        LTree {
            n,
            left: None,
            right: None,
            var: None,
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

    fn standardize(&mut self, std: Standardization) {}

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

    pub fn to_lambda(&self, n: u32, freevar_p: u32, std: Standardization) -> Term {
        self.to_lambda_h(freevar_p, 0)
    }
}

struct BTreeGen {
    n: u32,
    freevar_p: u32,
    std: Standardization,

    seed: [u8; 32],
    rng: ChaCha8Rng,
}

impl BTreeGen {
    fn generate(&self) -> Option<Term> {
        let n = self.n;
        assert!(
            n > 0,
            "btree generator does not produce zero-sized expressions."
        );
        let mut rng = ChaCha8Rng::from_seed(self.seed);
        let mut permutation = (0..n).collect::<Vec<u32>>();
        permutation.shuffle(&mut rng);
        let mut tree = LTree::new(permutation[0]);
        permutation.iter().skip(1).for_each(|i| tree.insert(*i));
        Some(tree.to_lambda(n, self.freevar_p, self.std))
    }

    fn set_seed(&mut self, seed: [u8; 32]) {
        self.seed = seed;
    }
}

struct FontanaGen {
    abs_range: (f64, f64),
    app_range: (f64, f64),
    depth_cutoff: u32,
    freevars_count: u32,

    seed: [u8; 32],
    rng: ChaCha8Rng,
}

impl FontanaGen {
    fn generate(&self) -> Option<Term> {
        None
    }

    fn set_seed(&mut self, seed: [u8; 32]) {
        self.seed = seed;
    }
}
