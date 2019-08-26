//! Template collections manage and render a set of templates intended to
//! generate an Abscissa application.

use super::{iter::Iter, Template};
use crate::{prelude::*, properties::Properties};
use failure::{format_err, Error};
use handlebars::Handlebars;
use std::io;

/// Include a file from the application template
macro_rules! template {
    ($path:expr) => {
        ($path, include_str!(concat!("../../template/", $path)))
    };
}

/// Handlebars template files which define the default application template.
///
/// Presently all of these need to be declared explicitly, and end in `.hbs`
// TODO(tarcieri): use `build.rs` to automatically manage these?
const DEFAULT_TEMPLATE_FILES: &[(&str, &str)] = &[
    template!("Cargo.toml.hbs"),
    template!("README.md.hbs"),
    template!("src/application.rs.hbs"),
    template!("src/bin/app/main.rs.hbs"),
    template!("src/commands.rs.hbs"),
    template!("src/commands/start.rs.hbs"),
    template!("src/commands/version.rs.hbs"),
    template!("src/config.rs.hbs"),
    template!("src/error.rs.hbs"),
    template!("src/lib.rs.hbs"),
    template!("src/prelude.rs.hbs"),
    template!("tests/acceptance.rs.hbs"),
];

/// Abscissa application template renderer
#[derive(Debug)]
pub struct Collection(Handlebars);

impl Default for Collection {
    fn default() -> Collection {
        Collection::new(
            DEFAULT_TEMPLATE_FILES
                .iter()
                .map(|(name, contents)| (*name, *contents)),
        )
        .unwrap()
    }
}

impl Collection {
    /// Create a new template collection by providing an iterator over
    /// `(&name, &contents)` tuples where "name" both names the template
    /// and provides its path within the application.
    pub fn new<'a, I>(template_files: I) -> Result<Collection, Error>
    where
        I: Iterator<Item = (&'a str, &'a str)>,
    {
        let mut hbs = Handlebars::new();
        hbs.set_strict_mode(true);
        hbs.register_escape_fn(handlebars::no_escape);

        for (name, contents) in template_files {
            debug!("registering template: {}", name);

            hbs.register_template_string(name, contents)
                .map_err(|e| format_err!("couldn't register template '{}': {}", name, e))?;
        }

        Ok(Collection(hbs))
    }

    /// Iterate over the templates in the collection
    pub fn iter(&self) -> Iter<'_> {
        // TODO: better way of constructing this `Iter`
        Iter::new(
            self.0
                .get_templates()
                .iter()
                .map(|(path, template)| Template::new(path.as_ref(), template))
                .collect(),
        )
    }

    /// Render a template
    pub fn render<W>(
        &self,
        template: &Template<'_>,
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
