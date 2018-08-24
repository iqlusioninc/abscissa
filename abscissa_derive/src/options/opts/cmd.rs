use syn::{Attribute, Ident, Meta, NestedMeta, Type};

use super::{is_outer, path_eq};
use options::{lit_str, tokens_str};

pub struct Cmd<'a> {
    pub name: String,
    pub help: Option<String>,
    pub variant_name: &'a Ident,
    pub ty: &'a Type,
}

#[derive(Default)]
pub struct CmdOpts {
    pub name: Option<String>,
    pub help: Option<String>,
}

impl CmdOpts {
    pub fn parse(attrs: &[Attribute]) -> CmdOpts {
        let mut opts = CmdOpts::default();

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
                                    Meta::Word(_) | Meta::List(..) => {
                                        panic!("unexpected meta item `{}`", tokens_str(item))
                                    }
                                    Meta::NameValue(ref nv) => match &nv.ident.to_string()[..] {
                                        "name" => opts.name = Some(lit_str(&nv.lit)),
                                        "help" => opts.help = Some(lit_str(&nv.lit)),
                                        _ => panic!("unexpected meta item `{}`", tokens_str(item)),
                                    },
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
