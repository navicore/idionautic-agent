use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{window, Headers, Request, RequestInit, Response};

#[wasm_bindgen]
pub async fn record_event(event_type: &str, target: &str, count: u32) -> Result<JsValue, JsValue> {
    let payload = json!({
        "eventType": event_type,
        "target": target,
        "count": count,
        "timestamp": js_sys::Date::now()
    });

    let payload_str =
        serde_json::to_string(&payload).map_err(|e| JsValue::from_str(&e.to_string()))?;

    let opts = RequestInit::new();
    opts.set_method("POST");
    opts.set_body(&JsValue::from_str(&payload_str));

    let headers = Headers::new().map_err(JsValue::from)?;
    headers
        .append("Content-Type", "application/json")
        .map_err(JsValue::from)?;
    opts.set_headers(&headers);

    let window = match window() {
        Some(win) => win,
        None => return Err(JsValue::from_str("Global `window` object is not available")),
    };

    let fetch_result = window.fetch_with_request(
        &Request::new_with_str_and_init("http://localhost:8080/telemetry", &opts)
            .map_err(JsValue::from)?,
    );

    // Convert JavaScript Promise to Rust Future
    let response = JsFuture::from(fetch_result).await?.dyn_into::<Response>()?;

    // Await the JSON response
    let json = JsFuture::from(response.json()?).await?;
    Ok(json)
}
