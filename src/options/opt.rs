use std::string::ToString;

/// Represents an option parsed from a `Parser`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Opt<'a> {
    /// Short option, e.g. `-o`
    Short(char),
    /// Long option, e.g. `--option`
    Long(&'a str),
    /// Long option with argument, e.g. `--option=value`
    LongWithArg(&'a str, &'a str),
    /// Free argument
    Free(&'a str),
}

impl<'a> ToString for Opt<'a> {
    fn to_string(&self) -> String {
        match *self {
            Opt::Short(ch) => format!("-{}", ch),
            Opt::Long(s) => format!("--{}", s),
            Opt::LongWithArg(opt, _) => format!("--{}", opt),
            Opt::Free(_) => "free".to_owned(),
        }
    }
}
