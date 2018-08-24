use syn::{Attribute, Lit, Meta, NestedMeta};

use super::{default::DefaultOpts, is_outer, path_eq};
use options::{lit_str, parse::ParseFn, tokens_str};

#[derive(Default)]
pub struct AttrOpts {
    pub long: Option<String>,
    pub short: Option<char>,
    pub free: bool,
    pub count: bool,
    pub help_flag: bool,
    pub no_help_flag: bool,
    pub no_short: bool,
    pub no_long: bool,
    pub required: bool,
    pub not_required: bool,
    pub help: Option<String>,
    pub meta: Option<String>,
    pub parse: Option<ParseFn>,
    pub default: Option<String>,
    pub command: bool,
}

impl AttrOpts {
    #[allow(unknown_lints, cyclomatic_complexity)]
    pub fn check(&self) {
        if self.command {
            if self.free {
                panic!("`command` and `free` are mutually exclusive");
            }
            if self.default.is_some() {
                panic!("`command` and `default` are mutually exclusive");
            }
            if self.long.is_some() {
                panic!("`command` and `long` are mutually exclusive");
            }
            if self.short.is_some() {
                panic!("`command` and `short` are mutually exclusive");
            }
            if self.count {
                panic!("`command` and `count` are mutually exclusive");
            }
            if self.help_flag {
                panic!("`command` and `help_flag` are mutually exclusive");
            }
            if self.no_help_flag {
                panic!("`command` and `no_help_flag` are mutually exclusive");
            }
            if self.no_short {
                panic!("`command` and `no_short` are mutually exclusive");
            }
            if self.no_long {
                panic!("`command` and `no_long` are mutually exclusive");
            }
            if self.help.is_some() {
                panic!("`command` and `help` are mutually exclusive");
            }
            if self.meta.is_some() {
                panic!("`command` and `meta` are mutually exclusive");
            }
        }

        if self.free {
            if self.default.is_some() {
                panic!("`free` and `default` are mutually exclusive");
            }
            if self.long.is_some() {
                panic!("`free` and `long` are mutually exclusive");
            }
            if self.short.is_some() {
                panic!("`free` and `short` are mutually exclusive");
            }
            if self.count {
                panic!("`free` and `count` are mutually exclusive");
            }
            if self.help_flag {
                panic!("`free` and `help_flag` are mutually exclusive");
            }
            if self.no_help_flag {
                panic!("`free` and `no_help_flag` are mutually exclusive");
            }
            if self.no_short {
                panic!("`free` and `no_short` are mutually exclusive");
            }
            if self.no_long {
                panic!("`free` and `no_long` are mutually exclusive");
            }
            if self.meta.is_some() {
                panic!("`free` and `meta` are mutually exclusive");
            }
        }

        if self.help_flag && self.no_help_flag {
            panic!("`help_flag` and `no_help_flag` are mutually exclusive");
        }

        if self.no_short && self.short.is_some() {
            panic!("`no_short` and `short` are mutually exclusive");
        }

        if self.no_long && self.long.is_some() {
            panic!("`no_long` and `long` are mutually exclusive");
        }

        if self.required && self.not_required {
            panic!("`required` and `not_required` are mutually exclusive");
        }

        if self.parse.is_some() && self.count {
            panic!("`count` and `parse` are mutually exclusive");
        }
    }

    pub fn parse(attrs: &[Attribute]) -> AttrOpts {
        let mut opts = AttrOpts::default();

        for attr in attrs {
            if is_outer(attr.style) && path_eq(&attr.path, "options") {
                let meta = attr
                    .interpret_meta()
                    .unwrap_or_else(|| panic!("invalid attribute: {}", tokens_str(attr)));

                match meta {
                    Meta::Word(_) => panic!("#[options] is not a valid attribute"),
                    Meta::NameValue(..) => panic!("#[options = ...] is not a valid attribute"),
                    Meta::List(ref items) => {
                        for item in &items.nested {
                            opts.parse_item(item);
                        }
                    }
                }
            }
        }

        opts.check();

        opts
    }

    pub fn parse_item(&mut self, item: &NestedMeta) {
        match *item {
            NestedMeta::Literal(_) => panic!("unexpected meta item `{}`", tokens_str(item)),
            NestedMeta::Meta(ref item) => match *item {
                Meta::Word(ref w) => match &w.to_string()[..] {
                    "free" => self.free = true,
                    "command" => self.command = true,
                    "count" => self.count = true,
                    "help_flag" => self.help_flag = true,
                    "no_help_flag" => self.no_help_flag = true,
                    "no_short" => self.no_short = true,
                    "no_long" => self.no_long = true,
                    "required" => self.required = true,
                    "not_required" => self.not_required = true,
                    _ => panic!("unexpected meta item `{}`", tokens_str(item)),
                },
                Meta::List(ref list) => match &list.ident.to_string()[..] {
                    "parse" => {
                        if list.nested.len() != 1 {
                            panic!("unexpected meta item `{}`", tokens_str(item));
                        }

                        self.parse = Some(ParseFn::parse(&list.nested[0]));
                    }
                    _ => panic!("unexpected meta item `{}`", tokens_str(item)),
                },
                Meta::NameValue(ref nv) => match &nv.ident.to_string()[..] {
                    "default" => self.default = Some(lit_str(&nv.lit)),
                    "long" => self.long = Some(lit_str(&nv.lit)),
                    "short" => self.short = Some(lit_char(&nv.lit)),
                    "help" => self.help = Some(lit_str(&nv.lit)),
                    "meta" => self.meta = Some(lit_str(&nv.lit)),
                    _ => panic!("unexpected meta item `{}`", tokens_str(item)),
                },
            },
        }
    }

    pub fn set_defaults(&mut self, defaults: &DefaultOpts) {
        if !self.help_flag && defaults.no_help_flag {
            self.no_help_flag = true;
        }
        if self.short.is_none() && defaults.no_short {
            self.no_short = true;
        }
        if self.long.is_none() && defaults.no_long {
            self.no_long = true;
        }

        if self.not_required {
            self.required = false;
        } else if defaults.required {
            self.required = true;
        }
    }
}

fn lit_char(lit: &Lit) -> char {
    match *lit {
        // Character literals in attributes are not necessarily allowed
        Lit::Str(ref s) => {
            let s = s.value();
            let mut chars = s.chars();

            let res = chars.next().expect("expected one-char string literal");
            if chars.next().is_some() {
                panic!("expected one-char string literal");
            }

            res
        }
        Lit::Char(ref ch) => ch.value(),
        _ => panic!("unexpected literal `{}`", tokens_str(lit)),
    }
}
