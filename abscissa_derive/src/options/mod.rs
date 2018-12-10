//! Provides `derive(Options)` for the `abscissa` crate
//!
//! # `derive(Options)`
//!
//! `derive(Options)` generates an implementation of the trait `Options`,
//! creating an option for each field of the decorated `struct`.
//!
//! See the `abscissa` [documentation](https://docs.rs/abscissa/) for an example
//! of its usage.
//!
//! ## `options` attribute
//!
//! Behavior of `derive(Options)` can be controlled by adding `#[options(...)]`
//! attributes to one or more fields within a decorated struct.
//!
//! Supported items are:
//!
//! * `command` indicates that a field represents a subcommand. The field must
//!   be of type `Option<T>` where `T` is a type implementing `Options`.
//!   Typically, this type is an `enum` containing subcommand option types.
//! * `help_flag` marks an option as a help flag. The field must be `bool` type.
//!   Options named `help` will automatically receive this option.
//! * `no_help_flag` prevents an option from being considered a help flag.
//! * `count` marks a field as a counter value. The field will be incremented
//!   each time the option appears in the arguments, i.e. `field += 1;`
//! * `free` marks a field as a positional argument field. Non-option arguments
//!   will be used to fill all `free` fields, in declared sequence.
//!   If the final `free` field is of type `Vec<T>`, it will contain all
//!   remaining free arguments.
//! * `short = "?"` sets the short option name to the given character
//! * `no_short` prevents a short option from being assigned to the field
//! * `long = "..."` sets the long option name to the given string
//! * `no_long` prevents a long option from being assigned to the field
//! * `required` will cause an error if the option is not present
//! * `not_required` will cancel a type-level `required` flag (see below).
//! * `help = "..."` sets help text returned from the `Options::usage` method
//! * `meta = "..."` sets the meta variable displayed in usage for options
//!   which accept an argument
//! * `parse(...)` uses a named function to parse a value from a string.
//!   Valid parsing function types are:
//!     * `parse(from_str = "...")` for `fn(&str) -> T`
//!     * `parse(try_from_str = "...")` for
//!       `fn(&str) -> Result<T, E> where E: Display`
//!     * `parse(from_str)` uses `std::convert::From::from`
//!     * `parse(try_from_str)` uses `std::str::FromStr::from_str`
//!
//! `#[options(...)]` may also be added at the type level. Only the flags
//! `no_help_flag`, `no_long`, `no_short`, and `required`
//! are supported at the type level.
//!
//! # Notice
//!
//! The `options` module of the `abscissa_derive` crate is a fork of the
//! `gumdrop_derive` crate:
//!
//! <https://github.com/murarth/gumdrop>
//!
//! Author: Murarth <murarth@gmail.com>

use std::iter::repeat;

use proc_macro::TokenStream;

use quote::ToTokens;
use syn::{DataEnum, DeriveInput, Fields, GenericArgument, Lit, PathArguments, Type};

mod action;
mod opts;
mod parse;

use self::{
    action::{Action, FreeAction},
    opts::{
        attr::AttrOpts,
        cmd::{Cmd, CmdOpts},
        default::DefaultOpts,
        free::FreeOpt,
        opt::Opt,
    },
    parse::ParseFn,
};

