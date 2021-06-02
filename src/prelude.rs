use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VERBOSE: Mutex<bool> = {
        let cell = Mutex::new(true);
        cell
    };
}

#[macro_export]
macro_rules! everboseln {
    ($($arg:tt)*) => ({
        if *VERBOSE.lock().unwrap() == true {
            eprintln!($($arg)*);
        }
    })
}

pub fn verbose_do<F>(f: F) where F: FnOnce() -> () {
    if *VERBOSE.lock().unwrap() == true {
        f();
    }
}
