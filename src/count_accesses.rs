use std::sync::Mutex;

lazy_static! {
    pub static ref TEXT_ACCESSES: Mutex<usize> = Mutex::new(0);
}

#[cfg(feature = "countaccesses")]
#[macro_export]
macro_rules! get {
    ($a:expr, $b:expr) => {{
        *TEXT_ACCESSES.lock().unwrap() += 1;

        $a[$b]
    }};
}

#[cfg(not(feature = "countaccesses"))]
#[macro_export]
macro_rules! get {
    ($a:expr, $b:expr) => {
        $a[$b]
    };
}
