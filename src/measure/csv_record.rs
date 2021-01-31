use serde::Serialize;

#[derive(Serialize)]
pub struct CSVRecord<'a> {
    #[serde(rename = "algorithm")]
    algorithm_name: &'a str,

    #[serde(rename = "text_length")]
    text_length: usize,
    #[serde(rename = "pattern_length")]
    pattern_length: usize,

    #[serde(rename = "execution")]
    execution: usize,

    #[serde(rename = "matches")]
    matches: usize,

    #[serde(rename = "prep_time_ms")]
    preparation_time_ms: u128,

    #[serde(rename = "time_ms")]
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
