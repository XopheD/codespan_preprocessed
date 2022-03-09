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
    type Position;

    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<Source> as Files<'a>>::FileId>) -> Result<(), Error>;
    fn primary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<Source> as Files<'a>>::FileId>;
    fn secondary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<Source> as Files<'a>>::FileId>;
}


pub trait Reportable<'a,S:'a+AsRef<str>>
{
    fn emit<R:Report<'a,S>>(&self, reporting: &R) -> Result<(), Error>;
}

impl<'a,S:'a+AsRef<str>> Reportable<'a,S> for Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>
{
    fn emit<R:Report<'a,S>>(&self, reporting: &R) -> Result<(), Error> { reporting.emit(self) }
}

pub trait Locator {
    type Position;
    fn locate(&self, pos: Self::Position) -> usize;
}


pub struct RelocatableReport<'a,S,P,L>
    where 
        S:'a+AsRef<str>,
        L: Fn(P)->usize
{
    writer: StandardStream,
    config: Config,
    source: &'a PreprocessedFile<S>,
    locator: L,
    position: PhantomData<P>
}

impl<'a,S,P,L> Locator for RelocatableReport<'a,S,P,L>
    where
        S:'a+AsRef<str>,
        L: Fn(P)->usize
{
    type Position = P;

    #[inline]
    fn locate(&self, pos: Self::Position) -> usize {
        (&self.locator)(pos)
    }
}


impl<'a,S,P,L> RelocatableReport<'a,S,P,L>
    where
        S:'a+AsRef<str>,
        L: Fn(P)->usize
{
    pub fn new(source: &'a PreprocessedFile<S>, locator: L) -> Self
    {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        Self { writer, config, source, locator, position: PhantomData::default() }
    }

}


impl<'a,S,P,L> Report<'a,S> for RelocatableReport<'a,S,P,L>
    where
        S: 'a+AsRef<str>,
        L: Fn(P)->usize
{
    type Position = P;

    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>) -> Result<(), Error>
    {
        term::emit(&mut self.writer.lock(), &self.config, self.source, &diag)
    }

    fn primary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<S> as Files<'a>>::FileId>
    {
        let loc = loc.into();
        let start = self.locate(loc.start);
        let end = self.locate(loc.end);
        self.source.primary_label(start..end)
    }

    fn secondary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<S> as Files<'a>>::FileId>
    {
        let loc = loc.into();
        let start = self.locate(loc.start);
        let end = self.locate(loc.end);
        self.source.secondary_label(start..end)
    }
}
