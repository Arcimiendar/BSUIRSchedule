use std::{error, fmt};
use std::fmt::{Formatter};
use reqwest;
use serde::de::DeserializeOwned;
use serde_json::from_str;
use crate::types::announcement::Announcement;
use crate::types::auditory::Auditory;
use crate::types::department::Department;
use crate::types::faculty::Faculty;
use crate::types::last_update::LastUpdate;
use crate::types::query_params::QueryParams;
use crate::types::specialities::Speciality;
use crate::types::employee::Employee;


#[derive(Debug, Clone)]
struct FetcherError;

impl fmt::Display for FetcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong")
    }
}


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


fn fetch(url: &str) -> Result<String> {
    match reqwest::blocking::get(url) {
        Ok(response) => {
            match response.text() {
                Ok(text) => Ok(text),
                Err(e) => Err(e.into())
            }
        },
        Err(e) => Err(e.into())
    }
}


fn fetch_and_deserialize<T: DeserializeOwned>(url: &str) -> Result<T> {
    match fetch(url) {
        Ok(text) => {
            match from_str::<T>(text.as_str()) {
                Ok(response) => Ok(response),
                Err(e) => Err(e.into())
            }
        },
        Err(e) => Err(e)
    }
}


fn get_week_number() -> Result<u32> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/schedule/current-week")
}


fn get_auditories() -> Result<Vec<Auditory>> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/auditories")
}


fn get_last_update<T: QueryParams>(param: T) -> Result<LastUpdate> {
    let url = format!(
        "https://iis.bsuir.by/api/v1/last-update-date/{}", param.get_query_params()
    );
    fetch_and_deserialize(url.as_str())
}


fn get_announcements<T: QueryParams>(param: T) -> Result<Vec<Announcement>> {
    let url = format!(
        "https://iis.bsuir.by/api/v1/announcements/{}", param.get_query_params()
    );
    fetch_and_deserialize(url.as_str())
}


fn get_specielities() -> Result<Vec<Speciality>> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/specialities")
}


fn get_departments() -> Result<Vec<Department>> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/departments")
}


fn get_faculties() -> Result<Vec<Faculty>> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/faculties")
}


fn get_employees() -> Result<Vec<Employee>> {
    fetch_and_deserialize("https://iis.bsuir.by/api/v1/employees/all")
}


#[cfg(test)]
mod tests {
    use crate::service::fetcher::{
        get_announcements, get_auditories, get_departments, get_employees,
        get_faculties, get_last_update, get_specielities, get_week_number
    };
    use crate::types::announcement::AnnouncementsOfEmployee;
    use crate::types::last_update::LastUpdateByGroupNumber;

    #[test]
    fn get_week_number_works() {
        let res = get_week_number();
        assert!(res.is_ok());
    }

    #[test]
    fn get_auditories_works() {
        let res = get_auditories();
        assert!(res.is_ok())
    }

    #[test]
    fn get_last_update_works() {
        let res = get_last_update(
            LastUpdateByGroupNumber { group_number: "155841".to_string() }
        );
        assert!(res.is_ok());
    }

    #[test]
    fn get_announcements_works() {
        let res = get_announcements(AnnouncementsOfEmployee {
            employee_url_id: "s-nesterenkov".to_string()
        });
        assert!(res.is_ok());
    }

    #[test]
    fn get_specialities_works() {
        let res = get_specielities();
        assert!(res.is_ok());
    }

    #[test]
    fn get_departments_works() {
        let res = get_departments();
        assert!(res.is_ok());
    }

    #[test]
    fn get_faculties_works() {
        let res = get_faculties();
        assert!(res.is_ok());
    }

    #[test]
    fn get_employees_works() {
        let res = get_employees();
        assert!(res.is_ok());
    }
}