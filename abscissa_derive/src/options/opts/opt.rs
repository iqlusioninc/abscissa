use proc_macro2::TokenStream as TokenStream2;
use std::iter::repeat;
use syn::Ident;

use options::action::Action;

pub struct Opt<'a> {
    pub field: &'a Ident,
    pub action: Action,
    pub long: Option<String>,
    pub short: Option<char>,
    pub no_short: bool,
    pub required: bool,
    pub help: Option<String>,
    pub meta: Option<String>,
    pub default: Option<String>,
}

impl<'a> Opt<'a> {
    pub fn display_form(&self) -> String {
        if let Some(ref long) = self.long {
            format!("--{}", long)
        } else {
            format!("-{}", self.short.unwrap())
        }
    }

    pub fn mark_used(&self) -> TokenStream2 {
        if self.required {
            let field = self.field;
            quote! { _used.#field = true; }
        } else {
            quote!{}
        }
    }

    pub fn width(&self) -> usize {
        let short = self.short.map_or(0, |_| 1 + 1); // '-' + char
        let long = self.long.as_ref().map_or(0, |s| s.len() + 2); // "--" + str
        let sep = if short == 0 || long == 0 { 0 } else { 2 }; // ", "
        let meta = self.meta.as_ref().map_or(0, |s| s.len() + 1); // ' ' + meta

        2 + short + long + sep + meta + 2 // total + spaces before and after
    }

    pub fn make_action(&self) -> TokenStream2 {
        use options::action::Action::*;

        let field = self.field;
        let mark_used = self.mark_used();

        let action = match self.action {
            Count => quote! {
                _result.#field += 1;
            },
            Push(ref meth) => {
                let act = meth.make_action_type();

                quote! {
                    _result.#field.push(#act);
                }
            }
            SetField(ref meth) => {
                let act = meth.make_action_type();

                quote! {
                    _result.#field = #act;
                }
            }
            SetOption(ref meth) => {
                let act = meth.make_action_type();

                quote! {
                    _result.#field = ::std::option::Option::Some(#act);
                }
            }
            Switch => quote! {
                _result.#field = true;
            },
        };

        quote! {
            #mark_used
            #action
        }
    }

    pub fn make_action_arg(&self) -> TokenStream2 {
        use options::action::Action::*;

        let field = self.field;
        let mark_used = self.mark_used();

        let action = match self.action {
            Push(ref meth) => {
                let act = meth.make_action_type_arg();

                quote! {
                    _result.#field.push(#act);
                }
            }
            SetField(ref meth) => {
                let act = meth.make_action_type_arg();

                quote! {
                    _result.#field = #act;
                }
            }
            SetOption(ref meth) => {
                let act = meth.make_action_type_arg();

                quote! {
                    _result.#field = ::std::option::Option::Some(#act);
                }
            }
            _ => unreachable!(),
        };

        quote! {
            #mark_used
            #action
        }
    }

    pub fn usage(&self, col_width: usize) -> String {
        let mut res = String::from("  ");

        if let Some(short) = self.short {
            res.push('-');
            res.push(short);
        }

        if self.short.is_some() && self.long.is_some() {
            res.push_str(", ");
        }

        if let Some(ref long) = self.long {
            res.push_str("--");
            res.push_str(long);
        }

        if let Some(ref meta) = self.meta {
            res.push(' ');
            res.push_str(meta);
        }

        if self.help.is_some() || self.default.is_some() {
            if res.len() < col_width {
                let n = col_width - res.len();
                res.extend(repeat(' ').take(n));
            } else {
                res.push('\n');
                res.extend(repeat(' ').take(col_width));
            }
        }

        if let Some(ref help) = self.help {
            res.push_str(help);
        }

        if let Some(ref default) = self.default {
            res.push_str(" (default: ");
            res.push_str(default);
            res.push_str(")");
        }

        res
    }
}
