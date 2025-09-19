// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod commands;
pub mod config;
pub mod core;
pub mod error;
pub mod hotkeys;
pub mod menu;
pub mod providers;
pub mod terminal;
pub mod tray;
pub mod ui;
pub mod utils;
pub mod tools;

pub use app::HoverShellApp;
pub use error::{HoverShellError, Result};