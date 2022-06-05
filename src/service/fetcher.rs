use std::{error, fmt};
use std::fmt::Formatter;
use reqwest;
use serde_json::from_str;
use crate::types::auditory::Auditory;


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


fn get_week_number() -> Result<u32> {
    match fetch("https://iis.bsuir.by/api/v1/schedule/current-week") {
        Ok(text) => {
            match text.parse::<u32>() {
                Ok(num) => Ok(num),
                Err(e) => Err(e.into())
            }
        },
        Err(e) => Err(e.into())
    }
}


fn get_auditories() -> Result<Vec<Auditory>> {
    match fetch("https://iis.bsuir.by/api/v1/auditories") {
        Ok(text) => {
            match from_str::<Vec<Auditory>>(text.as_str()) {
                Ok(auditories) => Ok(auditories),
                Err(e) => Err(e.into())
            }
        },
        Err(e) => Err(e)
    }
}




#[cfg(test)]
mod tests {
    use crate::service::fetcher::{get_auditories, get_week_number};

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
}