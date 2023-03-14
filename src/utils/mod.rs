#![allow(dead_code)]

use std::env;

pub fn get_cwd_string() -> String {
    match env::current_dir() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(_) => String::from(""),
    }
}