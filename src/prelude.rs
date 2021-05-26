use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref VERBOSE: Mutex<bool> = {
        let cell = Mutex::new(true);
        cell
    };
}