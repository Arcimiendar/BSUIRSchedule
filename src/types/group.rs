use serde;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub faculty_id: u32,
    pub faculty_name: String,
    pub speciality_department_education_form_id: Option<u32>,
    pub speciality_name: String,
    pub course: Option<u32>,
    pub calendar_id: Option<String>
}