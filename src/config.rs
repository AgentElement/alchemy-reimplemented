use serde::{Deserialize, Serialize};

/// `Config` stores the global configuration of the program.
#[warn(missing_docs)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Set of reaction rules. Each rule must always be a lambda expressions
    /// with two arguments. Default: `["\x.\y.x y"]`.
    pub rules: Vec<String>,

    /// When set, remove all results that are structurally isomorphic to parents.
    /// Default: `true`.
    pub discard_copy_actions: bool,

    /// When set, remove all results that are structurally isomorphic to the identity function:
    /// `\x.x`. Default: `true`.
    pub discard_identity: bool,

    /// When set, remove all expressions that contain free variables. Default: `true`.
    pub discard_free_variable_expressions: bool,

    /// When set, remove the parents from the soup instead of returning them. Default: `true`.
    pub discard_parents: bool,

    /// When set maintain a constant population size after each reaction. If there are more
    /// elements than the population originally started with, then remove elements randomly from
    /// the soup until the original population remains. If there are fewer elements after a
    /// reaction, then do nothing. This behavior may change. Default: `true`.
    pub maintain_constant_population_size: bool,

    ///  The number of reductions allowed before AlChemy gives up and fails the reaction. Default:
    ///  `500`.
    pub reduction_cutoff: usize,

    /// The number of reactions to run for this simulation. Default: `100000`
    pub run_limit: usize,

    /// Print out the state of the soup every `polling_interval` reactions. When set to `None`, never
    /// poll. Default: `None`
    pub polling_interval: Option<usize>, // TODO

    /// When set, print out all logs for each individual reaction. Default: `false`
    pub print_reaction_results: bool,

    /// When set, print out the soup in debruijn notation. Default: `false`
    pub debrujin_output: bool,

    /// The seed for the lambda expression generator. If set to `None`, then a seed is chosen
    /// randomly. Default: `None`
    pub generator_seed: Option<[u8; 32]>, // TODO

    /// The seed for the reactor. If set to `None`, then a seed is chosen
    /// randomly. Default: `None`
    pub reactor_seed: Option<[u8; 32]>,
}

impl Config {
    pub fn from_config_str(s: &str) -> Config {
        serde_json::from_str(s).unwrap()
    }

    pub fn to_config_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Produce a new `Config` struct with default values.
    pub fn new() -> Self {
        Config {
            rules: vec![String::from("\\x.\\y.x y")],

            discard_copy_actions: true,
            discard_identity: true,
            discard_free_variable_expressions: true,
            maintain_constant_population_size: true,
            discard_parents: false,
            reduction_cutoff: 500,
            run_limit: 100000,
            polling_interval: None,
            print_reaction_results: false,
            debrujin_output: false,
            generator_seed: None,
            reactor_seed: None,
        }
    }
}
