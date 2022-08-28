use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Messeger {
    Telegram(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Gender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Referrer {
    first_name: String,
    last_name: String,
    email: String,

    linkedin: Option<Url>,
    messeger: Option<Messeger>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Referee {
    first_name: String,
    last_name: String,
    email: String,

    linkedin: Option<Url>,
    messeger: Option<Messeger>,
}
