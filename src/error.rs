#[derive(Debug, thiserror::Error, PartialEq, Eq, Clone)]
pub enum ReplaceHtmlError {
    #[error("The element was not found: '{0}'")]
    ElementNotFound(String),
    #[error("HTML could not be replaced because the environment is headless.")]
    HeadlessEnvironment,
    #[error("Replacing HTML only works on `wasm32` architecture.")]
    TargetNotSupported,
}
