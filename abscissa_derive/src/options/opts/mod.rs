pub mod attr;
pub mod cmd;
pub mod default;
pub mod free;
pub mod opt;

use syn::{AttrStyle, Path, PathArguments};

fn is_outer(style: AttrStyle) -> bool {
    match style {
        AttrStyle::Outer => true,
        _ => false,
    }
}

fn path_eq(path: &Path, s: &str) -> bool {
    path.segments.len() == 1 && {
        let seg = path.segments.first().unwrap().into_value();

        match seg.arguments {
            PathArguments::None => seg.ident == s,
            _ => false,
        }
    }
}
