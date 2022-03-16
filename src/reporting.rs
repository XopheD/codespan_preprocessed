use std::ops::Range;
use std::sync::atomic::{AtomicU32, Ordering};
use codespan_reporting::diagnostic::{Diagnostic, Label, LabelStyle, Severity};
use codespan_reporting::files::Files;
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use crate::PreprocessedFile;

pub trait Report<'a, S>
    where S: 'a+AsRef<str>
{
    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>);

    fn source(&self) -> &'a PreprocessedFile<S>;

    fn label(&self, style: LabelStyle, range: impl Into<Range<usize>>) -> Label<<PreprocessedFile<S> as Files<'a>>::FileId>
    {
        self.source().label(style, range)
    }

    fn primary_label(&self, range: impl Into<Range<usize>>) -> Label<<PreprocessedFile<S> as Files<'a>>::FileId>
    {
        self.source().primary_label(range)
    }

    fn secondary_label(&self, range: impl Into<Range<usize>>) -> Label<<PreprocessedFile<S> as Files<'a>>::FileId>
    {
        self.source().secondary_label(range)
    }
}

pub trait Reportable<'a,S:'a+AsRef<str>,R:Report<'a,S>>
{
    fn emit(self, reporting: &R);
}

impl<'a,S:'a+AsRef<str>,R:Report<'a,S>> Reportable<'a,S,R> for Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>
{
    fn emit(self, reporting: &R) { reporting.emit(&self) }
}


pub struct PreprocessedReport<'a,S>
    where
        S:'a+AsRef<str>
{
    writer: StandardStream,
    config: Config,
    source: &'a PreprocessedFile<S>,
    errors: AtomicU32, // interior mutability
    warnings: AtomicU32 // interior mutability
}


impl<'a,S> PreprocessedReport<'a,S>
    where
        S:'a+AsRef<str>
{
    pub fn new(source: &'a PreprocessedFile<S>) -> Self
    {
        Self::with_config(source,codespan_reporting::term::Config::default())
    }

    pub fn with_config(source: &'a PreprocessedFile<S>, config: Config) -> Self
    {
        let writer = StandardStream::stderr(ColorChoice::Always);
        Self { writer, config, source, errors: AtomicU32::default(), warnings: AtomicU32::default() }
    }

    pub fn check(&self) -> bool
    {
        match self.warnings.load(Ordering::SeqCst) {
            0 => { /* no warnings was emmitted, good ! */ },
            1 => {
                Diagnostic::warning().with_message("1 warning emitted").emit(self)
            },
            n => {
                Diagnostic::warning()
                    .with_message(format!("{} warnings emitted", n))
                    .emit(self);
            }
        }
        match self.errors.load(Ordering::SeqCst) {
            0 => {
                /* no errors was emmitted, good ! */
                true
            },
            1 => {
                Diagnostic::error().with_message("1 error emitted").emit(self);
                false
            },
            n => {
                Diagnostic::error()
                    .with_message(format!("{} errors emitted", n))
                    .emit(self);
                false
            }
        }
    }
}

impl<'a,S> Report<'a,S> for PreprocessedReport<'a,S>
    where
        S: 'a+AsRef<str>
{
    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>)
    {
        match diag.severity {
            Severity::Bug | Severity::Error => {
                self.errors.fetch_add(1, Ordering::SeqCst);
            }
            Severity::Warning => {
                self.warnings.fetch_add(1, Ordering::SeqCst);
            }
            _ => { }
        }
        term::emit(&mut self.writer.lock(), &self.config, self.source, &diag)
            .expect("BUG when reporting errors...");
    }

    fn source(&self) -> &'a PreprocessedFile<S> {
        self.source
    }
}

