//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use dcc_wasm::parse;
use serde_json::Value;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const TEST_DCC: &str = "6BFOXN*TS0BI$ZD-PHQ7I9AD66V5B22CH9M9ESI9XBHXK-%69LQOGI.*V76GCV4*XUA2P-FHT-HNTI4L6N$Q%UG/YL WO*Z7ON15 BM0VM.JQ$F4W17PG4.VAS5EG4V*BRL0K-RDY5RWOOH6PO9:TUQJAJG9-*NIRICVELZUZM9EN9-O9:PICIG805CZKHKB-43.E3KD3OAJ6*K6ZCY73JC3KD3ZQTWD3E.KLC8M3LP-89B9K+KB2KK3M*EDZI9$JAQJKKIJX2MM+GWHKSKE MCAOI8%MCU5VTQDPIMQK9*O7%NC.UTWA6QK.-T3-SY$NCU5CIQ 52744E09TBOC.UKMI$8R+1A7CPFRMLNKNM8JI0JPGN:0K7OOBRLY667SYHJL9B7VPO:SWLH1/S4KQQK0$5REQT5RN1FR%SHPLRKWJO8LQ84EBC$-P4A0V1BBR5XWB3OCGEK:$8HHOLQOZUJ*30Q8CD1";

#[wasm_bindgen_test]
fn it_is_an_object() {
    let result = parse(TEST_DCC);
    assert_eq!(result.data().is_object(), true);
}

#[wasm_bindgen_test]
fn it_contains_props() {
    let result = parse(TEST_DCC);
    let json: Value = result.data().into_serde().unwrap();
    assert_eq!(json["1"], String::from("DE"))
}

#[wasm_bindgen_test]
fn it_is_valid() {
    assert_eq!(parse(TEST_DCC).is_successful(), true)
}

#[wasm_bindgen_test]
fn invalid_dcc_returns_false() {
    assert_eq!(parse("some_invalid_data").is_successful(), false)
}

#[wasm_bindgen_test]
fn invalid_dcc_returns_error_msg() {
    assert!(parse("some_invalid_data").error().len() > 0)
}

#[wasm_bindgen_test]
fn it_ignores_the_header() {
    assert!(parse(&format!("HC1:{}", TEST_DCC)).is_successful())
}
