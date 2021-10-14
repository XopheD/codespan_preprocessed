
use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

use codespan_preprocessed::PreprocessedFile;

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

    let diagnostic = Diagnostic::note()
        .with_message("this is just an example")
        .with_labels(vec![
            file.primary_label(113..117).with_message("do you see that ?"),
            file.secondary_label(21..26).with_message("it is related to this")
        ]);

    // We now set up the writer and configuration, and then finally render the
    // diagnostic to standard error.

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    term::emit(&mut writer.lock(), &config, &file, &diagnostic);
}