use std::fmt::{Debug, Display, Formatter};
use std::ops::Range;
use std::process::ExitCode;
use std::sync::atomic::{AtomicU32, Ordering};
use codespan_reporting::diagnostic;
use codespan_reporting::diagnostic::Severity;
use codespan_reporting::files::Files;
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use crate::codemap::EasyLocation;
use crate::EasyLocated;

pub trait EasyReport
{
    fn emit<E:Display>(&self, diag: impl Into<Diagnostic<E>>);
}


pub struct EasyReporting<'a,L:EasyLocation<'a>>
{
    writer: StandardStream,
    config: Config,
    source: &'a L,
    errors: AtomicU32, // interior mutability
    warnings: AtomicU32 // interior mutability
}

impl <'a,L:EasyLocation<'a>> EasyReport for EasyReporting<'a,L>
{
    fn emit<E: Display>(&self, diag: impl Into<Diagnostic<E>>)
    {
        let diag = diag.into();
        match diag.severity {
            Severity::Bug | Severity::Error => {
                self.errors.fetch_add(1, Ordering::SeqCst);
            }
            Severity::Warning => {
                self.warnings.fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        }
        let diag = diag.to_diagnostic(self.source);
        term::emit(&mut self.writer.lock(), &self.config, self.source, &diag)
            .expect("BUG when reporting errors...");
    }
}

#[derive(Copy, Clone)]
pub enum EasyReportingStatus {
    Faultless,
    Warnings(u32),
    Errors(u32)
}

impl EasyReportingStatus {

    pub fn exit_on_failure(self)
    {
        if let EasyReportingStatus::Errors(n) = self {
            std::process::exit(n as i32)
        }
    }
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

    pub fn check_status(&self) -> EasyReportingStatus
    {
        match self.errors.load(Ordering::SeqCst) {
            0 => match self.warnings.load(Ordering::SeqCst) {
                0 => EasyReportingStatus::Faultless,
                n => EasyReportingStatus::Warnings(n)
            }
            n => EasyReportingStatus::Errors(n)
        }
    }

    /// Displays the current status and returns exit code.
    ///
    /// If this report contains only warnings, then [`ExitCode::SUCCESS`] is returned
    /// but if it contains one or more errors, [`ExitCode::FAILURE`] is returned.
    pub fn emit_status(&self) -> EasyReportingStatus
    {
        let warns = match self.warnings.load(Ordering::SeqCst) {
            0 => { 0 /* no warnings was emmitted, good ! */ },
            1 => {
                term::emit(&mut self.writer.lock(), &self.config, self.source,
                           &diagnostic::Diagnostic::warning().with_message("1 warning emitted"))
                    .expect("BUG when reporting errors...");
                1
            },
            n => {
                term::emit(&mut self.writer.lock(), &self.config, self.source,
                           &diagnostic::Diagnostic::warning().with_message(format!("{} warnings emitted", n)))
                    .expect("BUG when reporting errors...");
                n
            }
        };
        match self.errors.load(Ordering::SeqCst) {
            0 => {
                /* no errors was emmitted, good ! */
                if warns == 0 { EasyReportingStatus::Faultless } else { EasyReportingStatus::Warnings(warns)}
            },
            1 => {
                term::emit(&mut self.writer.lock(), &self.config, self.source,
                           &diagnostic::Diagnostic::error().with_message("1 error emitted"))
                    .expect("BUG when reporting errors...");
                EasyReportingStatus::Errors(1)
            },
            n => {
                term::emit(&mut self.writer.lock(), &self.config, self.source,
                          &diagnostic::Diagnostic::error().with_message(format!("{} errors emitted", n)))
                    .expect("BUG when reporting errors...");
                EasyReportingStatus::Errors(n)
            }
        }
    }
}

impl<'a, R:EasyReport> EasyReport for &'a R
{
    #[inline]
    fn emit<E: Display>(&self, diag: impl Into<Diagnostic<E>>) {
        EasyReport::emit(*self, diag)
    }
}


#[derive(Clone)]
pub struct Diagnostic<E:Display> {
    code: E,
    severity: Severity,
    message: String,
    labels: Vec<(diagnostic::LabelStyle,Range<usize>,String)>,
    notes: Vec<String>,
}

impl Diagnostic<&'static str>
{
    #[inline]
    pub fn bug() -> Self { Self::new("", Severity::Bug) }
    #[inline]
    pub fn error() -> Self { Self::new("", Severity::Error) }
    #[inline]
    pub fn warning() -> Self { Self::new("", Severity::Warning) }
    #[inline]
    pub fn note() -> Self { Self::new("", Severity::Note) }
    #[inline]
    pub fn help() -> Self { Self::new("", Severity::Help) }
}

impl<E:Display> Diagnostic<E>
{
    #[inline]
    pub fn new(code: E, severity: Severity) -> Self
    {
        Self { code, severity, message: String::new(), labels: vec![], notes: vec![] }
    }

