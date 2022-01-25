const dcc = require('dcc-wasm')

describe('It parses a valid dcc', () => {

  let result = dcc.parse('HC1:6BFOXN*TS0BI$ZD-PHQ7I9AD66V5B22CH9M9ESI9XBHXK-%69LQOGI.*V76GCV4*XUA2P-FHT-HNTI4L6N$Q%UG/YL WO*Z7ON15 BM0VM.JQ$F4W17PG4.VAS5EG4V*BRL0K-RDY5RWOOH6PO9:TUQJAJG9-*NIRICVELZUZM9EN9-O9:PICIG805CZKHKB-43.E3KD3OAJ6*K6ZCY73JC3KD3ZQTWD3E.KLC8M3LP-89B9K+KB2KK3M*EDZI9$JAQJKKIJX2MM+GWHKSKE MCAOI8%MCU5VTQDPIMQK9*O7%NC.UTWA6QK.-T3-SY$NCU5CIQ 52744E09TBOC.UKMI$8R+1A7CPFRMLNKNM8JI0JPGN:0K7OOBRLY667SYHJL9B7VPO:SWLH1/S4KQQK0$5REQT5RN1FR%SHPLRKWJO8LQ84EBC$-P4A0V1BBR5XWB3OCGEK:$8HHOLQOZUJ*30Q8CD1')

  test('parse successful', () => {
    expect(result.successful).toBe(true)
  })

  test('signature valid', () => {
    expect(result.signature_valid).toBe(false) // we can't verify signature of test data
  })

  test('kid', () => {
    expect(result.kid).toBe('DEsVUSvpFAE=')
  })

  test('algorithm', () => {
    expect(result.algorithm).toBe(-7)
  })

  test('data', () => {
    expect(result.data).toEqual({
      '1': 'DE',
      '4': 1643356073,
      '6': 1622316073,
      '-260': {
        '1': {
          'r': [
            {
              'ci': 'URN:UVCI:01DE/5CWLU12RNOB9RXSEOP6FG8#W',
              'is': 'Robert Koch-Institut',
              'co': 'DE',
              'tg': '840539006',
              'fr': '2021-01-10',
              'df': '2021-05-29',
              'du': '2021-06-15'
            }
          ],
          'dob': '1964-08-12',
          'nam': {
            'fn': 'Mustermann',
            'gn': 'Erika',
            'fnt': 'MUSTERMANN',
            'gnt': 'ERIKA'
          },
          'ver': '1.0.0'
        }
      }
    })
  })
})

describe('Try to parse invalid dcc', () => {

  const result = dcc.parse('INVALID_DATA')

  test('unsuccessful', () => {
    expect(result.successful).toBe(false)
  })

  test('signature invalid', () => {
    expect(result.signature_valid).toBe(false)
  })

  test('error message', () => {
    expect(result.error.length).toBeGreaterThan(0)
  })
})