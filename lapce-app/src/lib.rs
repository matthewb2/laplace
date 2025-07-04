pub mod about;
pub mod alert;
pub mod app;
pub mod code_action;
pub mod code_lens;
pub mod command;
pub mod completion;
pub mod config;
pub mod db;
pub mod debug;
pub mod doc;
pub mod editor;
pub mod editor_tab;
pub mod file_explorer;
pub mod find;
pub mod focus_text;
pub mod global_search;
pub mod history;
pub mod hover;
pub mod id;
pub mod inline_completion;
pub mod keymap;
pub mod keypress;
pub mod listener;
pub mod lsp;
pub mod main_split;
pub mod markdown;
pub mod palette;
pub mod panel;
pub mod plugin;
pub mod proxy;
pub mod rename;
pub mod settings;
pub mod snippet;
pub mod source_control;
pub mod status;
pub mod terminal;
pub mod text_area;
pub mod text_input;
pub mod title;
pub mod tracing;
pub mod update;
pub mod wave;
pub mod web_link;
pub mod window;
pub mod window_tab;
pub mod workspace;

#[cfg(windows)]
extern crate windows_sys as windows;

// rust-i18n 크레이트의 매크로를 가져옵니다.
#[macro_use]
extern crate rust_i18n;

i18n!("locales");