    #[inline]
    pub fn code(&self) -> &E { &self.code }

    #[inline]
    pub fn with_code<EE:Display>(self, code: EE) -> Diagnostic<EE>
    {
        Diagnostic {
            code,
            severity: self.severity,
            message: self.message,
            labels: self.labels,
            notes: self.notes
        }
    }

    #[inline]
    pub fn with_message(mut self, msg: impl Into<String>) -> Self
    {
        self.message = msg.into();
        self
    }

    #[inline]
    pub fn with_note(mut self, note: impl Into<String>) -> Self
    {
        self.notes.push(note.into());
        self
    }

    #[inline]
    pub fn with_labeled_note(mut self, label: impl AsRef<str>, note: impl Into<String>) -> Self
    {
        // add a note with a bold label
        self.notes.push(format!("\x1B[1m{}\x1B[0m: {}", label.as_ref(), note.into()));
        self
    }


    #[inline]
    pub fn with_primary_label(mut self, range: impl Into<Range<usize>>, msg: impl Into<String>) -> Self
    {
        let range = range.into();
        assert![ !range.is_empty(), "invalid (empty) location" ];
        self.labels.push((diagnostic::LabelStyle::Primary, range, msg.into()));
        self
    }

    #[inline]
    pub fn with_primary_located_label<L:ToString>(self, label: EasyLocated<L>) -> Self
    {
        self.with_primary_label(label.location().clone(), label.to_string())
    }

    #[inline]
    pub fn with_secondary_label(mut self, range: impl Into<Range<usize>>, msg: impl Into<String>) -> Self
    {
        let range = range.into();
        assert![ !range.is_empty(), "invalid (empty) location" ];
        self.labels.push((diagnostic::LabelStyle::Secondary, range, msg.into()));
        self
    }

    #[inline]
    pub fn with_secondary_located_label<L:ToString>(self, label: EasyLocated<L>) -> Self
    {
        self.with_secondary_label(label.location().clone(), label.to_string())
    }

    pub fn to_diagnostic<'a,L:EasyLocation<'a>>(self, src: &'a L) -> diagnostic::Diagnostic<<L as Files<'a>>::FileId>
    {
        diagnostic::Diagnostic::new(self.severity)
            .with_code(self.code.to_string())
            .with_message(self.message)
            .with_notes(self.notes)
            .with_labels(self.labels
                .into_iter()
                .map(|(style, range, message)| {
                    (diagnostic::Label::new(style, src.file_id(range.start), range), message)
                })
                .map(|(diag, message)| {
                    if message.is_empty() { diag } else { diag.with_message(message) }
                })
                .collect()
            )
    }

    #[inline]
    pub fn report<R:EasyReport>(self, report: &R) { report.emit(self) }
}

impl<E:Display> Debug for Diagnostic<E>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {}", self.code, self.message)?;
        self.notes.iter().try_for_each(|note| writeln!(f,"   {}", note))
    }
}