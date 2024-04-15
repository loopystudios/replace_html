//! Easily replace HTML in wasm32 environments.
//!
//! # Usage
//! ```rust
//! #[cfg(target_arch = "wasm32")]
//! fn main() {
//!     replace_html::replace_by_id("body", "Your platform is not supported!").unwrap();
//! }
//! #[cfg(not(target_arch = "wasm32"))]
//! fn main() {
//!     // Proceed to run your application...
//! }
//! ```

mod error;
pub use error::ReplaceHtmlError;

/// Replace the inner HTML of the document body.
#[cfg(target_arch = "wasm32")]
pub fn replace_body(html: impl Into<String>) -> Result<(), ReplaceHtmlError> {
    let body = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.body())
        .ok_or(ReplaceHtmlError::HeadlessEnvironment)?;
    body.set_inner_html(&html.into());
    Ok(())
}

/// Replace the inner HTML of an element by ID.
#[cfg(target_arch = "wasm32")]
pub fn replace_by_id(
    id: impl Into<String>,
    html: impl Into<String>,
) -> Result<(), ReplaceHtmlError> {
    let id = id.into();
    let dom = web_sys::window()
        .and_then(|w| w.document())
        .ok_or(ReplaceHtmlError::HeadlessEnvironment)?;
    let element = dom
        .get_element_by_id(&id)
        .ok_or(ReplaceHtmlError::ElementNotFound(id))?;
    element.set_inner_html(&html.into());
    Ok(())
}

/// Replace the inner HTML of an element by ID.
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_variables)]
pub fn replace_by_id(
    id: impl Into<String>,
    html: impl Into<String>,
) -> Result<(), ReplaceHtmlError> {
    Err(ReplaceHtmlError::TargetNotSupported)
}

/// Replace the inner HTML of the document body.
#[cfg(not(target_arch = "wasm32"))]
#[allow(unused_variables)]
pub fn replace_body(html: impl Into<String>) -> Result<(), ReplaceHtmlError> {
    Err(ReplaceHtmlError::TargetNotSupported)
}
