use std::marker::PhantomData;
use std::ops::Range;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::{Error, Files};
use codespan_reporting::term;
use codespan_reporting::term::Config;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use crate::PreprocessedFile;

pub trait Report<'a,Source>
    where Source: 'a+AsRef<str>
{
    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<Source> as Files<'a>>::FileId>) -> Result<(), Error>;
}

pub trait Reportable<'a,S:'a+AsRef<str>>
{
    fn emit<R: Report<'a,S>>(&self, reporting: &R) -> Result<(), Error>;
}

impl<'a,S:'a+AsRef<str>> Reportable<'a,S> for Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>
{
    fn emit<R: Report<'a,S>>(&self, reporting: &R) -> Result<(), Error> { reporting.emit(self) }
}


pub struct PreprocessedReport<'a,S>
    where
        S:'a+AsRef<str>
{
    writer: StandardStream,
    config: Config,
    source: &'a PreprocessedFile<S>
}


impl<'a,S> PreprocessedReport<'a,S>
    where
        S:'a+AsRef<str>
{
    pub fn new(source: &'a PreprocessedFile<S>) -> Self
    {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        Self { writer, config, source }
    }

    pub fn with_config(source: &'a PreprocessedFile<S>, config: Config) -> Self
    {
        let writer = StandardStream::stderr(ColorChoice::Always);
        Self { writer, config, source }
    }

    pub fn source(&self) -> &'a PreprocessedFile<S> { self.source }
}

impl<'a,S> Report<'a,S> for PreprocessedReport<'a,S>
    where
        S: 'a+AsRef<str>
{
    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>) -> Result<(), Error>
    {
        term::emit(&mut self.writer.lock(), &self.config, self.source, &diag)
    }
}

