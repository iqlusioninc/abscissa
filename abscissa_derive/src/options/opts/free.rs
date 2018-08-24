use proc_macro2::TokenStream as TokenStream2;
use syn::Ident;

use options::{action::FreeAction, parse::ParseFn};

pub struct FreeOpt<'a> {
    pub field: &'a Ident,
    pub action: FreeAction,
    pub parse: ParseFn,
    pub required: bool,
    pub help: Option<String>,
}

impl<'a> FreeOpt<'a> {
    pub fn mark_used(&self) -> TokenStream2 {
        if self.required {
            let field = self.field;
            quote!{ _used.#field = true; }
        } else {
            quote!{}
        }
    }

    pub fn width(&self) -> usize {
        2 + self.field.to_string().len() + 2 // name + spaces before and after
    }
}
