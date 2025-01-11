use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize)]
pub struct QueryWeather {
    pub city: String
}

#[derive(Deserialize, Serialize, Default)]
pub struct WeatherModel {
    temperature: String,
    wind: String,
    description: String,
    forecast: Vec<WeatherForecastModel>
}

#[derive(Deserialize, Serialize, Default)]
pub  struct WeatherForecastModel {
    day: String,
    temperature: String,
    wind: String
}