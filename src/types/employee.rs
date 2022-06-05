use serde;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Employee {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub middle_name: String,
    pub degree: String,
    pub rank: Option<String>,
    pub photo_link: String,
    pub calendar_id: String,
    pub academic_department: Vec<String>,
    pub url_id: String,
    pub fio: String
}
