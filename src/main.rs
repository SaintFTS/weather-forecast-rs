use clap::Parser;
use weather::defines::functions::*;
use weather::defines::structs::*;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), String> {
    let args = Cli::parse();

    let country_code: String = (!args.country_code.is_empty()).then(
            || format!("&countryCode={}", args.country_code.to_uppercase())
        ).unwrap_or_default();

    let geo_link = format!("https://geocoding-api.open-meteo.com/v1/search?name={}{}&count=1&language=en&format=json", args.city_name, country_code);

    let cities = get_class_from_response::<VecCities>(geo_link.as_str())
        .await
        .map_err(|e| match e {
            ErrorCodes::NoResponse(_) => 
                format!("No response from open-meteo.com. Check your internet connection"),
            ErrorCodes::FailedParse(_) | ErrorCodes::FailedRead(_) => 
                format!("No citites '{}' found in '{}'", args.city_name, args.country_code),
        })?;

    let city = cities.results.first().unwrap();

    let forecast_link = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature", city.latitude, city.longitude);

    let w_result = get_class_from_response::<Weather>(forecast_link.as_str()).await;
    let weather = w_result.map_err(|e| match e {
            ErrorCodes::NoResponse(_) => 
                format!("No response from open-meteo.com. Check your internet connection"),
            ErrorCodes::FailedParse(_) | ErrorCodes::FailedRead(_) => 
                format!("No citites '{}' found in '{}'", args.city_name, args.country_code),
        })?.current;
    println!("\
       City: {}, {}\
       \nTemperature: {}°C\
       \nHumidity: {}%\
       \nApparent Temp: {}°C", 
       city.name.cyan(), city.country.cyan(),
       weather.temp, weather.humidity, weather.apparent_temp);
    Ok(())

}
