use std::{env, ops::Deref};

use anyhow::Context;
use tera::Tera;

pub struct Templates(Tera);

impl Templates {
    pub fn init() -> anyhow::Result<Self> {
        let dir = format!("{}/src/templates/*.template", env!("CARGO_MANIFEST_DIR"));
        let inner = Tera::new(&dir).context("failed to load section templates")?;

        Ok(Self(inner))
    }
}

impl Deref for Templates {
    type Target = Tera;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
