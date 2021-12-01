use std::collections::HashMap;

use super::*;
use rocket::http::ContentType;
use rocket::local::blocking::Client;

use crate::{KhaosRequest, TestResp};

// #[test]
// fn handler() {
//     let client = Client::tracked(rocket()).expect("valid rocket instance");

//     let req = KhaosRequest {
//         contract: "",
//         callback: "",
//         destination: "",
//         destination_query: HashMap::new(),
//         destination_parse_response: vec![""],
//         require: "https://chain.so/api/v2/get_info/BTC",
//         require_query: vec![""],
//         require_parse_response: "status",
//         secret_location: crate::SecretLocation::QueryParam,
//         secret_key: "user_token",
//     };

//     let serialized = serde_json::to_string(&req).unwrap();

//     let response: TestResp = match client
//         .post("/")
//         .header(ContentType::JSON)
//         .body(serialized)
//         .dispatch()
//         .into_json()
//     {
//         Some(thing) => thing,
//         None => TestResp {
//             val: "".to_string(),
//         },
//     };

//     let formatted_val = format!("{}", response.val).replace("\"", "");

//     assert_eq!(formatted_val, "success");
// }

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
