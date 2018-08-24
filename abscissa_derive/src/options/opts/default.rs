use syn::{Attribute, Meta, NestedMeta};

use super::{is_outer, path_eq};
use options::tokens_str;

#[derive(Default)]
pub struct DefaultOpts {
    pub no_help_flag: bool,
    pub no_long: bool,
    pub no_short: bool,
    pub required: bool,
}

impl DefaultOpts {
    pub fn parse(attrs: &[Attribute]) -> DefaultOpts {
        let mut opts = DefaultOpts::default();

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
                            match *item {
                                NestedMeta::Literal(_) => {
                                    panic!("unexpected meta item `{}`", tokens_str(item))
                                }
                                NestedMeta::Meta(ref item) => match *item {
                                    Meta::Word(ref w) => match &w.to_string()[..] {
                                        "no_help_flag" => opts.no_help_flag = true,
                                        "no_short" => opts.no_short = true,
                                        "no_long" => opts.no_long = true,
                                        "required" => opts.required = true,
                                        _ => panic!("unexpected meta item `{}`", tokens_str(item)),
                                    },
                                    Meta::List(..) | Meta::NameValue(..) => {
                                        panic!("unexpected meta item `{}`", tokens_str(item))
                                    }
                                },
                            }
                        }
                    }
                }
            }
        }

        opts
    }
}
