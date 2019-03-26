use std::slice::Iter;
use std::str::Chars;

use super::Opt;

/// Parses options from a series of `&str`-like values.
pub struct Parser<'a, S> {
    args: Iter<'a, S>,
    cur: Option<Chars<'a>>,
    style: ParsingStyle,
    terminated: bool,
}

impl<'a, S: 'a + AsRef<str>> Parser<'a, S> {
    /// Returns a new parser for the given series of arguments.
    ///
    /// The given slice should **not** contain the program name as its first
    /// element.
    pub fn new(args: &'a [S], style: ParsingStyle) -> Parser<'a, S> {
        Parser {
            args: args.iter(),
            cur: None,
            style,
            terminated: false,
        }
    }

    /// Returns the next option or `None` if no options remain.
    ///
    /// If the previous option had an explicit argument, e.g. `--option=argument`,
    /// which was not consumed by a call to `next_arg()`, an error will be
    /// returned indicating that the argument was ignored.
    pub fn next_opt(&mut self) -> Option<Opt<'a>> {
        if let Some(mut cur) = self.cur.take() {
            if let Some(opt) = cur.next() {
                self.cur = Some(cur);
                return Some(Opt::Short(opt));
            }
        }

        if self.terminated {
            return self.args.next().map(|s| Opt::Free(s.as_ref()));
        }

        match self.args.next().map(|s| s.as_ref()) {
            Some(arg @ "-") => {
                if self.style == ParsingStyle::StopAtFirstFree {
                    self.terminated = true;
                }
                Some(Opt::Free(arg))
            }
            Some("--") => {
                self.terminated = true;
                self.args.next().map(|s| Opt::Free(s.as_ref()))
            }
            Some(long) if long.starts_with("--") => match long.find('=') {
                Some(pos) => Some(Opt::LongWithArg(&long[2..pos], &long[pos + 1..])),
                None => Some(Opt::Long(&long[2..])),
            },
            Some(short) if short.starts_with('-') => {
                let mut chars = short[1..].chars();

                let res = chars.next().map(Opt::Short);

                self.cur = Some(chars);
                res
            }
            Some(free) => {
                if self.style == ParsingStyle::StopAtFirstFree {
                    self.terminated = true;
                }
                Some(Opt::Free(free))
            }
            None => None,
        }
    }

    /// Returns the next argument to an option or `None` if none remain.
    pub fn next_arg(&mut self) -> Option<&'a str> {
        if let Some(cur) = self.cur.take() {
            let arg = cur.as_str();

            if !arg.is_empty() {
                return Some(arg);
            }
        }

        self.args.next().map(|s| s.as_ref())
    }
}

impl<'a, S: 'a> Clone for Parser<'a, S> {
    fn clone(&self) -> Parser<'a, S> {
        Parser {
            args: self.args.clone(),
            cur: self.cur.clone(),
            style: self.style,
            terminated: self.terminated,
        }
    }
}

/// Controls behavior of free arguments in `Parser`
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ParsingStyle {
    /// Process all option arguments that appear
    AllOptions,
    /// After the first "free" argument is encountered,
    /// all remaining arguments will be considered "free" arguments.
    StopAtFirstFree,
}

impl Default for ParsingStyle {
    /// Returns the default parsing style, `AllOptions`.
    fn default() -> ParsingStyle {
        ParsingStyle::AllOptions
    }
}
