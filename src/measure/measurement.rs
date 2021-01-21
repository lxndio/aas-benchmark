use std::time::Duration;

/// A single measurement containing an optional preparation runtime,
/// a mandatory execution runtime (of the actual pattern matching algorithm
/// itself) and the number of matches, i. e. how often the pattern has been
/// found in the text.
pub type SingleMeasurement = (Option<Duration>, Duration, usize);
