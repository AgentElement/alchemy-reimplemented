use serde::{Deserialize, Serialize};

/// `Config` stores the global configuration of the program.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rules: Vec<String>,
    pub discard_copy_actions: bool,
    pub discard_identity: bool,
    pub discard_free_variable_expressions: bool,
    pub discard_parents: bool,
    pub maintain_constant_population_size: bool,
    pub reduction_cutoff: usize,
    pub run_limit: usize,
    pub polling_interval: u32,
    pub print_reaction_results: bool,
    pub debrujin_output: bool,
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
            polling_interval: 100,
            print_reaction_results: false,
            debrujin_output: false,
        }
    }
}
