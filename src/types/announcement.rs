use serde;
use serde::Deserialize;
use crate::types::query_params::QueryParams;


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudentGroup {
    pub id: u32,
    pub name: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    pub id: u32,
    pub employee: String,
    pub content: String,
    pub date: String,
    pub employee_departments: Vec<String>,
    pub student_groups: Vec<StudentGroup>
}


pub struct AnnouncementsOfDepartment {
    pub department_id: u32
}

impl QueryParams for AnnouncementsOfDepartment {
    fn get_query_params(&self) -> String {
        format!("departments?id={}", self.department_id)
    }
}


pub struct AnnouncementsOfEmployee {
    pub employee_url_id: String
}

impl QueryParams for AnnouncementsOfEmployee {
    fn get_query_params(&self) -> String {
        format!("employees?url-id={}", self.employee_url_id)
    }
}