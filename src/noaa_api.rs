#![allow(non_snake_case)]

use actix_web::client::{
    Client,
    JsonPayloadError,
};
use serde::{Deserialize, Serialize};

const API: &str = "https://api.weather.gov";

#[derive(Deserialize, Serialize, Debug)]
pub struct PointsResponseProperties {
    gridId: String,
    gridX: usize,
    gridY: usize,
}


async fn noaa_get<T>(url: String) -> Result<T, JsonPayloadError>
where
    for<'de> T: Deserialize<'de>
{
    Ok(
        Client::default()
            .get(url)
            .set_header("User-Agent", "Jacob's weather cli")
            .send()
            .await
            .unwrap()
            .json::<T>()
            .await?
    )
}


#[derive(Deserialize, Serialize, Debug)]
pub struct PointsResponse {
    properties: PointsResponseProperties
}


pub async fn get_points(lat: &str, lng: &str) -> Result<PointsResponse, JsonPayloadError> {
    noaa_get(
        format!(
            "{}/points/{},{}", API, lat, lng
        )
    ).await
}



#[derive(Deserialize, Serialize)]
pub struct ForecastResponse {

}

pub async fn get_forecast(points: PointsResponse) -> Result<serde_json::Value, JsonPayloadError> {
    let PointsResponseProperties { gridId, gridX, gridY } = points.properties;
    noaa_get(
        format!(
            "{}/gridpoints/{}/{},{}/forecast", API, gridId, gridX, gridY
        )
    ).await
}



#[derive(Deserialize, Serialize)]
pub struct HourlyForecastResponse {

}

pub async fn get_hourly_forecast(points: PointsResponse) -> Result<HourlyForecastResponse, JsonPayloadError>  {
    let PointsResponseProperties { gridId, gridX, gridY } = points.properties;
    noaa_get(
        format!(
            "{}/gridpoints/{}/{},{}/forecast/hourly", API, gridId, gridX, gridY
        )
    ).await
}