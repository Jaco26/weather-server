mod noaa_api;

use std::fs;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use actix_web::{
    error,
    middleware,
    web,
    App,
    Error,
    Result,
    HttpRequest,
    HttpResponse,
    HttpServer
};
use serde::{Deserialize, Serialize};
use serde_json::json;



#[derive(Deserialize, Serialize, Clone)]
struct ZipCodes {
    data: HashMap<String, (String, String)>
}


fn lookup_coords(zips: web::Data<ZipCodes>, zip_code: &str) -> (String, String) {
    let zips = zips.into_inner();
    zips.data.get(zip_code).unwrap().clone()
}


async fn get_coords(zips: web::Data<ZipCodes>, web::Path((zip,)): web::Path<(String,)>) -> HttpResponse {
    HttpResponse::Ok().json(lookup_coords(zips, &zip))
}


async fn get_forecast(zips: web::Data<ZipCodes>, web::Path((zip,)): web::Path<(String,)>) -> Result<HttpResponse> {
    let (lat, lng) = lookup_coords(zips, &zip);

    let points = noaa_api::get_points(&lat, &lng)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let forecast = noaa_api::get_forecast(points)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Ok().json(forecast))
}

async fn get_hourly_forecast(zips: web::Data<ZipCodes>, web::Path((zip,)): web::Path<(String,)>) -> Result<HttpResponse> {
    let (lat, lng) = lookup_coords(zips, &zip);
    
    let points = noaa_api::get_points(&lat, &lng)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    let forecast = noaa_api::get_hourly_forecast(points)
        .await
        .map_err(|err| error::ErrorInternalServerError(err))?;

    Ok(HttpResponse::Ok().json(forecast))
}


#[derive(Deserialize)]
struct ZipCode {
    zip: String,
    lat: String,
    lng: String
}


fn load_zip_codes() -> ZipCodes {
    let zip_codes_path = env::var("ZIP_CODES_PATH").unwrap();
    let zip_codes_path = Path::new(&zip_codes_path);
    let zip_codes = fs::read_to_string(zip_codes_path).unwrap();
    let zip_codes: Vec<ZipCode> = serde_json::from_str(&zip_codes).unwrap();
    zip_codes
        .iter()
        .fold(ZipCodes { data: HashMap::new() }, |mut acc, x| {
            acc.data.insert(x.zip.clone(), (x.lat.clone(), x.lng.clone()));
            acc
        })
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let zip_codes = load_zip_codes();

    HttpServer::new(move || {
        App::new()
            .data(zip_codes.clone())
            .route("/coords/{zip}", web::get().to(get_coords))
            .route("/forecast/{zip}", web::get().to(get_forecast))
            .route("/forecast/hourly/{zip}", web::get().to(get_hourly_forecast))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
