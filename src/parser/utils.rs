pub enum ParseError<'a> {
    MissingKey,
    MissingKeys,
    MissingValue,
    InvalidCommandOptions(&'a str),
    InvalidCommandOptionValue(&'a str),
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingKey => write!(f, "No key provided"),
            ParseError::MissingKeys => write!(f, "No keys provided"),
            ParseError::MissingValue => write!(f, "No value provided"),
            ParseError::InvalidCommandOptions(msg) => write!(f, "Invalid command options: {msg}"),
            ParseError::InvalidCommandOptionValue(msg) => {
                write!(f, "Invalid command option value: {msg}")
            }
        }
    }
}
