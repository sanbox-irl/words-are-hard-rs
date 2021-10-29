#![allow(clippy::bool_comparison)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gui;
mod words;

#[macro_use]
pub extern crate log;

pub use dauga::AnyResult;

fn main() -> AnyResult {
    #[cfg(debug_assertions)]
    dauga::initialize_pretty_env()?;

    // Time to Tango
    let mut tango = words::Words::new()?;
    tango.main_loop()?;

    info!("gracefully exited");

    Ok(())
}
