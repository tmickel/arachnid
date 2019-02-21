/**
 * Interface that makes requests to geckodriver.
 * Reference for writing new functions:
 * https://www.w3.org/TR/webdriver1/
 */
extern crate reqwest;
extern crate url;
extern crate serde;
extern crate serde_json;

use self::serde_json::json;
use self::serde::Deserialize;
use std::collections::HashMap;
use config;

/**
 * All the data needed to drive the functions below.
 * Instantiate one of these first!
 */
pub struct WebDriverContext {
    pub client: reqwest::Client,
    pub config: config::Config,
    pub session_id: Option<String>,
}

/**
 * Helpers: create URLs based on configs.
 */
fn make_webdriver_url(context: &WebDriverContext, path: &str) -> String {
    format!(
        "http://{}:{}{}",
        context.config.gecko_driver_host,
        context.config.gecko_driver_port,
        path
    )
}

fn make_webdriver_session_url(context: &WebDriverContext, session_id: &str, path: &str)
    -> String {
    format!(
        "http://{}:{}/session/{}{}",
        context.config.gecko_driver_host,
        context.config.gecko_driver_port,
        session_id,
        path
    )
}

/**
 * Create a new WebDriver session, and attach it to the provided context.
 * https://www.w3.org/TR/webdriver1/#new-session
 */
pub fn create_session(context: &mut WebDriverContext) {
    let session_request = json!({
        "capabilities":
            context.config.gecko_driver_capabilities
    });

    let mut response = context.client
        .post(&make_webdriver_url(context, "/session"))
        .json(&session_request)
        .send()
        .expect("Couldn't create WebDriver session.");
    assert!(response.status().is_success(), "Couldn't create WebDriver session");
    
    let inner_response : CreateSessionResponse = response.json()
        .expect("Received invalid response from WebDriver");
    context.session_id = Some(inner_response.value.sessionId);
}

#[derive(Deserialize, Debug)]
struct CreateSessionResponse {
    value: CreateSessionResponseWithId,
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct CreateSessionResponseWithId {
    sessionId: String,
}


/**
 * Delete the WebDriver session attached to the provided context.
 * https://www.w3.org/TR/webdriver1/#delete-session
 */
pub fn delete_session(context: &WebDriverContext) {
    let session_id = context.session_id.clone()
        .expect("Cannot run without an established session.");
    context.client
        .delete(&make_webdriver_session_url(context, &session_id, ""))
        .send()
        .expect("Couldn't destroy WebDriver session.");
}

/**
 * Navigate the WebDriver session to the given url.
 * https://www.w3.org/TR/webdriver1/#navigate-to
 */
pub fn navigate(context: &WebDriverContext, url: url::Url) {
    let navigate_request = json!({
        "url": url.as_str(),
    });

    let session_id = context.session_id.clone()
        .expect("Cannot run without an established session.");
    context.client
        .post(&make_webdriver_session_url(context, &session_id, "/url"))
        .json(&navigate_request)
        .send()
        .expect("Couldn't navigate WebDriver -- is geckodriver still running?");
}

/**
 * Strategies used to select elements from the active DOM.
 * https://www.w3.org/TR/webdriver1/#locator-strategies
 */
pub enum WebDriverLocatorStrategy {
    Css,
    LinkText,
    PartialLinkText,
    TagName,
    XPath,
}

impl WebDriverLocatorStrategy {
    pub fn as_str(&self) -> &str {
        match self {
            &WebDriverLocatorStrategy::Css => "css selector",
            &WebDriverLocatorStrategy::LinkText => "link text",
            &WebDriverLocatorStrategy::PartialLinkText => "partial link text",
            &WebDriverLocatorStrategy::TagName => "tag name",
            &WebDriverLocatorStrategy::XPath => "xpath",
        }
    }
}


/**
 * Represents a WebDriver element ID.
 */
type WebDriverElement = String;

/**
 * Element ID from WebDriver.
 */
pub fn get_element(context: &WebDriverContext, using: WebDriverLocatorStrategy, value: &str)
    -> WebDriverElement {
    let session_id = context.session_id.clone()
        .expect("Cannot run without an established session.");
    let element_request = json!({
        "using": using.as_str(),
        "value": value
    });
    let mut element_response = context.client
        .post(&make_webdriver_session_url(context, &session_id, "/element"))
        .json(&element_request)
        .send()
        .expect("Couldn't select top-level element.");
    let element_response_json: ElementResponse = element_response.json()
        .expect("Invalid response from WebDriver");
    let element_map : HashMap<String, String> =
        serde_json::from_value(element_response_json.value)
        .expect("WebDriver did not return elements");
    let element_id = element_map.values().next()
        .expect("WebDriver did not return elements");
    element_id.clone() as WebDriverElement
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct ElementResponse {
    value: serde_json::Value
}


/**
 * Return the visible text of the given element.
 * https://www.w3.org/TR/webdriver1/#get-element-text
 */
pub fn get_text(context: &WebDriverContext, element: WebDriverElement) -> String {
    let session_id = context.session_id.clone()
        .expect("Cannot run without an established session.");

    let mut text_response = context.client.get(
        &make_webdriver_session_url(
            context, &session_id, &format!("/element/{}/text", element))
        ).send()
        .expect("Couldn't get element text from WebDriver");
    let text_response_json: TextResponse = text_response.json()
        .expect("Couldn't parse element text from WebDriver");
    text_response_json.value
}

#[derive(Deserialize, Debug)]
struct TextResponse {
    value: String,
}
