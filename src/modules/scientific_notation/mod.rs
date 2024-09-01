#[cfg(test)]
mod tests;

use std::fmt::{Display, Formatter};
use error_mapper::{create_new_error, SystemErrorCodes, TheResult};
use rust_decimal::{Decimal, RoundingStrategy};

#[derive(Debug, Clone, Copy)]
/// Maximum parsing allowed as coefficient mantissa: i64, meaning, on creation, up to i64::MAX and i64::MIN
/// is allowed.
/// ## Examples:
/// - Valid positive number
/// 9.223372036854775807x10^5 -> The mantissa is equivalent to i64::MAX, hence, it can be parsed
/// - Valid negative number 
/// -9.223372036854775807x10^5 -> The mantissa is equivalent to i64::MIN, it'll be parsed as well
/// - Invalid number
/// 9.2233720368547758070x10^5 -> The mantissa contains a value 10 times higher than i64::MAX, 
/// conversion will fail
pub struct ScientificNotation {
    coefficient: Decimal,
    exponent: i16,
    display_decimals: usize
}

impl ScientificNotation {
    pub fn build() -> Self {
        ScientificNotation::default()
    }

    pub fn coefficient(mut self, coefficient: Decimal) -> Self {
        self.coefficient = coefficient;
        self
    }

    pub fn exponent(mut self, exponent: i16) -> Self {
        self.exponent = exponent;
        self
    }
    
    pub fn display_decimals(mut self, decimals: usize) -> Self {
        self.display_decimals = decimals;
        self
    }

    pub fn parse_from_str(input: &str) -> TheResult<Self> {

        let input = input.to_string();

        //  If notation x10^ is not found, return error, it's not valid scientific notation
        if !input.contains("x10^") {
            return Err(create_new_error!("Input format was incorrect. Expected CCcx10^EEe"))
        }

        let full_notation = input.split("x10^").collect::<Vec<&str>>();
        if full_notation.len() > 2 {
            return Err(create_new_error!("Input had an unexpected format!"))
        }

        let coefficient = full_notation[0]
            .parse::<Decimal>()
            .map_err(|error| create_new_error!(SystemErrorCodes::ParseError ,error.to_string()))?;

        let exponent = full_notation[1]
            .parse::<i16>()
            .map_err(|error| create_new_error!(SystemErrorCodes::ParseError ,error.to_string()))?;

        let display_decimals_unparsed = coefficient.fract();
        let display_decimals = if display_decimals_unparsed == Decimal::ZERO {
            0
        } else {
            display_decimals_unparsed
                .to_string()
                .split('.')
                .collect::<Vec<&str>>()[1]
                .len()
        };

        Ok(Self {
            coefficient,
            exponent,
            display_decimals
        })
    }
}

impl Default for ScientificNotation {
    fn default() -> Self {
        Self {
            coefficient: Decimal::from(1),
            exponent: i16::default(),
            display_decimals: 2
        }
    }
}

impl Display for ScientificNotation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}x10^{}",
            self.coefficient.round_dp_with_strategy(
                self.display_decimals as u32, RoundingStrategy::MidpointTowardZero
            ),
            self.exponent
        )
    }
}

impl std::ops::Add for ScientificNotation {
    type Output = ScientificNotation;
    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Sub for ScientificNotation {
    type Output = ScientificNotation;
    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Mul for ScientificNotation{
    type Output = ScientificNotation;
    fn mul(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl std::ops::Div for ScientificNotation {
    type Output = ScientificNotation;
    fn div(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
