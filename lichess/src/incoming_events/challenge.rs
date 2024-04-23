use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    pub id: String,
    pub url: String,
    pub color: String,
    pub time_control: TimeControl,
    pub variant: Variant,
    pub challenger: User,
    pub dest_user: User,
    pub perf: Perf,
    pub rated: bool,
    pub speed: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum TimeControl {
    Unlimited,
    Standard,
    Clock {
        increment: i32,
        limit: i32,
        show: String,
    },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub key: String,
    pub name: String,
    pub short: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    //pub online: bool,
    pub provisional: bool,
    pub rating: i32,
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Perf {
    pub icon: String,
    pub name: String,
}
