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
//! # Example
//!
//!```
//! use codespan_preprocessed::reporting::Diagnostic;
//! use codespan_preprocessed::PreprocessedFile;
//!
//! fn main()
//! {
//!    let contents = PreprocessedFile::new(
//!        unindent::unindent(
//!            r#"
//!                #line 1 "top_file"
//!                a first statement;
//!                another one
//!
//!                #line 1 "included_file"
//!                continue...
//!
//!                #line 5
//!                another line
//!                the last one
//!            "#,
//!        ),
//!    );
//!
//!    // build a diagnostic for reporting
//!    let diagnostic = Diagnostic::note()
//!        .with_message("this is just an example")
//!        .with_primary_label(113..117, "do you see that ?")
//!        .with_secondary_label(21..26, "is it related to this ?");
//! # }
//!```
//!
//! ### Reporting a diagnostic
//! The first way to make reporting is based of `codespan_reporting` (see documentation for more details).
//!```
//! use codespan_reporting::term;
//! use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
//! # use codespan_preprocessed::reporting::*;
//! # use codespan_preprocessed::PreprocessedFile;
//!
//! # fn main()
//! # {
//! #   let contents = PreprocessedFile::new("");
//! #   let diagnostic = Diagnostic::note();
//! let writer = StandardStream::stderr(ColorChoice::Always);
//! let config = codespan_reporting::term::Config::default();
//! term::emit(&mut writer.lock(), &config, &contents, &diagnostic.to_diagnostic(&contents));
//! # }
//!```
//! ### Easy reporting (alternative)
//! This crate provides an easier way to report diagnostic based on
//! preprocessed file.
//! ```
//! # use codespan_reporting::term;
//! # use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
//! use codespan_preprocessed::reporting::{Diagnostic,EasyReport,EasyReporting};
//! # use codespan_preprocessed::PreprocessedFile;
//!
//! # fn main()
//! # {
//! #   let contents = PreprocessedFile::new("");
//! #   let diagnostic = Diagnostic::note();
//! let report = EasyReporting::new(&contents);
//! report.emit(diagnostic);
//! # }
//! ```
//! ### Output
//! The both previous codes will produce:
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
mod easyloc;
pub mod reporting;

pub use codemap::PreprocessedFile;
pub use codemap::EasyLocation;
pub use easyloc::EasyLocated;