use serde;
use serde::Deserialize;



#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuditoryType {
    pub id: u32,
    pub name: String,
    pub abbrev: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BuildingNumber {
    pub id: u32,
    pub name: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AuditoryDepartment {
    pub id_department: u32,
    pub abbrev:  String,
    pub name: String,
    pub name_and_abbrev: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Auditory {
    pub id: u32,
    pub name: String,
    pub note: Option<String>,
    pub capacity: Option<u32>,
    pub auditory_type: AuditoryType,
    pub building_number: BuildingNumber,
    pub department: Option<AuditoryDepartment>
}