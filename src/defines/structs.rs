#[derive(serde::Deserialize)]
pub struct VecCities {
    pub results: Vec<City>,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Weather {
    pub current: WeatherCurrent,
}
 

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct WeatherCurrent {
    #[serde(rename="temperature_2m")]
    pub temp: f32,
    #[serde(rename="relative_humidity_2m")]
    pub humidity: u8,
    #[serde(rename="apparent_temperature")]
    pub apparent_temp: f32  
}
#[derive(Clone, serde::Deserialize, std::fmt::Debug)]
pub struct City {
    pub name: String,
    pub country: String,
    pub longitude: f32,
    pub latitude: f32
}

#[derive(clap_derive::Parser)]
pub struct Cli {
    #[arg()]
    pub city_name: String,
    #[arg(default_value_t=String::new())]
    pub country_code: String,
    
}

#[derive(std::fmt::Debug)]
pub enum ErrorCodes {
    NoResponse(String),
    FailedRead(String),
    FailedParse(String)
}

impl std::error::Error for ErrorCodes{}

impl std::fmt::Display for ErrorCodes{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCodes::NoResponse(msg) => write!(f, "No response from the server: {}. Check your internet connection", msg),
            ErrorCodes::FailedRead(msg) => write!(f, "Failed to read response body: {}", msg),
            ErrorCodes::FailedParse(msg) => write!(f, "Couldn't parse the response {}", msg),
        }
    }

}
