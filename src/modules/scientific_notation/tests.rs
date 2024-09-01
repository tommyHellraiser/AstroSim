use rust_decimal::Decimal;
use crate::modules::scientific_notation::ScientificNotation;

#[test]
fn parse_scientific_notation_from_string() {
    let input = "5x10^15";
    let sci_notation = ScientificNotation::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.coefficient, Decimal::from(5));
    assert_eq!(sci_notation.exponent, Decimal::from(15));
    assert_eq!(sci_notation.display_decimals, 0);
}

#[test]
fn parse_scientific_notation_from_string_2() {
    let input = "5.1237514651x10^38";
    let sci_notation = ScientificNotation::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.coefficient, Decimal::new(51237514651, 10));
    assert_eq!(sci_notation.exponent, Decimal::from(38));
    assert_eq!(sci_notation.display_decimals, 10);
}

#[test]
fn parse_with_error_invalid_exponent() {
    let input = "2x10^23.4";
    let sci_notation = ScientificNotation::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn parse_with_error_invalid_coefficient() {
    let input = "2ax10^20";
    let sci_notation = ScientificNotation::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn parse_with_error_x10_repeated() {
    let input = "2x10^x10^23";
    let sci_notation = ScientificNotation::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn parse_with_error_invalid_input() {
    let input = "asd1234";
    let sci_notation = ScientificNotation::parse_from_str(input);

    assert!(sci_notation.is_err());
}

#[test]
fn parse_and_display_back() {
    let input = "315.2x10^14";
    let sci_notation = ScientificNotation::parse_from_str(input).unwrap();

    assert_eq!(sci_notation.to_string(), input.to_string());
}

#[test]
fn from_builder_into_string() {    
    let sci_notation = ScientificNotation::build()
        .coefficient(Decimal::new(6278964, 5))
        .exponent(Decimal::from(20))
        .display_decimals(3);

    assert_eq!(sci_notation.to_string(), "62.790x10^20".to_string());
}
