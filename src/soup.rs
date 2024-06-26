use crate::config;
use lambda_calculus::{abs, app, Term, Var};
use rand::{thread_rng, Rng};

/// The principal AlChemy object. The `Soup` struct contains a set of
/// lambda expressions, and rules for composing and filtering them.
#[derive(Debug)]
pub struct Soup {
    expressions: Vec<Term>,
    reaction_rules: Vec<Term>,
    reduction_limit: usize,

    maintain_constant_population_size: bool,
    discard_copy_actions: bool,
    discard_identity: bool,
    discard_free_variable_expressions: bool,
    discard_parents: bool,
}

/// Stores the size and number of reductions for a collision
struct CollisionResult {
    pub size: u32,
    pub reductions: usize,
}

/// The result of composing a vector `v` of 2-ary lambda expressions with
/// the expressions A and B.
struct ReactionResult {
    pub collision_results: Vec<CollisionResult>,

    /// Size of A
    pub left_size: u32,

    /// Size of B
    pub right_size: u32,
}

impl Soup {
    /// Generate an empty soup with the following configuration options:
    ///
    pub fn new() -> Self {
        Soup::from_config(&config::Config::new())
    }

    /// Generate an empty soup from a given `config` object.
    pub fn from_config(cfg: &config::Config) -> Self {
        Soup {
            expressions: Vec::new(),
            reaction_rules: cfg
                .rules
                .iter()
                .map(|r| lambda_calculus::parse(r, lambda_calculus::Classic).unwrap())
                .collect(),
            reduction_limit: cfg.reduction_cutoff,

            maintain_constant_population_size: cfg.maintain_constant_population_size,
            discard_copy_actions: cfg.discard_copy_actions,
            discard_parents: cfg.discard_parents,
            discard_identity: cfg.discard_identity,
            discard_free_variable_expressions: cfg.discard_free_variable_expressions,
        }
    }

    /// Set the reduction limit of the soup
    pub fn set_limit(&mut self, limit: usize) {
        self.reduction_limit = limit;
    }

    /// Introduce all expressions in `expressions` into the soup, without
    /// reduction.
    pub fn perturb(&mut self, expressions: impl IntoIterator<Item=Term>) {
        self.expressions
            .extend(expressions.into_iter().filter(|e| !e.has_free_variables()));
    }

    /// Return the result of ((`rule` `left`) `right`), up to a limit of
    /// `self.reduction_limit`.
    // TODO: return a proper error type instead of `String`.
    fn collide(&self, rule: Term, left: Term, right: Term) -> Result<(Term, usize), String> {
        let mut expr = app!(rule, left.clone(), right.clone());
        let n = expr.reduce(lambda_calculus::HNO, self.reduction_limit);
        if n == self.reduction_limit {
            return Err(String::from("collision exceeds reduction limit"));
        }

        let identity = abs(Var(1));
        if expr.is_isomorphic_to(&identity) && self.discard_identity {
            return Err(String::from("collision result is identity function"));
        }

        let is_copy_action = expr.is_isomorphic_to(&left) || expr.is_isomorphic_to(&right);
        if is_copy_action && self.discard_copy_actions {
            return Err(String::from("collision result is isomorphic to parent"));
        }

        if expr.has_free_variables() && self.discard_free_variable_expressions {
            return Err(String::from("collision result has free variables"));
        }

        Ok((expr, n))
    }

    // TODO: This is a huge monolith, decompose into something neater
    /// Produce one atomic reaction on the soup.
    fn react(&mut self) -> Result<ReactionResult, String> {
        let mut rng = thread_rng();
        let n_expr = self.expressions.len();

        // Remove two distinct expressions randomly from the soup
        let i = rng.gen_range(0..n_expr);
        let left = &self.expressions.swap_remove(i);
        let left_size = left.max_depth();

        let j = rng.gen_range(0..n_expr - 1);
        let right = &self.expressions.swap_remove(j);
        let right_size = right.max_depth();

        // Record collision information
        let mut buf = Vec::with_capacity(self.reaction_rules.len());
        let mut collision_results = Vec::with_capacity(self.reaction_rules.len());

        // Collide expressions
        for rule in &self.reaction_rules {
            let (value, n) = self.collide(rule.clone(), left.clone(), right.clone())?;
            let datum = CollisionResult {
                reductions: n,
                size: value.max_depth(),
            };
            collision_results.push(datum);
            buf.push(value);
        }

        // Add collision results to soup
        self.expressions.append(&mut buf);

        // Add removed parents back into the soup, if necessary
        if !self.discard_parents {
            self.expressions.push(left.clone());
            self.expressions.push(right.clone());
        }

        // Remove additional expressions, if required.
        if self.maintain_constant_population_size {
            for _ in 0..(self.reaction_rules.len()) {
                let k = rng.gen_range(0..self.expressions.len());
                self.expressions.swap_remove(k);
            }
        }

        // Return collision log
        Ok(ReactionResult {
            collision_results,
            left_size,
            right_size,
        })
    }

    /// Simulate the soup for `n` collisions.
    pub fn simulate_for(&mut self, n: usize) {
        for i in 0..n {
            println!(
                "reaction {:?} {}",
                i,
                match self.react() {
                    Ok(result) => format!(
                        "successful with {} reductions between expressions of
                        sizes {} and {}, and produces an expression of size {}",
                        result.left_size,
                        result.right_size,
                        result.collision_results[0].reductions,
                        result.collision_results[0].size
                    ),
                    Err(message) => format!("failed because {}", message),
                }
            )
        }
    }
}
