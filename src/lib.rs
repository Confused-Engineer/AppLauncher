#![warn(clippy::all, rust_2018_idioms)]
mod app;
pub use app::TemplateApp;
pub use app::Config;

mod page_config;
mod page_config_edit;
mod page_main;
mod page_api;