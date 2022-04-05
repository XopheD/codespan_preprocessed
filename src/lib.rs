//! # Codemap for Preprocessed File
//!
//! This is an extension for the very useful crate `codespan_reporting`
//! to deal with preprocessed file through the well-known`m4` or `cpp`.
//!
//! Using such a preprocessor allows, among a lost of things, the
//! inclusion of many files which are identified in the bytes sequence
//! with preprecessor directive as:
//!```text
//! #line 42 "/my/preprocessed/file"
//!```
//! This directive breaks the location of the source and so
//! should be correctly processed to make correct location
//! for error reporting.
//!
//! This is the purpose of this crate: taking a preprocessor
//! output and managing the different underlying locations
//! inside it.
//!
//! # Example ignore (only-for-syntax-highlight)
//!
//! ```
//! use codespan_reporting::diagnostic::Diagnostic;
//! use codespan_reporting::term;
//! use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
//! use codespan_preprocessed::PreprocessedFile;
//!
//! fn main()
//! {
//!     let file = PreprocessedFile::new(
//!         unindent::unindent(
//!             r#"
//!                 #line 1 "top_file"
//!                 a first statement;
//!                 another one
//!
//!                 #line 1 "included_file"
//!                 continue...
//!
//!                 #line 5
//!                 another line
//!                 the last one
//!             "#,
//!         ),
//!     );
//!
//!     let diagnostic = Diagnostic::note()
//!         .with_message("this is just an example")
//!         .with_labels(vec![
//!             file.primary_label(113..117).with_message("do you see that ?"),
//!             file.secondary_label(21..26).with_message("is it related to this ?")
//!         ]);
//!
//!     // We now set up the writer and configuration, and then finally render the
//!     // diagnostic to standard error.
//!     // (see `codespan_reporting` documention for more details)
//!
//!     let writer = StandardStream::stderr(ColorChoice::Always);
//!     let config = codespan_reporting::term::Config::default();
//!
//!     term::emit(&mut writer.lock(), &config, &file, &diagnostic);
//! }
//! ```
//! The previous code will produce:
//! ```text
//! note: this is just an example
//!   ┌─ included_file:6:5
//!   │
//! 6 │ the last one
//!   │     ^^^^ do you see that ?
//!   │
//!   ┌─ top_file:1:3
//!   │
//! 1 │ a first statement;
//!   │   ----- is it related to this ?
//! ```
mod codemap;
pub mod reporting;

pub use codemap::PreprocessedFile;
pub use codemap::EasyLocation;
