use std::ops::{Range, Index};
use std::iter;
use codespan_reporting::files;
use codespan_reporting::files::Files;
use codespan_reporting::diagnostic::Label;


#[derive(Clone, Debug)]
struct LineDirective {
    line_index: usize,
    byte_index: usize,
    offset: usize,
    filename: Option<Range<usize>>
}

/// Slice of the input file.
///
/// The input file is sliced into different
/// parts corresponding to new location directive.
/// This slicing is so used as file identification for
/// `codespan_reporting`.
#[derive(Clone, Debug, PartialEq)]
pub struct FileSlice {
    name: Range<usize>,
    bytes: Range<usize>,
    lines: Range<usize>,
    offset: usize
}

/// The codemap of a preprocessed file.
pub struct PreprocessedFile<Source> {
    ids: Vec<FileSlice>,
    lines: Vec<Range<usize>>,
    contents: Source
}


impl<'a, Source> Files<'a> for PreprocessedFile<Source>
    where
        Source: 'a + AsRef<str>
{
    type FileId = &'a FileSlice;
    type Name = &'a str;
    type Source = &'a str;

    fn name(&'a self, id: Self::FileId) -> Result<Self::Name, files::Error> {
        Ok(&self.contents.as_ref().index(id.name.clone()))
    }

    fn source(&'a self, _: Self::FileId) -> Result<Self::Source, files::Error> {
        Ok(self.contents.as_ref())
    }

    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, files::Error>
    {
        if id.bytes.contains(&byte_index){
            Ok(self.lines.iter()
                .enumerate()
                // byte_index could reach b.end (which is the EOL byte)
                .find(|(_, b)| byte_index <= b.end && byte_index >= b.start)
                .unwrap().0 - id.offset)
        } else {
            Err(files::Error::IndexTooLarge { given: byte_index, max: id.bytes.end })
        }
    }

    fn line_range(&'a self, id: Self::FileId, line_index: usize) -> Result<Range<usize>, files::Error>
    {
        self.lines.get(line_index+id.offset).cloned()
            .ok_or(files::Error::LineTooLarge { given: line_index, max: self.lines.len() })
    }
}

impl<Source> PreprocessedFile<Source>
    where
        Source: AsRef<str>
{
    pub fn new(contents: Source) -> Self
    {

        let lines = contents
            .as_ref()
            .match_indices('\n')
            .map(|(b,_)| b )
            .collect::<Vec<_>>();

        let lines =
            iter::once(0)
                .chain(lines.iter().map(|e| *e+1))
                .zip(lines.iter())
                .map(|(s,e)| s .. *e)
                .collect::<Vec<_>>();

        let directives =
            lines.iter()
                .enumerate()
                .filter(|(_, r)| contents.as_ref()[r.start..r.end].starts_with("#line"))
                .map(|(l, r)| {
                    let str = &contents.as_ref()[r.start..r.end];
                    if let Some(sep) = str[6..].find(" ") {
                        let sep = sep + 6;
                        LineDirective {
                            line_index: l,
                            byte_index: r.start,
                            offset: l+2-str[6..sep].parse::<usize>().unwrap(),
                            filename: Some(r.start+sep+2..r.start+str.len()-1)
                        }
                    } else {
                        LineDirective {
                            line_index: l,
                            byte_index: r.start,
                            offset: l+2-str[6..].parse::<usize>().unwrap(),
                            filename: None
                        }
                    }
                })
                .collect::<Vec<_>>();

        let mut current = 0..0;
        let mut files = Vec::with_capacity(directives.len()+2);

        if let Some(first) = directives.first() {
            if first.line_index > 0 {
                files.push(FileSlice {
                    name: current.clone(),
                    bytes: 0..first.byte_index,
                    lines: 0..first.line_index,
                    offset: 0
                });
            }
            files.extend(directives.iter()
                .zip(directives.iter().skip(1))
                .map(|(start, end)| {
                    if let Some(filename) = start.filename.clone() {
                        current = filename;
                    }
                    FileSlice {
                        name: current.clone(),
                        bytes: lines[start.line_index+1].start .. end.byte_index ,
                        lines: start.line_index+1 .. end.line_index,
                        offset: start.offset
                    }
                }));

            let last_directive = directives.last().unwrap();
            files.push(FileSlice {
                name: last_directive.filename.clone().unwrap_or(current),
                bytes: lines[last_directive.line_index+1].start .. contents.as_ref().len(),
                lines: last_directive.line_index+1 .. lines.len(),
                offset: last_directive.offset
            });
        } else {
            files.push(FileSlice {
                name: current,
                bytes: 0..contents.as_ref().len(),
                lines: 0..lines.len(),
                offset: 0
            })
        }

        PreprocessedFile {
            ids: files,
            lines,
            contents
        }
    }

    pub fn primary_label(&self, range: impl Into<Range<usize>>) -> Label<<Self as Files>::FileId>
    {
        let range = range.into();
        Label::primary(self.file_id(range.start).unwrap(), range)
    }

    pub fn secondary_label(&self, range: impl Into<Range<usize>>) -> Label<<Self as Files>::FileId>
    {
        let range = range.into();
        Label::secondary(self.file_id(range.start).unwrap(), range)
    }

    pub fn file_id(&self, byte_index: usize) -> Result<&FileSlice,files::Error> {
        self.ids.iter()
            .find(|f| f.bytes.contains(&byte_index))
            .ok_or(files::Error::FileMissing)
    }
}