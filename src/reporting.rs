use std::ops::Range;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::Files;
use crate::PreprocessedFile;

pub trait Report<'a,Source>
    where Source: 'a+AsRef<str>
{
    type Position;
    fn emit(&self, diag: &Diagnostic<<PreprocessedFile<Source> as Files<'a>>::FileId>);
    fn primary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<Source> as Files<'a>>::FileId>;
    fn secondary_label(&'a self, loc: impl Into<Range<Self::Position>>) -> Label<<PreprocessedFile<Source> as Files<'a>>::FileId>;
}


pub trait Reportable<'a,S:'a+AsRef<str>>
{
    fn emit<R:Report<'a,S>>(&self, reporting: &R);
}

impl<'a,S:'a+AsRef<str>> Reportable<'a,S> for Diagnostic<<PreprocessedFile<S> as Files<'a>>::FileId>
{
    fn emit<R:Report<'a,S>>(&self, reporting: &R) {
        reporting.emit(self)
    }
}