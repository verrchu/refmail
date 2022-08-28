use crate::{Config, Templates};

mod conference_talk;
use conference_talk::ConferenceTalk;
mod hackathon;
use hackathon::Hackathon;

use serde::{Deserialize, Serialize};

pub trait Section {
    const TEMPLATE: &'static str;

    fn render(&self, config: &Config, templates: &Templates) -> anyhow::Result<String>;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sections {
    // referrer_background: Background,
    // acquaintance: Acquintance,
    // referee_background: Background<Referee>,
    // referee_experience: Experience<Referee>,
    // entrepreneurship: Option<Entrepreneurship>,
    // volunteering: Option<Volunteering>,
    // olympiads: Option<Olympiads>,
    // publications: Option<Publications>,
    // expert_roles: Option<ExpertRoles>,

    // open_source_contributions: Option<Vec<OpenSourceContribution>>,
    // hackathons: Option<Vec<Hackathon>>,
    conference_talks: Option<Vec<ConferenceTalk>>,
}

impl Sections {
    pub fn render(&self, config: &Config, templates: &Templates) -> anyhow::Result<String> {
        let entries = self
            .conference_talks
            .as_ref()
            .map(|section| {
                section
                    .into_iter()
                    .map(|entry| entry.render(config, templates))
                    .collect::<anyhow::Result<Vec<String>>>()
            })
            .transpose()?;

        Ok(entries.map(|entries| entries.join(" ")).unwrap_or_default())
    }
}

struct Background;
struct Acquintance;

struct Entrepreneurship;
struct Volunteering;

enum Publications {
    By,
    About,
}

struct OpenSourceContribution;

struct ExpertRoles;

struct OpenSourceContributions;
struct Olympiads;
