mod parsing_result;
mod certificates;

use crate::parsing_result::ParsingResult;
use ciborium::value::Value;
use flate2::read::ZlibDecoder;
use p256::ecdsa;
use p256::ecdsa::signature::*;
use p256::ecdsa::VerifyingKey;
use std::io::Read;
use wasm_bindgen::prelude::*;
use x509_parser::prelude::*;

///
/// Verify the signature of digital covid certificate
///
/// todo: ParsingResult
/// todo: get kid from protectedHeaders or unprotectedHeaders
///
#[wasm_bindgen]
pub fn verify_signature(dcc_certificate: &str, issuer_cert: &str) -> ParsingResult {
    // Strip HC1: prefix
    let mut dcc_certificate: &str = dcc_certificate;
    if let Some(stripped) = dcc_certificate.strip_prefix("HC1:") {
        dcc_certificate = stripped
    }

    // Base45 decode
    let decoded = match base45::decode(dcc_certificate) {
        Ok(d) => d,
        Err(e) => return ParsingResult::create_failure(&format!("Error on base45 decode: {}", e)),
    };

    // Zlib deflate
    let mut decompressed: Vec<u8> = Vec::new();
    let mut decompressor = ZlibDecoder::new(&decoded[..]);
    match decompressor.read_to_end(&mut decompressed) {
        Ok(_result) => (),
        Err(e) => {
            return ParsingResult::create_failure(&format!("Error on zlib decompressing: {}", e))
        }
    }

    // Decode cbor in a cose message
    // cose[0] = protected headers (bytes), cose[1] = unprotected headers (map),
    // cose[2] = payload (bytes), cose[3] = signature (bytes)
    let cose: [Value; 4] = match ciborium::de::from_reader(&decompressed[..]) {
        Ok(cose_message) => cose_message,
        Err(_e) => return ParsingResult::create_failure("The cbor data is not an array"),
    };

    // Base64 decode issuer certificate
    let issuer_cert = match base64::decode(issuer_cert) {
        Ok(c) => c,
        Err(e) => {
            return ParsingResult::create_failure(&format!(
                "Error on base64 decoding issuer cert: {}",
                e
            ))
        }
    };

    // Parse issuer certificate
    let x509cert = match X509Certificate::from_der(&issuer_cert) {
        Ok(c) => c.1,
        Err(e) => {
            return ParsingResult::create_failure(&format!("Couldn't load issuer cert: {}", e))
        }
    };

    // Get public key from issuer certificate
    let verify_key =
        match p256::PublicKey::from_sec1_bytes(x509cert.public_key().subject_public_key.data) {
            Ok(public_key) => VerifyingKey::from(&public_key),
            Err(e) => {
                return ParsingResult::create_failure(&format!("Couldn't load public key: {}", e))
            }
        };

    // The data to sign
    let to_sign: [Value; 4] = [
        Value::from("Signature1"),
        cose[0].clone(),
        Value::Bytes(vec![]),
        cose[2].clone(),
    ];

    // Cbor encode to_sign
    let mut cbor_encoded = Vec::new();
    match ciborium::ser::into_writer(&to_sign, &mut cbor_encoded) {
        Ok(..) => (),
        Err(e) => {
            return ParsingResult::create_failure(&format!(
                "Error on cbor encoding to sign object: {}",
                e
            ))
        }
    };

    // Parse signatures bytes
    let signature: &[u8] = match cose[3].as_bytes() {
        Some(s) => s,
        None => return ParsingResult::create_failure("No bytes found in cose signature"),
    };

    // Convert cose signature to ecdsa::Signature
    let signature = match ecdsa::Signature::from_bytes(signature) {
        Ok(s) => s,
        Err(e) => {
            return ParsingResult::create_failure(&format!(
                "Error on parsing signature bytes: {}",
                e
            ))
        }
    };

    // Verify with public key if the given signature is valid
    return match verify_key.verify(&cbor_encoded, &signature) {
        Ok(..) => ParsingResult::create_success(JsValue::from("YEAH SIGNATURE VALID")),
        Err(e) => ParsingResult::create_failure(&format!("Error verifying signature: {}", e)),
    };
}

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
    let decoded = match base45::decode(dcc_certificate) {
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
