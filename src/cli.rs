use clap::App;

pub struct CLIParams {
    pub algorithm: String,
    pub compare_algorithm: String,

    pub compare: bool,

    pub executions: usize,
}

impl CLIParams {
    pub fn new() -> Self {
        let clap_yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(clap_yaml).get_matches();

        let algorithm = String::from(
            matches
                .value_of("ALGORITHM")
                .unwrap_or("NonexistentAlgorithm"),
        );
        let executions: usize = matches
            .value_of("executions")
            .unwrap_or("1") // 1 so that if parameter is not given, the default of one execution is used
            .parse()
            .unwrap_or(0); // 0 so that if invalid parameter is given, failure is set to true below
        let compare: bool = matches.is_present("compare");
        let compare_algorithm = String::from(
            matches
                .value_of("compare")
                .unwrap_or("NonexistentAlgorithm"),
        );

        Self {
            algorithm,
            compare_algorithm,

            compare,

            executions,
        }
    }
}
