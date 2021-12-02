use std::collections::HashMap;

use super::*;
use rocket::http::ContentType;
use rocket::local::blocking::Client;

use crate::KhaosRequest;
use httpmock::prelude::*;
use serde_json::json;
// use tokio::test;

#[test]
fn it_handles_khaos_request() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    // Start a lightweight mock server.
    let dev_server: httpmock::MockServer = MockServer::start();
    let endpoint: httpmock::MockServer = MockServer::start();

    let dev_server_mock = dev_server.mock(|when, then| {
        when.method(GET).path("/fetch-secret");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"user_token": "abc123"}));
    });
    let json_body = json!({"nested": {
        "test": 123
    }});
    let endpoint_mock = endpoint.mock(|when, then| {
        when.method(GET).path("/query-endpoint");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json_body.clone());
    });

    let dev_server_address = format!("http://{}/fetch-secret", dev_server_mock.server_address());
    let endpoint_server_address =
        format!("http://{}/query-endpoint", endpoint_mock.server_address());

    let req = KhaosRequest {
        contract: "",
        callback: "",
        destination: &endpoint_server_address,
        destination_query: HashMap::new(),
        destination_parse_response: vec![""],
        require: &dev_server_address,
        require_query: vec![""],
        require_parse_response: "user_token",
        secret_location: crate::SecretLocation::QueryParam,
        secret_key: "user_token",
    };

    let serialized = serde_json::to_string(&req).unwrap();

    let response = client
        .post("/")
        .header(ContentType::JSON)
        .body(serialized)
        .dispatch();

    dev_server_mock.assert();
    endpoint_mock.assert();
    assert_eq!(response.into_string().unwrap(), json_body.to_string());
}

#[tokio::test]
async fn it_retreives_key() {
    let mock_server: httpmock::MockServer = MockServer::start();

    // Create a mock on the server.
    let dev_server_mock = mock_server.mock(|when, then| {
        when.method(GET).path("/fetch-secret");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({"user_token": "abc123"}));
    });

    let mock_server_address = format!("http://{}/fetch-secret", dev_server_mock.server_address());

    let key = match retreive_key(&mock_server_address, "user_token").await {
        Ok(val) => val,
        Err(error) => panic!("{}", error.to_string()),
    };

    dev_server_mock.assert();
    let formatted_key = format!("{}", key).replace("\"", "");
    assert_eq!(formatted_key, "abc123");
}

#[test]
fn it_adds_query_key() {
    let secret = "abcd123";
    let test_param_key = String::from("dev");
    let test_param_value = String::from("testing");
    let query_params = HashMap::from([(test_param_key.clone(), test_param_value.clone())]);
    let secret_location = SecretLocation::QueryParam;
    let secret_param_key = "token";

    let (params, _) = match prepare_query(secret, query_params, secret_location, secret_param_key) {
        Ok(val) => val,
        Err(error) => panic!("{}", error.to_string()),
    };

    assert_eq!(params.len(), 2);
    let secret_param_key = match params.get(secret_param_key) {
        Some(val) => val,
        None => panic!("No key found"),
    };
    assert_eq!(secret_param_key, &secret.to_string());

    let get_param_key = match params.get(&test_param_key) {
        Some(val) => val,
        None => panic!("No key found"),
    };
    assert_eq!(get_param_key, &test_param_value);
}

#[test]
fn it_adds_header_key() {
    let secret = "abcd123";
    let query_params = HashMap::new();
    let secret_location = SecretLocation::Header;
    let header_secret_key = "X-API-KEY";

    let (_, headers) = match prepare_query(secret, query_params, secret_location, header_secret_key)
    {
        Ok(val) => val,
        Err(error) => panic!("{}", error.to_string()),
    };

    assert_eq!(headers.len(), 1);
    let get_key = match headers.get(header_secret_key) {
        Some(val) => val,
        None => panic!("No key found"),
    };
    assert_eq!(get_key, &secret.to_string());
}

// #[test]
// fn hello_mir() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     let response = client.get("/hello/%D0%BC%D0%B8%D1%80").dispatch();
//     assert_eq!(response.into_string(), Some("Привет, мир!".into()));
// }

// #[test]
// fn wave() {
//     let client = Client::tracked(super::rocket()).unwrap();
//     for &(name, age) in &[("Bob%20Smith", 22), ("Michael", 80), ("A", 0), ("a", 127)] {
//         let uri = format!("/wave/{}/{}", name, age);
//         let real_name = RawStr::new(name).percent_decode_lossy();
//         let expected = format!("👋 Hello, {} year old named {}!", age, real_name);
//         let response = client.get(uri).dispatch();
//         assert_eq!(response.into_string().unwrap(), expected);

//         for bad_age in &["1000", "-1", "bird", "?"] {
//             let bad_uri = format!("/wave/{}/{}", name, bad_age);
//             let response = client.get(bad_uri).dispatch();
//             assert_eq!(response.status(), Status::NotFound);
//         }
//     }
// }
