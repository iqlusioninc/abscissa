//! `generate` subcommand - generate a new Abscissa application

#![allow(clippy::never_loop)]

use abscissa::{inflector::TitleCase, status_err, status_ok, Command, Options, Runnable};
use abscissa_generator::{
    properties::{self, Properties},
    template::{Collection, Template},
};
use failure::{bail, format_err, Error};
use std::{
    fs,
    path::{Path, PathBuf},
    process,
    time::Instant,
};

/// `new` subcommand - generate a new Abscissa application
#[derive(Command, Debug, Default, Options)]
pub struct NewCommand {
    /// Add a `[patch.crates-io]` section to Cargo.toml
    #[options(no_short, help = "add patch.crates-io to Cargo.toml")]
    patch_crates_io: Option<String>,

    /// Path to the newly generated application
    #[options(free)]
    app_path: Option<PathBuf>,
}

impl Runnable for NewCommand {
    /// Run the Abscissa application generator
    fn run(&self) {
        let started_at = Instant::now();
        let app_properties = self.parse_options().unwrap_or_else(|e| fatal_error(e));
        let app_template = Collection::default();

        self.create_parent_directory()
            .unwrap_or_else(|e| fatal_error(e));

        // Sort template files alphabetically
        let mut template_files = app_template.iter().collect::<Vec<_>>();
        template_files.sort_by(|a, b| a.name().cmp(b.name()));

        for template_file in &template_files {
            self.render_template_file(&app_template, &template_file, &app_properties)
                .unwrap_or_else(|e| fatal_error(e));
        }

        let duration = started_at.elapsed();

        status_ok!(
            "Finished",
            "`{}` generated in {:.2}s",
            &app_properties.name,
            duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) * 1e-9
        );
    }
}

impl NewCommand {
    /// Get the path to the newly generated application
    fn app_path(&self) -> Result<&Path, Error> {
        match &self.app_path {
            Some(path) => Ok(path.as_ref()),
            None => bail!("no app_path given"),
        }
    }

    /// Create the parent directory for the newly generated application (if necessary)
    fn create_parent_directory(&self) -> Result<(), Error> {
        let app_path = self.app_path()?;

        if app_path.exists() {
            fatal_error(format_err!(
                "destination `{}` already exists",
                app_path.display()
            ));
        }

        fs::create_dir(app_path)
            .map_err(|e| format_err!("couldn't create {}: {}", app_path.display(), e))?;

        status_ok!(
            "Created",
            "`{}` (application directory)",
            app_path.display()
        );

        Ok(())
    }

    /// Render an individual template file
    fn render_template_file(
        &self,
        app_template: &Collection,
        template_file: &Template,
        app_properties: &Properties,
    ) -> Result<(), Error> {
        let output_path_rel = template_file.output_path(app_properties);
        let output_path = self.app_path()?.join(&output_path_rel);

        if output_path.exists() {
            fatal_error(format_err!(
                "file already exists: {}",
                output_path.display()
            ));
        }

        // We should always have a parent directory
        let output_dir = output_path.parent().unwrap();

        // Create all of the necessary parent directories
        fs::create_dir_all(output_dir)
            .map_err(|e| format_err!("error creating {}: {}", output_dir.display(), e))?;

        let mut output_file = fs::File::create(&output_path)
            .map_err(|e| format_err!("couldn't create {}: {}", output_path.display(), e))?;

        app_template.render(template_file, &app_properties, &mut output_file)?;

        status_ok!("Created", "new file: {}", output_path_rel.display());

        Ok(())
    }

    /// Parse `abscissa_generate` properties from command-line options
    fn parse_options(&self) -> Result<Properties, Error> {
        let abscissa = properties::framework::Properties::new(abscissa::VERSION);
        let app_path = self.app_path()?;

        let app_name = app_path
            .file_name()
            .expect("no filename?")
            .to_string_lossy();

        let name: properties::name::App = app_name.parse().expect("no app name");

        // TODO(tarcieri): configurable title
        let title = name.to_string().to_title_case();

        // TODO(tarcieri): configurable description
        let description = title.clone();

        // TODO(tarcieri): configurable edition
        let edition = properties::rust::Edition::Rust2018;

        let patch_crates_io = self.patch_crates_io.clone();

        // TODO(tarcieri): configurable application type
        let application_type =
            properties::name::Type::from_snake_case(app_name.clone() + "_application");

        // TODO(tarcieri): configurable command type
        let command_type = properties::name::Type::from_snake_case(app_name.clone() + "_command");

        // TODO(tarcieri): configurable config type
        let config_type = properties::name::Type::from_snake_case(app_name.clone() + "_config");

        let properties = Properties {
            abscissa,
            name,
            title,
            description,
            authors: vec![],
            version: "0.1.0".parse().unwrap(),
            edition,
            patch_crates_io,
            application_type,
            command_type,
            config_type,
        };

        Ok(properties)
    }
}

/// Print a fatal error message and exit
pub fn fatal_error(err: Error) -> ! {
    status_err!("{}", err);
    process::exit(1)
}
