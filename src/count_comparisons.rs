use std::sync::Mutex;

lazy_static! {
    pub static ref COMPARISONS: Mutex<usize> = Mutex::new(0);
}

#[cfg(feature = "countcomparisons")]
#[macro_export]
macro_rules! eq {
    ($a:expr, $b:expr) => {{
        *COMPARISONS.lock().unwrap() += 1;

        $a == $b
    }};
}

#[cfg(not(feature = "countcomparisons"))]
#[macro_export]
macro_rules! eq {
    ($a:expr, $b:expr) => {
        $a == $b
    };
}

#[cfg(feature = "countcomparisons")]
#[macro_export]
macro_rules! neq {
    ($a:expr, $b:expr) => {{
        *COMPARISONS.lock().unwrap() += 1;

        $a != $b
    }};
}

#[cfg(not(feature = "countcomparisons"))]
#[macro_export]
macro_rules! neq {
    ($a:expr, $b:expr) => {
        $a != $b
    };
}

#[cfg(feature = "countcomparisons")]
#[macro_export]
macro_rules! eqs {
    ($a:expr, $b:expr) => {{
        let mut comparisons = 0;
        let mut success = true;

        for (a, b) in $a.iter().zip($b.iter()) {
            comparisons += 1;

            if a != b {
                success = false;
                break;
            }
        }

        *COMPARISONS.lock().unwrap() += comparisons;

        success
    }};
}

#[cfg(not(feature = "countcomparisons"))]
#[macro_export]
macro_rules! eqs {
    ($a:expr, $b:expr) => {
        $a == $b
    };
}

#[cfg(feature = "countcomparisons")]
#[macro_export]
macro_rules! neqs {
    ($a:expr, $b:expr) => {{
        let mut comparisons = 0;
        let mut success = true;

        for (a, b) in $a.iter().zip($b.iter()) {
            comparisons += 1;

            if a == b {
                success = false;
                break;
            }
        }

        *COMPARISONS.lock().unwrap() += comparisons;

        success
    }};
}

#[cfg(not(feature = "countcomparisons"))]
#[macro_export]
macro_rules! neqs {
    ($a:expr, $b:expr) => {
        $a != $b
    };
}

pub fn reset_comparison_counter() {
    *COMPARISONS.lock().unwrap() = 0;
}

pub fn comparison_counter() -> usize {
    *COMPARISONS.lock().unwrap()
}
