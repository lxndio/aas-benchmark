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

pub fn reset_text_access_counter() {
    *TEXT_ACCESSES.lock().unwrap() = 0;
}

pub fn text_access_counter() -> usize {
    *TEXT_ACCESSES.lock().unwrap()
}
