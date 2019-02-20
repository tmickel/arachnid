extern crate reqwest;
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde_json::json;
use std::process::Command;
use std::collections::HashMap;

use std::{thread, time};

#[derive(Deserialize, Debug)]
struct Ip {
    value: V,
}

#[derive(Deserialize, Debug)]
struct V {
    sessionId: String,
}

#[derive(Deserialize, Debug)]
struct Source {
    value: String,
}

#[derive(Deserialize, Debug)]
struct Element {
    value: serde_json::Value
}

fn main() -> Result<(), Box<std::error::Error>> {
let mut child = Command::new("/usr/local/bin/geckodriver")
        .arg("--log")
        .arg("debug")
        .spawn()
        .expect("failed to execute geckodriver");
    let ten_millis = time::Duration::from_millis(100);
    thread::sleep(ten_millis);

    let session_request = json!({
        "capabilities": {
            "alwaysMatch": {
                "moz:firefoxOptions": {
                    // "args": ["-headless"],
                    "prefs": {
                        "network.http.sendRefererHeader": 0, // Never send referer.
                        "permissions.default.image": 2, // Block all images.
                        "dom.storage.enabled": false // Block localStorage, etc.
                    }
                }
            }
        }
    });

    let client = reqwest::Client::new();
    let mut response = client
        .post("http://localhost:4444/session")
        .json(&session_request)
        .send()?;
    if response.status().is_success() {

        let json: Ip = response.json()?;
        let session_id = json.value.sessionId;
        let navigate_request = json!({
            "url": "https://recurse.com",
        });

        client
            .post(&format!("http://localhost:4444/session/{}/url", session_id))
            .json(&navigate_request)
            .send()?;

        let element_request = json!({
            "using": "tag name",
            "value": "html"
        });
        let mut page_text_response = client
            .post(&format!(
                "http://localhost:4444/session/{}/element",
                session_id
            ))
            .json(&element_request)
            .send()?;
        let source_response_json: Element = page_text_response.json()?;
        let o : HashMap<String, String> = serde_json::from_value(source_response_json.value).unwrap();
        let element_id = o.values().next().unwrap(); // TODO: values?
        let mut text_content = client.get(&format!(
                "http://localhost:4444/session/{}/element/{}/text",
                session_id,
                element_id
            )).send()?;
        let jsonwow: Source = text_content.json()?;
        println!("{:?}", jsonwow);

        client
        .delete(&format!("http://localhost:4444/session/{}", session_id))
        .send()?;
    }
    child.kill();
    Ok(())
}
