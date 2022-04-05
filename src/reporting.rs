use std::fmt::Display;
use std::ops::Range;
use std::sync::atomic::{AtomicU32, Ordering};
use codespan_reporting::diagnostic;
use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::Files;
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use crate::codemap::EasyLocation;
use crate::PreprocessedFile;


pub struct EasyReporting<'a,L:EasyLocation<'a>>
{
    writer: StandardStream,
    config: Config,
    source: &'a L,
    errors: AtomicU32, // interior mutability
    warnings: AtomicU32 // interior mutability
}


impl<'a,L:EasyLocation<'a>> EasyReporting<'a,L>
{
    pub fn new(source: &'a L) -> Self
    {
        Self::with_config(source,codespan_reporting::term::Config::default())
    }

    pub fn with_config(source: &'a L, config: Config) -> Self
    {
        let writer = StandardStream::stderr(ColorChoice::Always);
        Self { writer, config, source, errors: AtomicU32::default(), warnings: AtomicU32::default() }
    }


    fn emit(&self, diag: Diagnostic)
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
        let diag = diag.to_diagnostic(self.source);
        term::emit(&mut self.writer.lock(), &self.config, self.source, &diag)
            .expect("BUG when reporting errors...");
    }

    fn status(&self) -> Result<(),()>
    {
        match self.warnings.load(Ordering::SeqCst) {
            0 => { /* no warnings was emmitted, good ! */ },
            1 => {
                Diagnostic::warning().with_message("1 warning emitted").report(self);
                // but this warning should not be counted
                self.warnings.fetch_sub(1, Ordering::SeqCst);
            },
            n => {
                Diagnostic::warning()
                    .with_message(format!("{} warnings emitted", n))
                    .report(self);
                // but this warning should not be counted
                self.warnings.fetch_sub(1, Ordering::SeqCst);
            }
        }
        match self.errors.load(Ordering::SeqCst) {
            0 => {
                /* no errors was emmitted, good ! */
                Ok(())
            },
            1 => {
                Diagnostic::error().with_message("1 error emitted").report(self);
                // but this error should not be counted
                self.errors.fetch_sub(1, Ordering::SeqCst);
                Err(())
            },
            n => {
                Diagnostic::error()
                    .with_message(format!("{} errors emitted", n))
                    .report(self);
                // but this error should not be counted
                self.errors.fetch_sub(1, Ordering::SeqCst);
                Err(())
            }
        }
    }
}


#[derive(Clone,Debug)]
pub struct Diagnostic {
    severity: Severity,
    message: String,
    labels: Vec<(diagnostic::LabelStyle,Range<usize>,String)>,
    notes: Vec<String>,
}

impl Diagnostic
{
    pub fn new(severity: Severity) -> Self {
        Self { severity, message: String::new(), labels: vec![], notes: vec![] }
    }

    #[inline] pub fn bug() -> Self { Self::new(Severity::Bug)}
    #[inline] pub fn error() -> Self { Self::new(Severity::Error)}
    #[inline] pub fn warning() -> Self { Self::new(Severity::Warning)}
    #[inline] pub fn note() -> Self { Self::new(Severity::Note)}
    #[inline] pub fn help() -> Self { Self::new(Severity::Help)}

    pub fn with_message(mut self, msg: impl Into<String>) -> Self
    {
        self.message = msg.into();
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self
    {
        self.notes.push(note.into());
        self
    }

    pub fn with_primary_label(mut self, range: impl Into<Range<usize>>, msg: impl Into<String>) -> Self
    {
        self.labels.push((diagnostic::LabelStyle::Primary, range.into(), msg.into()));
        self
    }

    pub fn with_secondary_label(mut self, range: impl Into<Range<usize>>, msg: impl Into<String>) -> Self
    {
        self.labels.push((diagnostic::LabelStyle::Secondary, range.into(), msg.into()));
        self
    }

    pub fn to_diagnostic<'a,L:EasyLocation<'a>>(mut self, src: &'a L) -> diagnostic::Diagnostic<<L as Files<'a>>::FileId>
    {
        diagnostic::Diagnostic::new(self.severity)
            .with_message(self.message)
            .with_notes(self.notes)
            .with_labels(self.labels.into_iter()
                .map(|(style, range, message)| {
                    if message.is_empty() {
                        diagnostic::Label::new(style, src.file_id(range.start), range)
                    } else {
                        diagnostic::Label::new(style, src.file_id(range.start), range).with_message(message)
                    }
                }).collect())
    }

    #[inline]
    pub fn report<'a,L:EasyLocation<'a>>(self, report: &EasyReporting<'a,L>) { report.emit(self) }
}
