//! A crate to represent open files ('documents').

pub mod buffer;
mod document;
mod ranked_tree;
mod rb_tree;
mod scapegoat_tree;

pub use crate::document::Document;
