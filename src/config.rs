use serde::{Deserialize, Serialize};

/// `Config` stores the global configuration of the program.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub rules: Vec<String>,
    pub discard_copy_actions: bool,
    pub discard_identity: bool,
    pub discard_free_variable_expressions: bool,
    pub reduction_cutoff: usize,
    pub run_limit: usize,
    pub polling_interval: u32,
}

impl Config {
    pub fn from_config_str(s: &str) -> Config {
        serde_json::from_str(s).unwrap()
    }

    /// Produce a new `Config` with default 
    pub fn new() -> Self {
        Config {
            rules: vec![
                String::from("\\x.\\y.x y"),
                String::from("\\x.\\y.x"),
                String::from("\\x.\\y.y"),
            ],

            discard_copy_actions: true,
            discard_identity: true,
            discard_free_variable_expressions: true,
            reduction_cutoff: 100000,
            run_limit: 100000,
            polling_interval: 100,
        }
    }
}


