use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::console;
use web_sys::{window, Headers, Request, RequestInit, Response};

#[wasm_bindgen]
pub async fn record_event(
    url: &str,
    event_type: &str,
    target: &str,
    count: u32,
) -> Result<JsValue, JsValue> {
    // Create a new Date object and manually format as ISO 8601
    let date = js_sys::Date::new_0();
    let timestamp = format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        date.get_utc_full_year(),
        date.get_utc_month() + 1, // Months are zero-indexed
        date.get_utc_date(),
        date.get_utc_hours(),
        date.get_utc_minutes(),
        date.get_utc_seconds(),
        date.get_utc_milliseconds()
    );

    let payload = json!({
        "eventType": event_type,
        "target": target,
        "count": count,
        "timestamp": timestamp // ISO 8601 formatted timestamp as String
    });

    let payload_str =
        serde_json::to_string(&payload).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let payload_str_jsvalue = JsValue::from_str(&payload_str);
    console::log_1(&payload_str_jsvalue);

    let opts = RequestInit::new();
    opts.set_method("POST");

    let headers = Headers::new().map_err(JsValue::from)?;
    headers
        .append("Content-Type", "application/json")
        .map_err(JsValue::from)?;
    opts.set_headers(&headers);
    opts.set_body(&payload_str_jsvalue);

    let window =
        window().ok_or_else(|| JsValue::from_str("Global `window` object is not available"))?;
    let request = Request::new_with_str_and_init(url, &opts).map_err(JsValue::from)?;
    let fetch_result = window.fetch_with_request(&request);

    let response = JsFuture::from(fetch_result).await?.dyn_into::<Response>()?;

    // Check for 202 Accepted status
    if response.status() == 202 {
        if response.headers().get("Content-Type").unwrap_or_default()
            == Some("application/json".to_string())
        {
            let json = JsFuture::from(response.json()?).await?;
            Ok(json)
        } else {
            Err(JsValue::from_str("Received non-JSON response from server"))
        }
    } else {
        Err(JsValue::from_str(&format!(
            "Unexpected response status: {}",
            response.status()
        )))
    }
}
