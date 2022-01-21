use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

///
/// The parsing result
///
///
#[wasm_bindgen(getter_with_clone)]
pub struct ParsingResult {
    pub successful: bool,
    pub error: String,
    pub data: JsValue,
}

impl ParsingResult {
    ///
    /// Create a valid ParsingResult
    ///
    pub fn create_success(data: JsValue) -> ParsingResult {
        ParsingResult {
            successful: true,
            error: String::from(""),
            data,
        }
    }

    ///
    /// Create a invalid ParsingResult
    ///
    pub fn create_failure(error: &str) -> ParsingResult {
        ParsingResult {
            successful: false,
            error: error.to_string(),
            data: JsValue::UNDEFINED,
        }
    }

    pub fn is_successful(&self) -> bool {
        self.successful
    }

    pub fn error(&self) -> &str {
        &self.error
    }

    pub fn data(&self) -> &JsValue {
        &self.data
    }
}

///
/// Implement default value
///
impl Default for ParsingResult {
    fn default() -> Self {
        ParsingResult {
            successful: false,
            error: String::from(""),
            data: JsValue::UNDEFINED,
        }
    }
}