/// Derive the `Options` trait on an enum
pub(crate) fn derive_options_enum(ast: &DeriveInput, data: &DataEnum) -> TokenStream {
    let name = &ast.ident;
    let mut commands = Vec::new();
    let mut var_ty = Vec::new();

    for var in &data.variants {
        let ty = match var.fields {
            Fields::Unit | Fields::Named(_) => {
                panic!("command variants must be unary tuple variants")
            }
            Fields::Unnamed(ref fields) if fields.unnamed.len() != 1 => {
                panic!("command variants must be unary tuple variants")
            }
            Fields::Unnamed(ref fields) => &fields.unnamed.first().unwrap().into_value().ty,
        };

        let opts = CmdOpts::parse(&var.attrs);

        let var_name = &var.ident;

        var_ty.push(ty);

        commands.push(Cmd {
            name: opts
                .name
                .unwrap_or_else(|| make_command_name(&var_name.to_string())),
            help: opts.help,
            variant_name: var_name,
            ty,
        });
    }

    let mut command = Vec::new();
    let mut handle_cmd = Vec::new();
    let mut help_req_impl = Vec::new();
    let mut variant = Vec::new();
    let usage = make_cmd_usage(&commands);

    for cmd in commands {
        command.push(cmd.name);

        let var_name = cmd.variant_name;
        let ty = &cmd.ty;

        variant.push(var_name);

        handle_cmd.push(quote! {
            #name::#var_name(<#ty as ::abscissa::Options>::parse(_parser)?)
        });

        help_req_impl.push(quote! {
            #name::#var_name(ref cmd) => { ::abscissa::Options::help_requested(cmd) }
        });
    }

    // Borrow re-used items
    let command = &command;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let command_name_impl = {
        let name = repeat(name);

        quote! {
            match *self {
                #( #name::#variant(_) => ::std::option::Option::Some(#command), )*
            }
        }
    };

    let expr = quote! {
        impl #impl_generics ::abscissa::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    _parser: &mut ::abscissa::options::Parser<__S>)
                    -> ::std::result::Result<Self, ::abscissa::options::Error> {
                let _arg = _parser.next_arg()
                    .ok_or_else(::abscissa::options::Error::missing_command)?;

                Self::parse_command(_arg, _parser)
            }

            fn command_name(&self) -> ::std::option::Option<&'static str> {
                #command_name_impl
            }

            fn help_requested(&self) -> bool {
                match *self {
                    #( #help_req_impl )*
                }
            }

            fn parse_command<__S: ::std::convert::AsRef<str>>(name: &str,
                    _parser: &mut ::abscissa::options::Parser<__S>)
                    -> ::std::result::Result<Self, ::abscissa::options::Error> {
                let cmd = match name {
                    #( #command => { #handle_cmd } )*
                    _ => return ::std::result::Result::Err(
                        ::abscissa::options::Error::unrecognized_command(name))
                };

                ::std::result::Result::Ok(cmd)
            }

            fn usage() -> &'static str {
                #usage
            }

            fn command_list() -> ::std::option::Option<&'static str> {
                ::std::option::Option::Some(<Self as ::abscissa::Options>::usage())
            }

            fn command_usage(name: &str) -> ::std::option::Option<&'static str> {
                match name {
                    #( #command => ::std::option::Option::Some(
                        <#var_ty as ::abscissa::Options>::usage()), )*
                    _ => ::std::option::Option::None
                }
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

