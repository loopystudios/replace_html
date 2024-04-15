#[cfg(not(target_arch = "wasm32"))]
#[test]
fn replace_by_id_test() {
    assert_eq!(
        replace_html::ReplaceHtmlError::TargetNotSupported,
        replace_html::replace_by_id("test", "").unwrap_err()
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn replace_body_test() {
    assert_eq!(
        replace_html::ReplaceHtmlError::TargetNotSupported,
        replace_html::replace_body("").unwrap_err()
    );
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn replace_by_id_test() {
    _ = replace_html::replace_by_id("test", "");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
fn replace_body_test() {
    _ = replace_html::replace_body("");
}
