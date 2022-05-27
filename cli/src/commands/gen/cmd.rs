//! Generate a new subcommand in an existing application

use crate::{config::target_app_root, error::Error, handlebars_registry, prelude::*};
use abscissa_core::{clap::Parser, fs, Command, Runnable};
use ident_case::RenameRule;
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    process::exit,
};

/// Subcommand template file
const SUBCOMMAND_TEMPLATE: &str = include_str!("../../../template/src/commands/subcommand.rs.hbs");

/// Generate a new subcommand in an existing application
#[derive(Command, Debug, Parser)]
pub struct Cmd {
    /// Path to the application's `Cargo.toml`
    #[clap(long, short)]
    manifest_path: Option<PathBuf>,

    /// Names of the commands
    #[clap()]
    names: Vec<String>,
}

impl Runnable for Cmd {
    fn run(&self) {
        if self.names.is_empty() {
            status_err!("no command name given!");
            eprintln!("usage: abscissa gen cmd <commandname>");
            exit(1);
        }

        let app_root = target_app_root(self.manifest_path.as_ref().map(AsRef::as_ref));

        for name in &self.names {
            self.generate(&app_root, name)
        }
    }
}

impl Cmd {
    /// Generate a new subcommand in this application
    pub fn generate(&self, app_root: &Path, name: &str) {
        let snake_case_name = name.replace('-', "_");
        let output_dir = app_root.join("src").join("commands");

        if !output_dir.exists() {
            status_err!("command directory missing: {}", output_dir.display());
            exit(1);
        }

        let mut output_path = output_dir.join(&snake_case_name);
        output_path.set_extension("rs");

        self.render(&snake_case_name, &output_path)
            .unwrap_or_else(|e| {
                status_err!("error rendering template: {}", e);
                exit(1);
            })
    }

    /// Render the subcommand template
    pub fn render(&self, snake_case_name: &str, output_path: &Path) -> Result<(), Error> {
        let hbs = handlebars_registry!();
        let output_data =
            hbs.render_template(SUBCOMMAND_TEMPLATE, &Context::new(snake_case_name))?;

        fs::write(output_path, output_data)?;
        status_ok!("Generated", "{}", output_path.display());

        // TODO(tarcieri): do this automagically
        status_info!(
            "Notice",
            "add `pub mod {}` to your app's commands.rs",
            snake_case_name
        );

        Ok(())
    }
}

/// Rendering context for `subcommand.rs.hbs`
#[derive(Serialize)]
struct Context {
    /// Name in lower case (i.e. "kebab-case")
    name_lower: String,

    /// Capitalized (i.e. "PascalCase")
    name_capital: String,
}

impl Context {
    /// Create a new rendering context from the snake case name
    pub fn new(snake_case_name: &str) -> Self {
        let name_lower = RenameRule::KebabCase.apply_to_field(snake_case_name);
        let name_capital = RenameRule::PascalCase.apply_to_field(snake_case_name);
        Self {
            name_lower,
            name_capital,
        }
    }
}
