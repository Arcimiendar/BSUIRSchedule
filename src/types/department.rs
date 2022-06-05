use serde;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Department {
    pub id: u32,
    pub name: String,
    pub abbrev: String
}