use crate::CoseSingleSigned;
use ciborium::value::Value;
use flate2::read::ZlibDecoder;
use std::convert::TryFrom;
use std::io::Read;

pub fn read_dcc(dcc_certificate: &str) -> Result<CoseSingleSigned, String> {
    // Strip HC1: prefix
    let mut dcc_certificate: &str = dcc_certificate;
    if let Some(stripped) = dcc_certificate.strip_prefix("HC1:") {
        dcc_certificate = stripped
    }

    // Base45 decode
    let decoded = match base45::decode(dcc_certificate) {
        Ok(d) => d,
        Err(e) => return Err(format!("Error on base45 decode: {}", e)),
    };

    // Zlib deflate
    let mut decompressed: Vec<u8> = Vec::new();
    let mut decompressor = ZlibDecoder::new(&decoded[..]);
    match decompressor.read_to_end(&mut decompressed) {
        Ok(_result) => (),
        Err(e) => return Err(format!("Error on zlib decompressing: {}", e)),
    }

    // Decode cbor in a cose message
    let cose: [Value; 4] = match ciborium::de::from_reader(&decompressed[..]) {
        Ok(cose_message) => cose_message,
        Err(_e) => return Err(String::from("The cbor data is not an array")),
    };

    match CoseSingleSigned::try_from(cose) {
        Ok(cose) => Ok(cose),
        Err(e) => Err(format!("Couldn't parse COSE message: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DCC: &str = "6BFOXN*TS0BI$ZD-PHQ7I9AD66V5B22CH9M9ESI9XBHXK-%69LQOGI.*V76GCV4*XUA2P-FHT-HNTI4L6N$Q%UG/YL WO*Z7ON15 BM0VM.JQ$F4W17PG4.VAS5EG4V*BRL0K-RDY5RWOOH6PO9:TUQJAJG9-*NIRICVELZUZM9EN9-O9:PICIG805CZKHKB-43.E3KD3OAJ6*K6ZCY73JC3KD3ZQTWD3E.KLC8M3LP-89B9K+KB2KK3M*EDZI9$JAQJKKIJX2MM+GWHKSKE MCAOI8%MCU5VTQDPIMQK9*O7%NC.UTWA6QK.-T3-SY$NCU5CIQ 52744E09TBOC.UKMI$8R+1A7CPFRMLNKNM8JI0JPGN:0K7OOBRLY667SYHJL9B7VPO:SWLH1/S4KQQK0$5REQT5RN1FR%SHPLRKWJO8LQ84EBC$-P4A0V1BBR5XWB3OCGEK:$8HHOLQOZUJ*30Q8CD1";

    #[test]
    fn it_parses() {
        assert_eq!(read_dcc(TEST_DCC).is_ok(), true);
    }

    #[test]
    fn it_reads_kid() {
        assert_eq!(
            read_dcc(TEST_DCC).unwrap().kid(),
            Some(String::from("DEsVUSvpFAE="))
        )
    }

    #[test]
    fn it_reads_alg() {
        assert_eq!(read_dcc(TEST_DCC).unwrap().alg(), Some(-7))
    }
}
