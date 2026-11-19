//! Executable entry point; the app itself lives in the library crate.

// Release builds run without a console window on Windows.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    pirates_parrot_lib::run();
}
