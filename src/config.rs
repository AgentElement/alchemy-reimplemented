use lambda_calculus::Term;

/// `Config` stores the global configuration of the program.
struct Config {
    rules: Vec<Term>,
    discard_copy_actions: bool,
    discard_identity: bool,
    discard_free_variable_expressions: bool,
    reduction_cutoff: u32,
    run_limit: u32,
    polling_interval: u32,
}

impl Config {
    fn from_config_file(s: &str) -> Self {
    }

    fn new() -> Self {
        Config {
            rules: vec![
                lambda_calculus::parse("\\x.\\y.x y", lambda_calculus::Classic).unwrap(),
                lambda_calculus::parse("\\x.\\y.x", lambda_calculus::Classic).unwrap(),
                lambda_calculus::parse("\\x.\\y.y", lambda_calculus::Classic).unwrap(),
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


