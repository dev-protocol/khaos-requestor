use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use rocket::{
    response::status::BadRequest,
    routes,
    serde::{json::Json, Deserialize, Serialize},
};
use std::{collections::HashMap, str::FromStr};

#[cfg(test)]
mod tests;

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize, Clone, Copy)]
enum SecretLocation {
    QueryParam,
    Header,
    None,
}

#[derive(Deserialize, Serialize)]
struct KhaosRequest<'r> {
    // The associated ethereum address
    contract: &'r str,
    // Associated solidity callback function
    callback: &'r str,
    // Url endpoint to query
    destination: &'r str,
    destination_query: HashMap<String, String>,
    destination_parse_response: Vec<&'r str>,
    // Should we set the secret in the headers or the URL?
    secret_location: SecretLocation,
    secret_key: &'r str,
    // Developer Server Url
    require: &'r str,
    require_query: Vec<&'r str>,
    require_parse_response: &'r str,
}

#[post("/", format = "application/json", data = "<req>")]
async fn handler(
    req: Json<KhaosRequest<'_>>,
) -> Result<Json<serde_json::Value>, BadRequest<String>> {
    // Fetch key from developer server
    let key = match retreive_key(req.require, req.require_parse_response).await {
        Ok(key) => key,
        Err(error) => return Err(BadRequest(Some(error))),
    };

    let client = reqwest::Client::new();

    let (query_params, headers) = match prepare_query(
        &key,
        req.destination_query.clone(),
        req.secret_location,
        req.secret_key,
    ) {
        Ok(res) => res,
        Err(error) => return Err(BadRequest(Some(error.to_string()))),
    };

    // Format URL with params
    let url = match Url::parse_with_params(req.destination, query_params) {
        Ok(url) => url,
        Err(error) => return Err(BadRequest(Some(error.to_string()))),
    };

    // Make external Query
    let query = match client.get(url).headers(headers).send().await {
        Ok(val) => val,
        Err(error) => return Err(BadRequest(Some(error.to_string()))),
    };

    // Parse returned JSON
    let json: serde_json::Value = match query.json().await {
        Ok(json) => json,
        Err(error) => return Err(BadRequest(Some(error.to_string()))),
    };

    Ok(Json(json))
}

fn prepare_query(
    key: &str,
    mut query_params: HashMap<String, String>,
    secret_location: SecretLocation,
    param_key: &str,
) -> Result<(HashMap<String, String>, HeaderMap), String> {
    let mut headers = HeaderMap::new();

    match secret_location {
        SecretLocation::QueryParam => {
            query_params.insert(param_key.to_string(), key.to_string());
        }
        SecretLocation::Header => {
            let header_val = match HeaderValue::from_str(&key) {
                Ok(val) => val,
                Err(error) => return Err(error.to_string()),
            };

            let header_name = match HeaderName::from_str(param_key) {
                Ok(val) => val,
                Err(error) => return Err(error.to_string()),
            };

            headers.insert(header_name, header_val);
        }
        SecretLocation::None => {
            println!("No secret needed for request");
        }
    };

    Ok((query_params, headers))
}

async fn retreive_key(
    dev_server_url: &str,
    require_parse_response: &str,
) -> Result<String, String> {
    let dev_res = match reqwest::get(dev_server_url).await {
        Ok(val) => val,
        Err(error) => return Err(error.to_string()),
    };

    let json: serde_json::Value = match dev_res.json().await {
        Ok(json) => json,
        Err(error) => return Err(error.to_string()),
    };

    let key: String = match json.get(&require_parse_response) {
        Some(val) => val.to_string(),
        None => return Err("Error fetching require_parse_response from json".to_string()),
    };

    Ok(key)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![handler])
}
