//! Abscissa application template renderer

use crate::properties::Properties;
use failure::{format_err, Error};
use handlebars::Handlebars;
use hashbrown::hash_map;
use log::debug;
use std::{
    fmt, io,
    path::{Path, PathBuf},
};

/// Include a file from the application template
macro_rules! template {
    ($path:expr) => {
        ($path, include_str!(concat!("../template/", $path)))
    };
}

/// Template files to render.
///
/// Right now all of these need to be declared explicitly, and end in `.hbs`
const FILES: &[(&str, &str)] = &[
    template!("Cargo.toml.hbs"),
    template!("README.md.hbs"),
    template!("src/application.rs.hbs"),
    template!("src/bin/app/main.rs.hbs"),
    template!("src/commands.rs.hbs"),
    template!("src/commands/hello.rs.hbs"),
    template!("src/commands/version.rs.hbs"),
    template!("src/config.rs.hbs"),
    template!("src/error.rs.hbs"),
    template!("src/lib.rs.hbs"),
];

/// Abscissa application template renderer
#[derive(Debug)]
pub struct AppTemplate(Handlebars);

impl Default for AppTemplate {
    fn default() -> AppTemplate {
        AppTemplate::new(
            FILES
                .iter()
                .map(|(name, contents)| File::new(*name, *contents)),
        )
        .unwrap()
    }
}

impl AppTemplate {
    pub fn new<I>(template_files: I) -> Result<AppTemplate, Error>
    where
        I: IntoIterator<Item = File>,
    {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_escape_fn(handlebars::no_escape);

        for file in template_files.into_iter() {
            debug!("registering template: {}", file.name);

            hbs.register_template_string(file.name.as_ref(), file.contents.as_ref())
                .map_err(|e| {
                    format_err!("couldn't register template '{}': {}", file.name.as_ref(), e)
                })?;
        }

        Ok(AppTemplate(hbs))
    }

    /// Iterate over the templates in the collection
    pub fn iter(&self) -> Iter {
        Iter::new(self.0.get_templates().iter())
    }

    /// Render a template
    pub fn render<W>(
        &self,
        template: &TemplateEntry,
        properties: &Properties,
        output: W,
    ) -> Result<(), Error>
    where
        W: io::Write,
    {
        let ctx = handlebars::Context::wraps(properties)?;
        let mut render_context = handlebars::RenderContext::new(template.inner.name.as_ref());
        let mut output = Output::new(output);

        use handlebars::Renderable;
        template
            .inner
            .render(&self.0, &ctx, &mut render_context, &mut output)
            .map_err(|e| format_err!("render error: {}", e))
    }
}

type HandlebarsIter<'a> = hash_map::Iter<'a, String, handlebars::Template>;

/// Iterator over all registered template files
pub struct Iter<'a>(HandlebarsIter<'a>);

impl<'a> Iter<'a> {
    fn new(templates: HandlebarsIter<'a>) -> Self {
        Iter(templates)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = TemplateEntry<'a>;

    fn next(&mut self) -> Option<TemplateEntry<'a>> {
        self.0
            .next()
            .map(|(path, template)| TemplateEntry::new(path.as_ref(), template))
    }
}

/// Directory in an application template where binaries live
const APP_BIN_DIR: &str = "src/bin/app";

/// Template for an individual file
pub struct TemplateEntry<'a> {
    /// Template name
    name: Name,

    /// Inner Handlebars template
    inner: &'a handlebars::Template,
}

impl<'a> TemplateEntry<'a> {
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

impl<'a> TemplateEntry<'a> {
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

/// Wrapper for writing template output to a file
struct Output<W: io::Write>(W);

impl<W: io::Write> Output<W> {
    pub fn new(writeable: W) -> Output<W> {
        Output(writeable)
    }
}

impl<W: io::Write> handlebars::Output for Output<W> {
    fn write(&mut self, seg: &str) -> Result<(), io::Error> {
        self.0.write_all(seg.as_bytes())
    }
}

/// Individual template file
#[derive(Debug)]
pub struct File {
    /// Name of a template file
    name: Name,

    /// Contents of the template
    contents: Contents,
}

impl File {
    fn new<N, C>(name: N, contents: C) -> File
    where
        N: Into<Name>,
        C: Into<Contents>,
    {
        File {
            name: name.into(),
            contents: contents.into(),
        }
    }
}

/// Template names
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct Name(String);

impl<'a> From<&'a str> for Name {
    fn from(name: &'a str) -> Name {
        assert!(name.ends_with(".hbs"), "template files must end with .hbs");
        Name(name.to_owned())
    }
}

impl AsRef<str> for Name {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Template file body
#[derive(Clone, Debug)]
pub struct Contents(String);

impl<'a> From<&'a str> for Contents {
    fn from(contents: &'a str) -> Contents {
        Contents(contents.to_owned())
    }
}

impl AsRef<str> for Contents {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
