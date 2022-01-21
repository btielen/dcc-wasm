mod parsing_result;

use crate::parsing_result::ParsingResult;
use base45::decode;
use ciborium::value::Value;
use flate2::read::ZlibDecoder;
use std::io::Read;
use wasm_bindgen::prelude::*;

///
/// Parse a european digital covid certificate
///
#[wasm_bindgen]
pub fn parse(dcc_certificate: &str) -> ParsingResult {
    // strip HC1: prefix
    let mut dcc_certificate: &str = dcc_certificate;
    if let Some(stripped) = dcc_certificate.strip_prefix("HC1:") {
        dcc_certificate = stripped
    }

    // base45 decode
    let decoded = match decode(dcc_certificate) {
        Ok(d) => d,
        Err(e) => return ParsingResult::create_failure(&format!("Error on base45 decode: {}", e)),
    };

    // zlib deflate
    let mut decompressed: Vec<u8> = Vec::new();
    let mut decompressor = ZlibDecoder::new(&decoded[..]);
    match decompressor.read_to_end(&mut decompressed) {
        Ok(_result) => (),
        Err(e) => {
            return ParsingResult::create_failure(&format!("Error on zlib decompressing: {}", e))
        }
    }

    // decode cbor in a cose message
    let cose: [Value; 4] = match ciborium::de::from_reader(&decompressed[..]) {
        Ok(cose_message) => cose_message,
        Err(_e) => return ParsingResult::create_failure("The cbor data is not an array"),
    };

    // convert payload of cose message to a javascript value
    let payload: &[u8] = match cose[2].as_bytes() {
        Some(payload) => payload,
        None => return ParsingResult::create_failure("No bytes found in payload"),
    };

    let cbor: Value = match ciborium::de::from_reader(payload) {
        Ok(cbor) => cbor,
        Err(_e) => return ParsingResult::create_failure("Couldn't parse payload of COSE message"),
    };

    let json = match JsValue::from_serde(&cbor) {
        Ok(json) => json,
        Err(_e) => return ParsingResult::create_failure("Error on converting the payload to Json"),
    };

    ParsingResult::create_success(json)
}
