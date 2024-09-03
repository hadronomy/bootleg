use const_format::formatcp;

use crate::build;

pub struct Example {
    pub title: &'static str,
    pub cmd: &'static str,
    pub comments: &'static str,
}

impl Example {
    pub const fn new(title: &'static str, cmd: &'static str, comments: &'static str) -> Self {
        Self {
            title,
            cmd,
            comments,
        }
    }
}

pub static EXAMPLES: &[Example] = &[
    Example::new(
        "Print the text in the clipboard",
        formatcp!("{}", build::PROJECT_NAME),
        "This will print the text in the clipboard",
    ),
    Example::new(
        "Copy the text `Hello, world!` to the clipboard",
        formatcp!("{} \"Hello, world!\"", build::PROJECT_NAME),
        "This will copy the text `Hello, world!` to the clipboard",
    ),
    Example::new(
        "Copy the contents of a file to the clipboard",
        formatcp!("{} < file.txt", build::PROJECT_NAME),
        "This will copy the contents of `file.txt` to the clipboard",
    ),
    Example::new(
        "Copy the output of a command to the clipboard",
        formatcp!(r#"echo "Hello, world!" | {}"#, build::PROJECT_NAME),
        "This will copy the output of `echo \"Hello, world!\"` to the clipboard",
    ),
];
