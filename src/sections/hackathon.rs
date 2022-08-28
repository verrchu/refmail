use crate::types;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct Hackathon {
    hackathon_name: String,
    field: String,
    hlink: Option<Url>,

    project_description: String,
    plink: Option<Url>,

    // FIXME
    result: String,

    location: Option<String>,
    date: NaiveDate,
}
