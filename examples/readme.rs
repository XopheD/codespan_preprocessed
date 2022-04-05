
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

use codespan_preprocessed::PreprocessedFile;
use codespan_preprocessed::reporting::Diagnostic;

fn main() {

    let file = PreprocessedFile::new(
        unindent::unindent(
            r#"
                    #line 1 "top_file"
                    a first statement;
                    another one

                    #line 1 "included_file"
                    continue...

                    #line 5
                    another line
                    the last one
            "#,
        ),
    );

    let diagnostic = Diagnostic::note("")
        .with_message("this is just an example")
        .with_primary_label(113..117, "do you see that ?")
        .with_secondary_label(21..26, "is it related to this ?");

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let writer = StandardStream::stderr(ColorChoice::Always);

    term::emit(&mut writer.lock(), &Default::default(), &file, &diagnostic.to_diagnostic(&file)).expect("can’t write diagnostic");
}