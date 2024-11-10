use idionautic_agent::record_event;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_record_event() {
    // Directly await the async function
    let result = record_event("http://localhost:8080/ingest", "testEvent", "testTarget", 1).await;

    // Assertion to check for success
    assert!(result.is_ok(), "record_event failed: {:?}", result.err());

    // Optional: Log to console for debugging
    console::log_1(&"Test for record_event completed".into());
}
