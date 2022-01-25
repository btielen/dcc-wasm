mod certificates;
mod cose;
mod parsing_result;
mod read_dcc;

use crate::certificates::find_issuer_cert;
use crate::cose::CoseSingleSigned;
use crate::parsing_result::{ParsingResult, ParsingResultBuilder};
use crate::read_dcc::read_dcc;
use ciborium::value::Value;
use p256::ecdsa;
use p256::ecdsa::signature::*;
use p256::ecdsa::VerifyingKey;
use wasm_bindgen::prelude::*;
use x509_parser::prelude::*;

///
/// Parse a European Digital Covid Certificate (DCC)
///
/// This will parse and verify the signature of a dcc. If parsing or verifying
/// fails, an error message will be available in the ParsingResult. If the verification
/// of the signature fails, the parsed data will still be available.
///
///
#[wasm_bindgen]
pub fn parse(dcc_certificate: &str) -> ParsingResult {
    let result_builder = ParsingResultBuilder::new();

    // base45 decode, zlib inflate, into cose
    let cose = match read_dcc(dcc_certificate) {
        Ok(cose) => cose,
        Err(e) => {
            return result_builder
                .fail_with_error(&format!("Couldn't parse COSE message: {}", e))
                .build()
        }
    };

    // Parse cbor payload into json
    let json = match JsValue::from_serde(cose.payload()) {
        Ok(json) => json,
        Err(_e) => {
            return result_builder
                .fail_with_error("Error on converting the payload to JSON")
                .build()
        }
    };

    let kid = cose.kid().unwrap_or_else(String::new);
    let alg = cose.alg().unwrap_or(0);

    // Parsed successfully
    let parsed_successful = result_builder.success().kid(&kid).alg(alg).data(json);

    let issuer_cert = match find_issuer_cert(&kid) {
        Some(c) => c,
        None => {
            return parsed_successful
                .signature_error(&format!(
                    "No public certificate known for issuer with kid {}",
                    kid
                ))
                .build()
        }
    };

    // Base64 decode issuer certificate
    let issuer_cert = match base64::decode(issuer_cert) {
        Ok(c) => c,
        Err(e) => {
            return parsed_successful
                .signature_error(&format!("Error on base64 decoding issuer cert: {}", e))
                .build()
        }
    };

    // Parse issuer certificate
    let x509cert = match X509Certificate::from_der(&issuer_cert) {
        Ok(c) => c.1,
        Err(e) => {
            return parsed_successful
                .signature_error(&format!("Couldn't load issuer cert: {}", e))
                .build()
        }
    };

    // Get public key from issuer certificate
    let verify_key =
        match p256::PublicKey::from_sec1_bytes(x509cert.public_key().subject_public_key.data) {
            Ok(public_key) => VerifyingKey::from(&public_key),
            Err(e) => {
                return parsed_successful
                    .signature_error(&format!("Couldn't load public key: {}", e))
                    .build()
            }
        };

    // The data to sign
    let to_sign: [Value; 4] = cose.to_be_signed();

    // Cbor encode to_sign
    let mut cbor_encoded = Vec::new();
    match ciborium::ser::into_writer(&to_sign, &mut cbor_encoded) {
        Ok(..) => (),
        Err(e) => {
            return parsed_successful
                .signature_error(&format!("Error on cbor encoding to sign object: {}", e))
                .build()
        }
    };

    // Convert cose signature to ecdsa::Signature
    let signature = match ecdsa::Signature::from_bytes(cose.signature()) {
        Ok(s) => s,
        Err(e) => {
            return parsed_successful
                .signature_error(&format!("Error on parsing signature bytes: {}", e))
                .build()
        }
    };

    // Verify with public key if the given signature is valid
    return match verify_key.verify(&cbor_encoded, &signature) {
        Ok(..) => parsed_successful.signature_valid(true).build(),
        Err(e) => parsed_successful
            .signature_error(&format!("Error verifying signature: {}", e))
            .build(),
    };
}