#[allow(clippy::cyclomatic_complexity)]
pub(crate) fn derive_options_struct(ast: &DeriveInput, fields: &Fields) -> TokenStream {
    let mut pattern = Vec::new();
    let mut handle_opt = Vec::new();
    let mut short_names = Vec::new();
    let mut long_names = Vec::new();
    let mut free: Vec<FreeOpt> = Vec::new();
    let mut required = Vec::new();
    let mut required_err = Vec::new();
    let mut command = None;
    let mut command_ty = None;
    let mut command_required = false;
    let mut help_flag = Vec::new();
    let mut options = Vec::new();
    let mut field_name = Vec::new();
    let mut default = Vec::new();

    let default_expr = quote! { ::std::default::Default::default() };
    let default_opts = DefaultOpts::parse(&ast.attrs);

    for field in fields {
        let mut opts = AttrOpts::parse(&field.attrs);
        opts.set_defaults(&default_opts);

        let ident = field.ident.as_ref().unwrap();

        field_name.push(ident);

        if let Some(ref expr) = opts.default {
            default.push(
                opts.parse
                    .as_ref()
                    .unwrap_or(&ParseFn::Default)
                    .make_parse_default_action(ident, &expr),
            );
        } else {
            default.push(default_expr.clone());
        }

        if opts.command {
            if command.is_some() {
                panic!("duplicate declaration of `command` field");
            }
            if !free.is_empty() {
                panic!("`command` and `free` options are mutually exclusive");
            }

            command = Some(ident);
            command_ty = Some(first_ty_param(&field.ty).unwrap_or(&field.ty));
            command_required = opts.required;

            if opts.required {
                required.push(ident);
                required_err.push(quote! {
                ::abscissa::options::Error::missing_required_command() });
            }

            continue;
        }

        if opts.free {
            if command.is_some() {
                panic!("`command` and `free` options are mutually exclusive");
            }

            if let Some(last) = free.last() {
                if last.action == FreeAction::Push {
                    panic!("only the final `free` option may be of type `Vec<T>`");
                }
            }

            if opts.required {
                required.push(ident);
                required_err.push(quote! {
                ::abscissa::options::Error::missing_required_free() });
            }

            free.push(FreeOpt {
                field: ident,
                action: FreeAction::infer(&field.ty),
                parse: opts.parse.unwrap_or_default(),
                required: opts.required,
                help: opts.help,
            });

            continue;
        }

        if opts.long.is_none() && !opts.no_long {
            opts.long = Some(make_long_name(&ident.to_string()));
        }

        if let Some(ref long) = opts.long {
            validate_long_name(long, &long_names);
            long_names.push(long.clone());
        }

        if let Some(short) = opts.short {
            validate_short_name(short, &short_names);
            short_names.push(short);
        }

        if opts.help_flag
            || (!opts.no_help_flag && opts.long.as_ref().map(|s| &s[..]) == Some("help"))
        {
            help_flag.push(ident);
        }

        let action = if opts.count {
            Action::Count
        } else {
            Action::infer(&field.ty, opts.parse)
        };

        if action.takes_arg() {
            if opts.meta.is_none() {
                opts.meta = Some(make_meta(&ident.to_string(), &action));
            }
        } else if opts.meta.is_some() {
            panic!("`meta` value is invalid for option `{}`", ident);
        }

        options.push(Opt {
            field: ident,
            action,
            long: opts.long.take(),
            short: opts.short,
            no_short: opts.no_short,
            required: opts.required,
            meta: opts.meta.take(),
            help: opts.help.take(),
            default: opts.default.take(),
        });
    }

    // Assign short names after checking all options.
    // Thus, manual short names will take priority over automatic ones.
    for opt in &mut options {
        if opt.short.is_none() && !opt.no_short {
            let short = make_short_name(&opt.field.to_string(), &short_names);

            if let Some(short) = short {
                short_names.push(short);
            }

            opt.short = short;
        }
    }

    for opt in &options {
        if opt.required {
            required.push(opt.field);
            let display = opt.display_form();
            required_err.push(quote! {
            ::abscissa::options::Error::missing_required(#display) });
        }

        let pat = match (opt.long.as_ref(), opt.short) {
            (Some(long), Some(short)) => quote! {
                ::abscissa::options::Opt::Long(#long) | ::abscissa::options::Opt::Short(#short)
            },
            (Some(long), None) => quote! {
                ::abscissa::options::Opt::Long(#long)
            },
            (None, Some(short)) => quote! {
                ::abscissa::options::Opt::Short(#short)
            },
            (None, None) => {
                panic!("option `{}` has no long or short flags", opt.field);
            }
        };

        pattern.push(pat);
        handle_opt.push(opt.make_action());

        if let Some(ref long) = opt.long {
            let (pat, handle) = if let Some(n) = opt.action.tuple_len() {
                (
                    quote! { ::abscissa::options::Opt::LongWithArg(#long, _) },
                    quote! { return ::std::result::Result::Err(
                    ::abscissa::options::Error::unexpected_single_argument(_opt, #n)) },
                )
            } else if opt.action.takes_arg() {
                (
                    quote! { ::abscissa::options::Opt::LongWithArg(#long, _arg) },
                    opt.make_action_arg(),
                )
            } else {
                (
                    quote! { ::abscissa::options::Opt::LongWithArg(#long, _) },
                    quote! { return ::std::result::Result::Err(
                    ::abscissa::options::Error::unexpected_argument(_opt)) },
                )
            };

            pattern.push(pat);
            handle_opt.push(handle);
        }
    }

    let name = &ast.ident;
    let usage = make_usage(&free, &options);

    let handle_free = if !free.is_empty() {
        let catch_all = if free.last().unwrap().action == FreeAction::Push {
            let last = free.pop().unwrap();

            let free = last.field;

            let parse = last.parse.make_parse_action();
            let mark_used = last.mark_used();

            quote! {
                #mark_used
                let _arg = _free;
                _result.#free.push(#parse);
            }
        } else {
            quote! {
                return ::std::result::Result::Err(
                    ::abscissa::options::Error::unexpected_free(_free))
            }
        };

        let num = 0..free.len();
        let action = free
            .iter()
            .map(|free| {
                let field = free.field;

                let mark_used = free.mark_used();
                let parse = free.parse.make_parse_action();

                let assign = match free.action {
                    FreeAction::Push => quote! {
                        let _arg = _free;
                        _result.#field.push(#parse);
                    },
                    FreeAction::SetField => quote! {
                        let _arg = _free;
                        _result.#field = #parse;
                    },
                    FreeAction::SetOption => quote! {
                        let _arg = _free;
                        _result.#field = ::std::option::Option::Some(#parse);
                    },
                };

                quote! {
                    #mark_used
                    #assign
                }
            })
            .collect::<Vec<_>>();

        quote! {
            match _free_counter {
                #( #num => {
                    _free_counter += 1;
                    #action
                } )*
                _ => { #catch_all }
            }
        }
    } else if let Some(ident) = command {
        let mark_used = if command_required {
            quote! { _used.#ident = true; }
        } else {
            quote! {}
        };

        quote! {
            #mark_used
            _result.#ident = ::std::option::Option::Some(
                ::abscissa::Options::parse_command(_free, _parser)?);
            break;
        }
    } else {
        quote! {
            return ::std::result::Result::Err(
                ::abscissa::options::Error::unexpected_free(_free));
        }
    };

    let command_name_impl = match command {
        None => quote! { ::std::option::Option::None },
        Some(ref field) => quote! {
            ::std::option::Option::and_then(
                ::std::option::Option::as_ref(&self.#field),
                ::abscissa::Options::command_name)
        },
    };

    let command_list = match command_ty {
        Some(ty) => quote! {
            ::std::option::Option::Some(
                <#ty as ::abscissa::Options>::usage())
        },
        None => quote! {
            ::std::option::Option::None
        },
    };

    let command_usage = match command_ty {
        Some(ty) => quote! {
            <#ty as ::abscissa::Options>::command_usage(_name)
        },
        None => quote! {
            ::std::option::Option::None
        },
    };

    let help_requested_impl = match (&help_flag, &command) {
        (flags, &None) => quote! {
            fn help_requested(&self) -> bool {
                false #( || self.#flags )*
            }
        },
        (flags, &Some(ref cmd)) => quote! {
            fn help_requested(&self) -> bool {
                #( self.#flags || )*
                ::std::option::Option::map_or(
                    ::std::option::Option::as_ref(&self.#cmd),
                    false, ::abscissa::Options::help_requested)
            }
        },
    };

    let required = &required;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let expr = quote! {
        impl #impl_generics ::abscissa::Options for #name #ty_generics #where_clause {
            fn parse<__S: ::std::convert::AsRef<str>>(
                    _parser: &mut ::abscissa::options::Parser<__S>)
                    -> ::std::result::Result<Self, ::abscissa::options::Error> {
                #[derive(Default)]
                struct _Used {
                    #( #required: bool , )*
                }

                let mut _result = #name{
                    #( #field_name: #default ),*
                };
                let mut _free_counter = 0usize;
                let mut _used = _Used::default();

                while let ::std::option::Option::Some(_opt) = _parser.next_opt() {
                    match _opt {
                        #( #pattern => { #handle_opt } )*
                        ::abscissa::options::Opt::Free(_free) => {
                            #handle_free
                        }
                        _ => {
                            return ::std::result::Result::Err(
                                ::abscissa::options::Error::unrecognized_option(_opt));
                        }
                    }
                }

                #( if !_used.#required {
                    return ::std::result::Result::Err(#required_err);
                } )*

                ::std::result::Result::Ok(_result)
            }

            fn command_name(&self) -> ::std::option::Option<&'static str> {
                #command_name_impl
            }

            #help_requested_impl

            fn parse_command<__S: ::std::convert::AsRef<str>>(name: &str,
                    _parser: &mut ::abscissa::options::Parser<__S>)
                    -> ::std::result::Result<Self, ::abscissa::options::Error> {
                ::std::result::Result::Err(
                    ::abscissa::options::Error::unrecognized_command(name))
            }

            fn usage() -> &'static str {
                #usage
            }

            fn command_list() -> ::std::option::Option<&'static str> {
                #command_list
            }

            fn command_usage(_name: &str) -> ::std::option::Option<&'static str> {
                #command_usage
            }
        }
    };

    expr.to_string().parse().expect("parse quote!")
}

pub(crate) fn first_ty_param(ty: &Type) -> Option<&Type> {
    match *ty {
        Type::Path(ref path) => {
            let path = path.path.segments.last().unwrap().into_value();

            match path.arguments {
                PathArguments::AngleBracketed(ref data) => data
                    .args
                    .iter()
                    .filter_map(|arg| {
                        if let GenericArgument::Type(ref ty) = arg {
                            Some(ty)
                        } else {
                            None
                        }
                    })
                    .next(),
                _ => None,
            }
        }
        _ => None,
    }
}

pub(crate) fn lit_str(lit: &Lit) -> String {
    match *lit {
        Lit::Str(ref s) => s.value(),
        _ => panic!("unexpected literal `{}`", tokens_str(lit)),
    }
}

pub(crate) fn tokens_str<T: ToTokens>(t: &T) -> String {
    t.into_token_stream().to_string()
}

fn make_command_name(name: &str) -> String {
    let mut res = String::with_capacity(name.len());

    for ch in name.chars() {
        if ch.is_lowercase() {
            res.push(ch);
        } else {
            if !res.is_empty() {
                res.push('-');
            }

            res.extend(ch.to_lowercase());
        }
    }

    res
}

fn make_long_name(name: &str) -> String {
    name.replace('_', "-")
}

fn make_short_name(name: &str, short: &[char]) -> Option<char> {
    let first = name.chars().next().expect("empty field name");

    if !short.contains(&first) {
        return Some(first);
    }

    let mut to_upper = first.to_uppercase();
    let upper = to_upper.next().expect("empty to_uppercase");

    if to_upper.next().is_some() {
        return None;
    }

    if !short.contains(&upper) {
        Some(upper)
    } else {
        None
    }
}

fn validate_long_name(name: &str, names: &[String]) {
    if name.is_empty() || name.starts_with('-') || name.contains(|ch: char| ch.is_whitespace()) {
        panic!("`{}` is not a valid long option", name);
    }

    if names.iter().any(|n| n == name) {
        panic!("duplicate option name `--{}`", name);
    }
}

fn validate_short_name(ch: char, names: &[char]) {
    if ch == '-' || ch.is_whitespace() {
        panic!("`{}` is not a valid short option", ch);
    }

    if names.contains(&ch) {
        panic!("duplicate option name `-{}`", ch);
    }
}

fn make_meta(name: &str, action: &Action) -> String {
    use std::fmt::Write;

    let mut name = name.replace('_', "-").to_uppercase();

    match action.tuple_len() {
        Some(0) => unreachable!(),
        Some(1) | None => (),
        Some(2) => {
            name.push_str(" VALUE");
        }
        Some(n) => {
            for i in 1..n {
                let _ = write!(name, " VALUE{}", i - 1);
            }
        }
    }

    name
}

fn make_usage(free: &[FreeOpt], opts: &[Opt]) -> String {
    let mut res = String::new();

    let width = max_width(free, |opt| opt.width()).max(max_width(opts, |opt| opt.width()));

    if !free.is_empty() {
        res.push_str("Positional arguments:\n");

        for opt in free {
            let mut line = String::from("  ");

            line.push_str(&opt.field.to_string());

            if let Some(ref help) = opt.help {
                if line.len() < width {
                    let n = width - line.len();
                    line.extend(repeat(' ').take(n));
                } else {
                    line.push('\n');
                    line.extend(repeat(' ').take(width));
                }

                line.push_str(help);
            }

            res.push_str(&line);
            res.push('\n');
        }
    }

    if !opts.is_empty() {
        if !res.is_empty() {
            res.push('\n');
        }

        res.push_str("Optional arguments:\n");

        for opt in opts {
            res.push_str(&opt.usage(width));
            res.push('\n');
        }
    }

    // Pop the last newline so the user may println!() the result.
    res.pop();

    res
}

fn max_width<T, F>(items: &[T], f: F) -> usize
where
    F: Fn(&T) -> usize,
{
    const MIN_WIDTH: usize = 8;
    const MAX_WIDTH: usize = 30;

    let width = items
        .iter()
        .filter_map(|item| {
            let w = f(item);

            if w > MAX_WIDTH {
                None
            } else {
                Some(w)
            }
        })
        .max()
        .unwrap_or(0);

    width.max(MIN_WIDTH).min(MAX_WIDTH)
}

fn make_cmd_usage(cmds: &[Cmd]) -> String {
    let mut res = String::new();

    let width = max_width(
        cmds,
        // Two spaces each, before and after
        |cmd| cmd.name.len() + 4,
    );

    for cmd in cmds {
        let mut line = String::from("  ");

        line.push_str(&cmd.name);

        if let Some(ref help) = cmd.help {
            if line.len() < width {
                let n = width - line.len();
                line.extend(repeat(' ').take(n));
            } else {
                line.push('\n');
                line.extend(repeat(' ').take(width));
            }

            line.push_str(help);
        }

        res.push_str(&line);
        res.push('\n');
    }

    // Pop the last newline
    res.pop();

    res
}
