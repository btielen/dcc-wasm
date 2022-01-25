# Parse a digital covid certificate

![types](https://img.shields.io/npm/types/dcc-wasm)
![version](https://img.shields.io/npm/v/dcc-wasm)
![licence](https://img.shields.io/npm/l/dcc-wasm)

This is a WebAssembly (wasm) package to parse a European Digital Covid Certificate (DCC). The
source is written in Rust, compiled to webassembly. **Be careful!** Signatures are not being verified.

## Installation

Use your favourite package manager to use this package in your javascript projects.

`npm install dcc-wasm --save`

### webpack >= 5

Add the following to `webpack.config.js`

```
{
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
      }
    ]
  },
  experiments: {
    asyncWebAssembly: true
  }
}
```


## Getting started

Here is an [example](https://runkit.com/embed/cfztjsur5pe2) on how use this package

```js
import { parse } from "dcc-wasm"

const parseResult = parse("HC1:6BFOXN*TS0BI$ZD-PHQ7I9AD66V5B22CH9M9ESI9XBHXK-%69LQOGI.*V76GCV4*XUA2P-FHT-HNTI4L6N$Q%UG/YL WO*Z7ON15 BM0VM.JQ$F4W17PG4.VAS5EG4V*BRL0K-RDY5RWOOH6PO9:TUQJAJG9-*NIRICVELZUZM9EN9-O9:PICIG805CZKHKB-43.E3KD3OAJ6*K6ZCY73JC3KD3ZQTWD3E.KLC8M3LP-89B9K+KB2KK3M*EDZI9$JAQJKKIJX2MM+GWHKSKE MCAOI8%MCU5VTQDPIMQK9*O7%NC.UTWA6QK.-T3-SY$NCU5CIQ 52744E09TBOC.UKMI$8R+1A7CPFRMLNKNM8JI0JPGN:0K7OOBRLY667SYHJL9B7VPO:SWLH1/S4KQQK0$5REQT5RN1FR%SHPLRKWJO8LQ84EBC$-P4A0V1BBR5XWB3OCGEK:$8HHOLQOZUJ*30Q8CD1");

if(!parseResult.successful || !parseResult.signature_valid) {
  console.log(parseResult.error)
}

console.log(parseResult.data)
```

Would result in:

```json
{
  "1": "DE",
  "4": 1643356073,
  "6": 1622316073,
  "-260": {
    "1": {
      "r": [
        {
          "ci": "URN:UVCI:01DE/5CWLU12RNOB9RXSEOP6FG8#W",
          "is": "Robert Koch-Institut",
          "co": "DE",
          "tg": "840539006",
          "fr": "2021-01-10",
          "df": "2021-05-29",
          "du": "2021-06-15"
        }
      ],
      "dob": "1964-08-12",
      "nam": {
        "fn": "Mustermann",
        "gn": "Erika",
        "fnt": "MUSTERMANN",
        "gnt": "ERIKA"
      },
      "ver": "1.0.0"
    }
  }
}
```

The specifications of this data can be found [here](https://github.com/ehn-dcc-development/hcert-spec/blob/main/hcert_spec.md)

## ParsingResult

| Property        | Type    | Description                                           |
|-----------------|---------|-------------------------------------------------------|
| successful      | bool    | Will be true if the data could be parsed successfully |
| signature_valid | bool    | Will be true if the signature could be verifyed       |
| kid             | string  | Issuer identifier                                     |
| algorithm       | number  | The algorithm to sign the data                        |
| data            | unknown | The data in the health certificate                    |

## Building from source
1. Clone repository
2. Install `wasm-pack` by running
   `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`
   in your terminal
3. Run `wasm-pack build --target nodejs --outdir package/node` to build the wasm package for a nodejs environment
4. Run `wasm-pack build --outdir package/webpack` to build the wasm package a browser package

## Running tests

### Rust

- `wasm-pack test --node`
- `wasm-pack test --firefox --headless`

### Javascript

Follow these steps from the root dir to run the javascript tests
- `wasm-pack build --target nodejs --outdir package/node` to build
- change directory to package `cd package`
- `npm install`
- `npm run test` to run the tests