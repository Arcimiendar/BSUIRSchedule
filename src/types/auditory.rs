use serde;
use serde::Deserialize;



#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AuditoryType {
    id: u32,
    name: String,
    abbrev: String,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BuildingNumber {
    id: u32,
    name: String
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Department {
    id_department: u32,
    abbrev:  String,
    name: String,
    name_and_abbrev: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Auditory {
    id: u32,
    name: String,
    note: Option<String>,
    capacity: Option<u32>,
    auditory_type: AuditoryType,
    building_number: BuildingNumber,
    department: Option<Department>
}