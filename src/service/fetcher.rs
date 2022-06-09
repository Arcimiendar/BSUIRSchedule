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
use crate::types::speciality::Speciality;
use crate::types::employee::Employee;
use crate::types::group::Group;


#[derive(Debug, Clone)]
struct FetcherError;

impl fmt::Display for FetcherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "something went wrong")
    }
}


type Result<T> = std::result::Result<T, Box<dyn error::Error>>;


pub struct Fetcher {
    base_url: String
}

impl Fetcher {
    pub fn new(url: String) -> Fetcher {
        Fetcher { base_url: url }
    }

    fn fetch(url: &String) -> Result<String> {
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

    fn fetch_and_deserialize<T: DeserializeOwned>(url: &String) -> Result<T> {
        match Fetcher::fetch(url) {
            Ok(text) => {
                match from_str::<T>(text.as_str()) {
                    Ok(response) => Ok(response),
                    Err(e) => Err(e.into())
                }
            },
            Err(e) => Err(e)
        }
    }

    fn url(&self, url: &str) -> String {
        self.base_url.to_string() + url
    }

    pub fn get_week_number(&self) -> Result<u32> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/schedule/current-week"))
    }

    pub fn get_auditories(&self) -> Result<Vec<Auditory>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/auditories"))
    }

    pub fn get_last_update<T: QueryParams>(&self, param: T) -> Result<LastUpdate> {
        let url = format!(
            "/api/v1/last-update-date/{}", param.get_query_params()
        );
        Fetcher::fetch_and_deserialize(&self.url(&url.as_str()))
    }

    pub fn get_announcements<T: QueryParams>(&self, param: T) -> Result<Vec<Announcement>> {
        let url = format!(
            "/api/v1/announcements/{}", param.get_query_params()
        );
        Fetcher::fetch_and_deserialize(&self.url(&url.as_str()))
    }

    pub fn get_specielities(&self) -> Result<Vec<Speciality>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/specialities"))
    }

    pub fn get_departments(&self) -> Result<Vec<Department>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/departments"))
    }

    pub fn get_faculties(&self) -> Result<Vec<Faculty>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/faculties"))
    }

    pub fn get_employees(&self) -> Result<Vec<Employee>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/employees/all"))
    }

    pub fn get_groups(&self) -> Result<Vec<Group>> {
        Fetcher::fetch_and_deserialize(&self.url("/api/v1/student-groups"))
    }
}


#[cfg(test)]
mod tests {
    use crate::service::fetcher::Fetcher;
    use crate::types::announcement::AnnouncementsOfEmployee;
    use crate::types::last_update::LastUpdateByGroupNumber;

    #[test]
    fn get_week_number_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_week_number();
        assert!(res.is_ok());
    }

    #[test]
    fn get_auditories_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_auditories();
        assert!(res.is_ok())
    }

    #[test]
    fn get_last_update_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_last_update(
            LastUpdateByGroupNumber { group_number: "155841".to_string() }
        );
        assert!(res.is_ok());
    }

    #[test]
    fn get_announcements_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_announcements(AnnouncementsOfEmployee {
            employee_url_id: "s-nesterenkov".to_string()
        });
        assert!(res.is_ok());
    }

    #[test]
    fn get_specialities_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_specielities();
        assert!(res.is_ok());
    }

    #[test]
    fn get_departments_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_departments();
        assert!(res.is_ok());
    }

    #[test]
    fn get_faculties_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_faculties();
        assert!(res.is_ok());
    }

    #[test]
    fn get_employees_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_employees();
        assert!(res.is_ok());
    }

    #[test]
    fn get_groups_works() {
        let fetcher = Fetcher::new("https://iis.bsuir.by".to_string());
        let res = fetcher.get_groups();
        assert!(res.is_ok());
    }
}