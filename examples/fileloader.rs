
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

use codespan_preprocessed::PreprocessedFile;
use codespan_preprocessed::reporting::*;

fn main() {

    let file = PreprocessedFile::open("examples/readme.rs").unwrap();

    let diagnostic = Diagnostic::note()
        .with_message("this is just an example")
        .with_primary_label(131..135, "do you see that ?")
        .with_secondary_label(39..47, "is it related to this ?");

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let writer = StandardStream::stderr(ColorChoice::Always);

    term::emit(&mut writer.lock(), &Default::default(), &file, &diagnostic.to_diagnostic(&file)).expect("can’t write diagnostic");
}