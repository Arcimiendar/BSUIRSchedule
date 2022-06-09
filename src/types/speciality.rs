use serde;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EducationForm {
    pub id: u32,
    pub name: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Speciality {
    pub id: u32,
    pub name: String,
    pub abbrev: String,
    pub education_form: EducationForm,
    pub faculty_id: u32,
    pub code: String
}
