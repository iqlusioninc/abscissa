use proc_macro2::TokenStream as TokenStream2;
use std::iter::repeat;
use syn::{parse_str, Ident, Meta, NestedMeta, Path};

use super::{lit_str, tokens_str};

pub enum ParseFn {
    Default,
    FromStr(Option<Path>),
    TryFromStr(Path),
}

impl Default for ParseFn {
    fn default() -> ParseFn {
        ParseFn::Default
    }
}

impl ParseFn {
    pub fn parse(item: &NestedMeta) -> ParseFn {
        match *item {
            NestedMeta::Meta(Meta::Word(ref ident)) => match &ident.to_string()[..] {
                "from_str" => ParseFn::FromStr(None),
                "try_from_str" => ParseFn::Default,
                _ => panic!("unexpected meta item `{}`", tokens_str(item)),
            },
            NestedMeta::Meta(Meta::NameValue(ref nv)) => match &nv.ident.to_string()[..] {
                "from_str" => {
                    let path = parse_str(&lit_str(&nv.lit)).unwrap();
                    ParseFn::FromStr(Some(path))
                }
                "try_from_str" => {
                    let path = parse_str(&lit_str(&nv.lit)).unwrap();
                    ParseFn::TryFromStr(path)
                }
                _ => panic!("unexpected meta item `{}`", tokens_str(item)),
            },
            _ => panic!("unexpected meta item `{}`", tokens_str(item)),
        }
    }

    pub fn make_parse_action(&self) -> TokenStream2 {
        match *self {
            ParseFn::Default => quote! {
                ::std::str::FromStr::from_str(_arg)
                    .map_err(|e| ::abscissa::options::Error::failed_parse(_opt,
                        ::std::string::ToString::to_string(&e)))?
            },
            ParseFn::FromStr(None) => quote! {
                ::std::convert::From::from(_arg)
            },
            ParseFn::FromStr(Some(ref fun)) => quote! {
                #fun(_arg)
            },
            ParseFn::TryFromStr(ref fun) => quote! {
                #fun(_arg)
                    .map_err(|e| ::abscissa::options::Error::failed_parse(_opt,
                        ::std::string::ToString::to_string(&e)))?
            },
        }
    }

    pub fn make_parse_default_action(&self, ident: &Ident, expr: &str) -> TokenStream2 {
        match *self {
            ParseFn::Default => quote! {
                ::std::str::FromStr::from_str(#expr)
                    .map_err(|e| ::abscissa::options::Error::failed_parse_default(
                        stringify!(#ident), #expr,
                        ::std::string::ToString::to_string(&e)))?
            },
            ParseFn::FromStr(None) => quote! {
                ::std::convert::From::from(#expr)
            },
            ParseFn::FromStr(Some(ref fun)) => quote! {
                #fun(#expr)
            },
            ParseFn::TryFromStr(ref fun) => quote! {
                #fun(#expr)
                    .map_err(|e| ::abscissa::options::Error::failed_parse_default(
                        stringify!(#ident), #expr,
                        ::std::string::ToString::to_string(&e)))?
            },
        }
    }
}

pub struct ParseMethod {
    pub parse_fn: ParseFn,
    pub tuple_len: Option<usize>,
}

impl ParseMethod {
    pub fn make_action_type(&self) -> TokenStream2 {
        let parse = self.parse_fn.make_parse_action();

        match self.tuple_len {
            None => quote! { {
                let _arg = _parser.next_arg()
                    .ok_or_else(|| ::abscissa::options::Error::missing_argument(_opt))?;

                #parse
            } },
            Some(n) => {
                let num = 0..n;
                let n = repeat(n);
                let parse = repeat(parse);

                quote! {
                    ( #( {
                        let _found = #num;
                        let _arg = _parser.next_arg()
                            .ok_or_else(|| ::abscissa::options::Error::insufficient_arguments(
                                _opt, #n, _found))?;

                        #parse
                    } , )* )
                }
            }
        }
    }

    pub fn make_action_type_arg(&self) -> TokenStream2 {
        match self.tuple_len {
            None => self.parse_fn.make_parse_action(),
            Some(_) => unreachable!(),
        }
    }

    pub fn takes_arg(&self) -> bool {
        match self.tuple_len {
            Some(0) => false,
            _ => true,
        }
    }
}
