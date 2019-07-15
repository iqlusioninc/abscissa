//! Template names (i.e. paths)

use std::fmt;

/// Template names
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Name(String);

impl<'a> From<&'a str> for Name {
    fn from(name: &'a str) -> Name {
        assert!(name.ends_with(".hbs"), "template files must end with .hbs");
        Name(name.to_owned())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
