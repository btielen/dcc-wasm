use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// ParsingResult
///
/// The ParsingResult will contain all information
/// about the parsing process. There are two important steps
/// while parsing a Digital Covid Certificate.
///
/// 1) Parsing data
/// When this step was successful the `successful` property
/// should be true
///
/// 2) Validating signature
/// If the signature could be verified, the
/// `signature_valid` should be true
///
/// If one of these steps fail, the error property will be filled
/// with an error message.
///
#[wasm_bindgen(getter_with_clone)]
pub struct ParsingResult {
    /// The algorithm used to sign the certificate
    pub algorithm: i32,

    /// Unique identifier for certificate issuer
    pub kid: String,

    /// Parse successful
    pub successful: bool,

    /// Error message when parsing or verifying the signature fails
    pub error: String,

    /// Data in the DCC
    pub data: JsValue,

    /// signature valid
    pub signature_valid: bool,
}

/// A builder to construct a ParsingResult
pub struct ParsingResultBuilder {
    pub successful: bool,
    pub error: String,
    pub data: JsValue,
    pub signature_valid: bool,
    pub kid: String,
    pub algorithm: i128,
}

impl ParsingResultBuilder {
    /// ParsingResultBuilder is used to build
    /// a ParsingResult. The ParsingResult is by default
    /// unsuccessful.
    pub fn new() -> ParsingResultBuilder {
        ParsingResultBuilder {
            successful: false,
            error: String::from(""),
            data: JsValue::null(),
            signature_valid: false,
            kid: String::from(""),
            algorithm: 0,
        }
    }

    /// Set the successful on the ParsingResult to true
    pub fn success(mut self) -> ParsingResultBuilder {
        self.successful = true;
        self
    }

    /// Parsing failure with an error message
    pub fn fail_with_error(mut self, message: &str) -> ParsingResultBuilder {
        self.successful = false;
        self.error = message.to_string();
        self
    }

    /// Failure on verifying the signature
    pub fn signature_error(mut self, message: &str) -> ParsingResultBuilder {
        self.signature_valid = false;
        self.error = message.to_string();
        self
    }

    /// Set data
    pub fn data(mut self, data: JsValue) -> ParsingResultBuilder {
        self.data = data;
        self
    }

    /// Set when the signature is verified
    pub fn signature_valid(mut self, valid: bool) -> ParsingResultBuilder {
        self.signature_valid = valid;
        self
    }

    /// Set the kid
    pub fn kid(mut self, kid: &str) -> ParsingResultBuilder {
        self.kid = kid.to_string();
        self
    }

    /// Set the algorithm
    pub fn alg(mut self, algorithm: i128) -> ParsingResultBuilder {
        self.algorithm = algorithm;
        self
    }

    /// Build ParsingResult
    pub fn build(self) -> ParsingResult {
        ParsingResult {
            successful: self.successful,
            error: self.error,
            data: self.data,
            signature_valid: self.signature_valid,
            kid: self.kid,
            algorithm: self.algorithm as i32,
        }
    }
}
