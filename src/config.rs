use serde::{Deserialize, Serialize};

/// `Config` stores the global configuration of the program.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// Set of reaction rules
    pub rules: Vec<String>,

    /// When set, remove all results that are structurally isomorphic to parents
    pub discard_copy_actions: bool,

    /// When set, remove all results that are structurally isomorphic to the identity function:
    /// `\x.x`
    pub discard_identity: bool,

    /// When set, remove all expressions that contain free variables
    pub discard_free_variable_expressions: bool,

    /// When set, remove the parents from the soup instead of returning them
    pub discard_parents: bool,

    /// When set, if there are more 
    pub maintain_constant_population_size: bool,

    ///  The number of reductions allowed before AlChemy gives up and fails the reaction
    pub reduction_cutoff: usize,

    /// The number of reactions to run
    pub run_limit: usize,

    /// Print out the state of the soup every `polling_interval` reactions. When set to 0, never
    /// poll. 
    pub polling_interval: u32, // TODO

    /// When set, print out all logs for each individual reaction
    pub print_reaction_results: bool,

    /// When set, print out the soup in debruijn notation
    pub debrujin_output: bool,

    /// The seed for the random number generator.
    pub seed: u32, // TODO
}

impl Config {
    pub fn from_config_str(s: &str) -> Config {
        serde_json::from_str(s).unwrap()
    }

    pub fn to_config_str(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    /// Produce a new `Config` with default
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
            polling_interval: 0,
            print_reaction_results: false,
            debrujin_output: false,
            seed: 1000,
        }
    }
}
