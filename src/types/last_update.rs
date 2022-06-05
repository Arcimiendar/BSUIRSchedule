use serde;
use serde::Deserialize;
use crate::types::query_params::QueryParams;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastUpdate {
    last_update_date: String
}


pub struct LastUpdateByGroupNumber {
    pub(crate) group_number: String
}

impl QueryParams for LastUpdateByGroupNumber {
    fn get_query_params(&self) -> String {
        format!("student-group?groupNumber={}", self.group_number)
    }
}

pub struct LastUpdateByGroupId {
    group_id: u32
}

impl QueryParams for LastUpdateByGroupId {
    fn get_query_params(&self) -> String {
        format!("student-group?id={}", self.group_id)
    }
}

pub struct LastUpdateByEmployeeUrlId {
    employee_id: String
}

impl QueryParams for LastUpdateByEmployeeUrlId {
    fn get_query_params(&self) -> String {
        format!("employee?url-id={}", self.employee_id)
    }
}

pub struct LastUpdateByEmployeeId {
    employee_id: u32
}

impl QueryParams for LastUpdateByEmployeeId {
    fn get_query_params(&self) -> String {
        format!("employee?id={}", self.employee_id)
    }
}

