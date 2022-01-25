use ciborium::value::Value;
use std::convert::{TryFrom, TryInto};

type Headers = Vec<(Value, Value)>;

///
/// A Single Signed Cose Message
///
///
/// https://datatracker.ietf.org/doc/html/rfc8152
///
///
pub struct CoseSingleSigned {
    protected_headers: ProtectedHeaders,
    unprotected_headers: Headers,
    payload: Payload,
    signature: Vec<u8>,
}

///
/// Protected headers of a COSE Message
///
struct ProtectedHeaders {
    raw: Vec<u8>,
    data: Headers,
}

///
/// Payload of cose message
///
#[derive(PartialEq)]
struct Payload {
    raw: Vec<u8>,
    data: Value,
}

impl CoseSingleSigned {
    /// Get a reference to the payload
    pub fn payload(&self) -> &Value {
        &self.payload.data
    }

    /// Get a reference to the signature
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Get kid, the functions prefers the kid from the protected headers
    pub fn kid(&self) -> Option<String> {
        let mut kid = header(&self.protected_headers.data, Header::Kid);

        if kid.is_none() {
            kid = header(&self.unprotected_headers, Header::Kid)
        }

        let kid_bytes = match kid {
            Some(kid) => match kid.as_bytes() {
                Some(b) => b,
                None => return None,
            },
            None => return None,
        };

        Some(base64::encode(kid_bytes))
    }

    /// Get algorithm, prefer the value in the protected headers
    pub fn alg(&self) -> Option<i128> {
        let mut alg = header(&self.protected_headers.data, Header::Alg);

        if alg.is_none() {
            alg = header(&self.unprotected_headers, Header::Alg);
        }

        alg?;

        let int: i128 = alg.unwrap().as_integer().unwrap().try_into().unwrap();
        Some(int)
    }

    /// The value that has to be signed
    pub fn to_be_signed(&self) -> [Value; 4] {
        [
            Value::from("Signature1"),
            Value::from(self.protected_headers.raw.clone()),
            Value::Bytes(vec![]),
            Value::from(self.payload.raw.clone()),
        ]
    }
}

impl TryFrom<[Value; 4]> for CoseSingleSigned {
    type Error = String;

    fn try_from(value: [Value; 4]) -> Result<Self, Self::Error> {
        // convert protected headers
        let protected_headers: ProtectedHeaders = value[0].clone().try_into()?;

        // convert payload
        let payload: Payload = value[2].clone().try_into()?;

        // convert unprotected headers
        let unprotected_headers: Headers = match value[1].as_map() {
            Some(map) => map.to_vec(),
            None => return Err(String::from("Unprotected headers is not a valid map")),
        };

        // convert signature
        let signature: Vec<u8> = match value[3].as_bytes() {
            Some(bytes) => bytes.to_vec(),
            None => return Err(String::from("No bytes found in signature")),
        };

        Ok(CoseSingleSigned {
            protected_headers,
            unprotected_headers,
            payload,
            signature,
        })
    }
}

impl TryFrom<Value> for Payload {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let bytes = match value.as_bytes() {
            Some(payload_as_bytes) => payload_as_bytes,
            None => return Err(String::from("Payload is not an byte value")),
        };

        match ciborium::de::from_reader(&bytes[..]) {
            Ok(value) => Ok(Payload {
                raw: bytes.clone(),
                data: value,
            }),
            Err(_e) => Err(String::from("Payload is not cbor encoded")),
        }
    }
}

impl TryFrom<Value> for ProtectedHeaders {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        // Get bytes
        let value: Vec<u8> = match value.as_bytes() {
            Some(cbor_decoded) => cbor_decoded.to_vec(),
            None => return Err(String::from("Protected headers is not a byte value")),
        };

        // Cbor decode
        let protected_headers: Value = match ciborium::de::from_reader(&value[..]) {
            Ok(v) => v,
            Err(_e) => return Err(String::from("Could't parse protected headers")),
        };

        match protected_headers {
            Value::Null => Ok(ProtectedHeaders {
                raw: value,
                data: Vec::new(), // convert null to empty map
            }),
            Value::Map(map) => Ok(ProtectedHeaders {
                raw: value,
                data: map,
            }),
            _ => Err(String::from("Protected headers is not a valid map")),
        }
    }
}

///
/// Common COSE Headers Parameters
///
enum Header {
    Alg,
    Kid,
}

impl Header {
    /// Label value of header
    fn label(&self) -> u8 {
        match *self {
            Header::Alg => 1,
            Header::Kid => 4,
        }
    }
}

///
/// Get header value from headers
///
fn header(headers: &[(Value, Value)], header_label: Header) -> Option<&Value> {
    headers
        .iter()
        .find(|&h| h.0 == Value::from(header_label.label()))
        .map(|header| &header.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ciborium::value::Value;
    use serde::ser::Serialize;

    /// Helper function to encode an value to cbor bytes
    fn cbor_encode<T: ?Sized + Serialize>(value: &T) -> Vec<u8> {
        let mut encoded: Vec<u8> = Vec::new();
        ciborium::ser::into_writer(value, &mut encoded).unwrap();

        encoded
    }

    #[test]
    fn payload_from_value_is_ok() {
        let value: Vec<u8> = cbor_encode(&"test");
        assert!(Payload::try_from(Value::from(&value[..])).is_ok())
    }

    #[test]
    fn payload_from_non_bytes_results_in_error() {
        assert!(Payload::try_from(Value::from("test")).is_err())
    }

    #[test]
    fn payload_raw_data() {
        let value: Vec<u8> = cbor_encode(&"test");
        let payload = Payload::try_from(Value::from(&value[..])).unwrap();
        assert_eq!(payload.data, Value::from("test"))
    }

    #[test]
    fn protected_headers_is_ok() {
        let headers: Value = vec![(Value::from(1), Value::from("-7"))]
            .try_into()
            .unwrap();
        let encoded = cbor_encode(&headers);
        let result = ProtectedHeaders::try_from(Value::from(encoded));
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn protected_headers_is_error_when_no_map() {
        let encoded = cbor_encode("not_a_map");
        let result = ProtectedHeaders::try_from(Value::from(encoded));
        assert_eq!(result.is_err(), true);
    }

    fn test_values() -> [Value; 4] {
        let headers: Value = vec![
            (Value::from(1), Value::from(-7)),
            (Value::from(4), Value::from("some_kid")),
        ]
        .try_into()
        .unwrap();

        [
            Value::from(cbor_encode(&headers)),
            Value::from(headers),
            Value::from(cbor_encode("some_payload")),
            Value::from(cbor_encode("some_signature")),
        ]
    }

    #[test]
    fn value_array_into_cose() {
        let result = CoseSingleSigned::try_from(test_values());
        assert_eq!(result.is_ok(), true);
    }

    #[test]
    fn alg() {
        let cose = CoseSingleSigned {
            protected_headers: ProtectedHeaders {
                data: vec![(Value::from(1), Value::from(-7))].try_into().unwrap(),
                raw: Vec::new(),
            },
            unprotected_headers: vec![(Value::from(1), Value::from(10))].try_into().unwrap(),
            payload: Payload {
                raw: vec![],
                data: Value::Null,
            },
            signature: vec![],
        };

        assert_eq!(cose.alg(), Some(-7));
    }
}
