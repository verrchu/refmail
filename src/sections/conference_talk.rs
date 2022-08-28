use super::Section;
use crate::{Config, Templates};

use anyhow::Context;
use chrono::{Datelike, Month, NaiveDate};
use num_traits::cast::FromPrimitive;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct ConferenceTalk {
    conference_name: String,
    organizers: Option<Vec<String>>,
    link: Option<String>,

    /// Name of the talk
    talk_name: String,
    /// Link to the talk slides
    slides: Option<String>,

    /// Conference field (e.g Electrical Engineering)
    field: String,
    /// Conference location
    location: String,
    /// Conference date. Even if conference spanned over several days
    /// it doesn't matter as we will most likely only use year and month
    date: NaiveDate,
}

impl Section for ConferenceTalk {
    const TEMPLATE: &'static str = "conference_talk.template";

    fn render(&self, config: &Config, templates: &Templates) -> anyhow::Result<String> {
        use serde_json::json;
        use tera::Context;

        let context = Context::from_serialize(json!({
            "section": self,
            "config": config,
            "time": {
                "year": self.date.year(),
                "month": Month::from_u32(self.date.month()).unwrap(),
            }
        }))
        .context("failed to extract template context")?;

        templates
            .render(Self::TEMPLATE, &context)
            .context("failed to render template")
            .map(|rendered| {
                rendered
                    .lines()
                    .filter(|line| !line.is_empty())
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Config;

    use pretty_assertions::assert_eq;
    use test_case::test_case;

    #[test_case(ConferenceTalk {
        conference_name: "Annual Smart Conf".into(),
        organizers: Some(vec!["John Davis".into(), "Peter Ball".into()]),
        link: Some("www.conf.com".into()),

        talk_name: "How To Become Smarter".into(),
        slides: Some("www.slides.com".into()),

        field: "Smart technologies".into(),
        location: "Minsk, Belarus".into(),

        date: NaiveDate::from_ymd(2022, 10, 10)
    }, vec![
        r#"Dmitry participated in the "Annual Smart Conf" conference"#,
        r#"in the field of Smart technologies held in Minsk, Belarus"#,
        r#"in October 2022 organized by John Davis, Peter Ball"#
    ].join(" "); "full")]
    fn test_render(entity: ConferenceTalk, rendered: String) {
        let templates = Templates::init().unwrap();
        let config = Config::load_from(std::env::var("CONFIG").unwrap()).unwrap();

        assert_eq!(entity.render(&config, &templates).unwrap(), rendered);
    }
}
