use core::fmt;
use std::fmt::{Debug, Display};

use crate::config;
use lambda_calculus::{abs, app, Term, Var};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// The principal AlChemy object. The `Soup` struct contains a set of
/// lambda expressions, and rules for composing and filtering them.
#[derive(Debug, Clone)]
pub struct Soup {
    expressions: Vec<Term>,
    n_collisions: usize,
    reaction_rules: Vec<Term>,
    reduction_limit: usize,
    size_limit: usize,

    maintain_constant_population_size: bool,
    discard_copy_actions: bool,
    discard_identity: bool,
    discard_free_variable_expressions: bool,
    discard_parents: bool,

    rng: ChaCha8Rng,
}

pub struct Tape {
    soup: Soup,
    history: Vec<Soup>,
    polling_interval: usize,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReactionError {
    ExceedsReductionLimit,
    NotEnoughExpressions,
    IsIdentity,
    IsParent,
    HasFreeVariables,
    ExceedsDepthLimit,
}

pub fn reduce_with_limit(
    expr: &mut Term,
    rlimit: usize,
    slimit: usize,
) -> Result<usize, ReactionError> {
    let mut n = 0;
    for _ in 0..rlimit {
        if expr.reduce(lambda_calculus::HAP, 1) == 0 {
            break;
        }

        // WARNING: This is EXTREMELY expensive. Calling max_depth is log(depth), and is done
        // per reduction step. Remove when possible.
        let depth = expr.size();
        if depth > slimit {
            return Err(ReactionError::ExceedsDepthLimit);
        }
        n += 1;
    }
    Ok(n)
}

impl Soup {
    /// Generate an empty soup with the following configuration options:
    pub fn new() -> Self {
        Soup::from_config(&config::Reactor::new())
    }

    /// Generate an empty soup from a given `config` object.
    pub fn from_config(cfg: &config::Reactor) -> Self {
        let seed = cfg.seed.get();
        let rng = ChaCha8Rng::from_seed(seed);
        Soup {
            expressions: Vec::new(),
            reaction_rules: cfg
                .rules
                .iter()
                .map(|r| lambda_calculus::parse(r, lambda_calculus::Classic).unwrap())
                .collect(),
            reduction_limit: cfg.reduction_cutoff,
            size_limit: cfg.size_cutoff,

            maintain_constant_population_size: cfg.maintain_constant_population_size,
            discard_copy_actions: cfg.discard_copy_actions,
            discard_parents: cfg.discard_parents,
            discard_identity: cfg.discard_identity,
            discard_free_variable_expressions: cfg.discard_free_variable_expressions,
            rng,
            n_collisions: 0,
        }
    }

    /// Set the reduction limit of the soup
    pub fn set_limit(&mut self, limit: usize) {
        self.reduction_limit = limit;
    }

    /// Introduce all expressions in `expressions` into the soup, without
    /// reduction.
    pub fn perturb(&mut self, expressions: impl IntoIterator<Item = Term>) {
        self.expressions
            .extend(expressions.into_iter().filter(|e| !e.has_free_variables()));
    }

    /// Return the result of ((`rule` `left`) `right`), up to a limit of
    /// `self.reduction_limit`.
    // TODO: return a proper error type instead of `String`.
    fn collide(&self, rule: Term, left: Term, right: Term) -> Result<(Term, usize), ReactionError> {
        let mut expr = app!(rule, left.clone(), right.clone());
        let n = reduce_with_limit(&mut expr, self.reduction_limit, self.size_limit)?;

        if n == self.reduction_limit {
            return Err(ReactionError::ExceedsReductionLimit);
        }

        let identity = abs(Var(1));
        if expr.is_isomorphic_to(&identity) && self.discard_identity {
            return Err(ReactionError::IsIdentity);
        }

        let is_copy_action = expr.is_isomorphic_to(&left) || expr.is_isomorphic_to(&right);
        if is_copy_action && self.discard_copy_actions {
            return Err(ReactionError::IsParent);
        }

        if expr.has_free_variables() && self.discard_free_variable_expressions {
            return Err(ReactionError::HasFreeVariables);
        }


        Ok((expr, n))
    }

