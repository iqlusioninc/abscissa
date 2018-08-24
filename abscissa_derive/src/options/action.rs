use syn::Type;

use super::{
    first_ty_param,
    parse::{ParseFn, ParseMethod},
};

pub enum Action {
    /// Increase count
    Count,

    /// Push an argument to a `Vec<T>` field
    Push(ParseMethod),

    /// Set field
    SetField(ParseMethod),

    /// Set `Option<T>` field
    SetOption(ParseMethod),

    /// Set field to `true`
    Switch,
}

impl Action {
    pub fn infer(ty: &Type, parse: Option<ParseFn>) -> Action {
        match *ty {
            Type::Path(ref path) => {
                let path = path.path.segments.last().unwrap().into_value();
                let param = first_ty_param(ty);

                match &path.ident.to_string()[..] {
                    "bool" if parse.is_none() => Action::Switch,
                    "Vec" if param.is_some() => {
                        let tuple_len = tuple_len(param.unwrap());

                        Action::Push(ParseMethod {
                            parse_fn: parse.unwrap_or_default(),
                            tuple_len,
                        })
                    }
                    "Option" if param.is_some() => {
                        let tuple_len = tuple_len(param.unwrap());

                        Action::SetOption(ParseMethod {
                            parse_fn: parse.unwrap_or_default(),
                            tuple_len,
                        })
                    }
                    _ => Action::SetField(ParseMethod {
                        parse_fn: parse.unwrap_or_default(),
                        tuple_len: tuple_len(ty),
                    }),
                }
            }
            _ => {
                let tuple_len = tuple_len(ty);

                Action::SetField(ParseMethod {
                    parse_fn: parse.unwrap_or_default(),
                    tuple_len,
                })
            }
        }
    }

    pub fn takes_arg(&self) -> bool {
        use self::Action::*;

        match *self {
            Push(ref meth) | SetField(ref meth) | SetOption(ref meth) => meth.takes_arg(),
            _ => false,
        }
    }

    pub fn tuple_len(&self) -> Option<usize> {
        use self::Action::*;

        match *self {
            Push(ref meth) | SetField(ref meth) | SetOption(ref meth) => meth.tuple_len,
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum FreeAction {
    Push,
    SetField,
    SetOption,
}

impl FreeAction {
    pub fn infer(ty: &Type) -> FreeAction {
        match *ty {
            Type::Path(ref path) => {
                let path = path.path.segments.last().unwrap().into_value();

                match &path.ident.to_string()[..] {
                    "Option" => FreeAction::SetOption,
                    "Vec" => FreeAction::Push,
                    _ => FreeAction::SetField,
                }
            }
            _ => FreeAction::SetField,
        }
    }
}

fn tuple_len(ty: &Type) -> Option<usize> {
    match *ty {
        Type::Tuple(ref tup) => Some(tup.elems.len()),
        _ => None,
    }
}
