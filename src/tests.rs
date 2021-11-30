use std::collections::HashMap;

use super::rocket;
use rocket::http::ContentType;
use rocket::local::blocking::Client;

use crate::{KhaosRequest, TestResp};

#[test]
fn handler() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");

    let req = KhaosRequest {
        contract: "",
        callback: "",
        destination: "",
        destination_query: HashMap::new(),
        destination_parse_response: vec![""],
        require: "https://chain.so/api/v2/get_info/BTC",
        require_query: vec![""],
        require_parse_response: "status",
        secret_location: crate::SecretLocation::QueryParam,
        secret_key: "user_token",
    };

    let serialized = serde_json::to_string(&req).unwrap();

    let response: TestResp = match client
        .post("/")
        .header(ContentType::JSON)
        .body(serialized)
        .dispatch()
        .into_json()
    {
        Some(thing) => thing,
        None => TestResp {
            val: "".to_string(),
        },
    };

    let formatted_val = format!("{}", response.val).replace("\"", "");

    assert_eq!(formatted_val, "success");
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
