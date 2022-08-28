mod config;
use config::Config;

mod sections;
use sections::Sections;

mod templates;
use templates::Templates;

mod types;
use types::{Referee, Referrer};

use std::io::{stdout, Write};

use anyhow::Context;

#[derive(Default)]
struct Conclusion;

fn main() -> anyhow::Result<()> {
    let config = Config::load()?;
    let templates = Templates::init()?;

    let rendered = config.sections.render(&config, &templates)?;

    stdout().write_all(rendered.as_bytes())
        .context("failed to output rendered")
}
