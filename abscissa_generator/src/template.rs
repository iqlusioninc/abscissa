//! Abscissa application templates

use crate::properties::Properties;
use std::path::{Path, PathBuf};

mod collection;
mod iter;
mod name;

pub use self::{collection::Collection, iter::Iter, name::Name};

/// Directory in an application template where binaries live
const APP_BIN_DIR: &str = "src/bin/app";

/// Template file (Handlebars)
pub struct Template<'a> {
    /// Template name
    name: Name,

    /// Inner Handlebars template
    inner: &'a handlebars::Template,
}

impl<'a> Template<'a> {
    /// Make a new `TemplateEntry`
    fn new(name: &'a str, template: &'a handlebars::Template) -> Self {
        Self {
            name: Name::from(name),
            inner: template,
        }
    }

    /// Get the template's name
    pub fn name(&self) -> &Name {
        &self.name
    }
}

impl<'a> Template<'a> {
    /// Relative path in the application where the output file should be located
    pub fn output_path(&self, properties: &Properties) -> PathBuf {
        let path = Path::new(self.name().as_ref());

        // Remove the '.hbs' extension (the `Name` type ensures we have one)
        let stemmed_path = path
            .parent()
            .expect("no output parent path")
            .join(path.file_stem().expect("no file stem"));

        // If we're in the `src/bin/app` directory, rename 'app' appropriately
        if stemmed_path.starts_with(APP_BIN_DIR) {
            Path::new(APP_BIN_DIR)
                .parent()
                .unwrap()
                .join(properties.name.as_ref())
                .join(stemmed_path.strip_prefix(APP_BIN_DIR).unwrap())
        } else {
            stemmed_path
        }
    }
}
