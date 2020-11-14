use serde::Serialize;

#[derive(Serialize)]
pub struct CSVRecord<'a> {
    #[serde(rename = "Algorithm")]
    algorithm_name: &'a str,

    #[serde(rename = "Text Length")]
    text_length: usize,
    #[serde(rename = "Pattern Length")]
    pattern_length: usize,

    #[serde(rename = "Execution")]
    execution: usize,

    #[serde(rename = "Matches")]
    matches: usize,

    #[serde(rename = "Preparation Time [ms]")]
    preparation_time_ms: u128,

    #[serde(rename = "Time [ms]")]
    time_ms: u128,
}

impl<'a> CSVRecord<'a> {
    pub fn new(
        algorithm_name: &'a str,
        text_length: usize,
        pattern_length: usize,
        execution: usize,
        matches: usize,
        preparation_time_ms: u128,
        time_ms: u128,
    ) -> Self {
        Self {
            algorithm_name,

            text_length,
            pattern_length,

            execution,

            matches,

            preparation_time_ms,
            time_ms,
        }
    }
}