    /// Produce one atomic reaction on the soup.
    fn react(&mut self) -> Result<ReactionResult, ReactionError> {
        let n_expr = self.expressions.len();

        if n_expr < 2 {
            return Err(ReactionError::NotEnoughExpressions);
        }

        // Remove two distinct expressions randomly from the soup
        let i = self.rng.gen_range(0..n_expr);
        let left = &self.expressions.swap_remove(i);
        let left_size = left.max_depth();

        let j = self.rng.gen_range(0..n_expr - 1);
        let right = &self.expressions.swap_remove(j);
        let right_size = right.max_depth();

        // Record collision information
        let mut buf = Vec::with_capacity(self.reaction_rules.len());
        let mut collision_results = Vec::with_capacity(self.reaction_rules.len());

        // Collide expressions
        //
        let mut n_successful_reactions = 0;
        for rule in &self.reaction_rules {
            let result = self.collide(rule.clone(), left.clone(), right.clone());
            self.n_collisions += 1;
            match result {
                Ok((value, n)) => {
                    let datum = CollisionResult {
                        reductions: n,
                        size: value.max_depth(),
                    };
                    collision_results.push(datum);
                    buf.push(value);
                    n_successful_reactions += 1;
                }
                Err(s) => {
                    if !self.discard_parents {
                        self.expressions.push(left.clone());
                        self.expressions.push(right.clone());
                    }
                    return Err(s);
                }
            }
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
            for _ in 0..n_successful_reactions {
                let k = self.rng.gen_range(0..self.expressions.len());
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

    fn log_failure_reaction(reaction: &Result<ReactionResult, ReactionError>) {
        if let Err(message) = reaction {
            println!("failed because {}", message)
        }
    }

    fn log_message_from_reaction(reaction: &Result<ReactionResult, ReactionError>) -> String {
        match reaction {
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
    }

    /// Simulate the soup for `n` collisions. If `log` is set, then print
    /// out a log message for each reaction. Returns the number of successful reactions
    /// (the fraction of failed reactions).
    pub fn simulate_for(&mut self, n: usize, log: bool) -> usize {
        let mut n_successes = 0;
        for _ in 0..n {
            let reaction = self.react();
            if reaction.is_ok() {
                n_successes += 1;
            }

            if log {
                // let message = Soup::log_message_from_reaction(&reaction);
                // println!("reaction {:?} {}", i, message)
                Soup::log_failure_reaction(&reaction);
            }
        }
        n_successes
    }

    pub fn simulate_and_poll<F, T>(
        &mut self,
        n: usize,
        polling_interval: usize,
        log: bool,
        poller: F,
    ) -> Vec<T>
    where
        F: Fn(&Self) -> T,
    {
        let mut data: Vec<T> = Vec::new();
        for i in 0..n {
            let reaction = self.react();
            if (i % polling_interval) == 0 {
                data.push(poller(self))
            }
            if log {
                let message = Soup::log_message_from_reaction(&reaction);
                println!("reaction {:?} {}", i, message)
            }
        }
        data
    }

    pub fn simulate_and_poll_with_killer<F, T>(
        &mut self,
        n: usize,
        polling_interval: usize,
        log: bool,
        killpoller: F,
    ) -> Vec<T>
    where
        F: Fn(&Self) -> (T, bool),
    {
        let mut data: Vec<T> = Vec::new();
        for i in 0..n {
            let reaction = self.react();
            if (i % polling_interval) == 0 {
                let (datum, should_kill) = killpoller(self);
                data.push(datum);
                if should_kill {
                    return data;
                };
            }
            if log {
                let message = Soup::log_message_from_reaction(&reaction);
                println!("reaction {:?} {}", i, message)
            }
        }
        data
    }

    /// Simulate the soup for `n` collisions, recording the state of the soup every
    /// `polling_interval` reactions. If `log` is set, then print out a log message for each
    /// reaction
    pub fn simulate_and_record(&mut self, n: usize, polling_interval: usize, log: bool) -> Tape {
        let mut history: Vec<Soup> = Vec::new();
        for i in 0..n {
            let reaction = self.react();
            if (i % polling_interval) == 0 {
                history.push(self.clone())
            }
            if log {
                let message = Soup::log_message_from_reaction(&reaction);
                println!("reaction {:?} {}", i, message)
            }
        }

        Tape {
            soup: self.clone(),
            history,
            polling_interval,
        }
    }

    /// Print out all expressions within the soup. Defaults to Church notation.
    /// If `debruijn_output` is set, then expressions are printed in DeBruijn
    /// notation.
    pub fn print(&self) {
        for expression in &self.expressions {
            println!("{}", expression)
        }
    }

    /// Get an iterator over all expressions.
    pub fn expressions(&self) -> impl Iterator<Item = &Term> {
        self.expressions.iter()
    }

    /// Get the number of expressions in the soup.
    pub fn len(&self) -> usize {
        self.expressions.len()
    }

    pub fn collisions(&self) -> usize {
        self.n_collisions
    }
}

impl Tape {
    pub fn final_state(&self) -> &Soup {
        &self.soup
    }

    pub fn history(&self) -> impl Iterator<Item = &Soup> {
        self.history.iter()
    }

    pub fn polling_interval(&self) -> usize {
        self.polling_interval
    }
}

/// Clippy asked me to do this
impl Default for Soup {
    fn default() -> Self {
        Soup::new()
    }
}

impl fmt::Display for ReactionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReactionError::IsIdentity => Display::fmt("collision result is identity function", f),
            ReactionError::IsParent => Display::fmt("collision result is isomorphic to parent", f),
            ReactionError::ExceedsReductionLimit => {
                Display::fmt("collision exceeds reduction limit", f)
            }
            ReactionError::NotEnoughExpressions => {
                Display::fmt("not enough expressions for further reactions", f)
            }
            ReactionError::HasFreeVariables => {
                Display::fmt("collision result has free variables", f)
            }

            ReactionError::ExceedsDepthLimit => {
                Display::fmt("expression exceeds depth limit during reduction", f)
            }
        }
    }
}

impl std::error::Error for ReactionError {}
