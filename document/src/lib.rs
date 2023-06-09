//! A crate to represent open files ('documents').

pub mod buffer;
mod document;
mod scapegoat_tree;
mod ranked_tree;

pub use crate::document::Document;
