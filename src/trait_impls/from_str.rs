use crate::{UncGas, UncGasError, ONE_GIGA_GAS, ONE_TERA_GAS};

impl std::str::FromStr for UncGas {
    type Err = UncGasError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uppercase_s = s.trim().to_ascii_uppercase();
        let (value, unit) = uppercase_s.split_at(
            s.find(|c: char| c.is_ascii_alphabetic())
                .ok_or_else(|| UncGasError::IncorrectUnit(s.to_owned()))?,
        );
        let unit_precision = match unit {
            "TGAS" | "TERAGAS" => ONE_TERA_GAS,
            "GIGAGAS" | "GGAS" => ONE_GIGA_GAS,
            _ => return Err(UncGasError::IncorrectUnit(s.to_owned())),
        };
        Ok(UncGas::from_gas(
            crate::utils::parse_decimal_number(value.trim(), unit_precision)
                .map_err(UncGasError::IncorrectNumber)?,
        ))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{DecimalNumberParsingError, UncGas, UncGasError};

    #[test]
    fn doubledot() {
        let data = "1.1.1 TeraGas";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1.1.1".to_owned())
            ))
        )
    }

    #[test]
    fn space_after_dot() {
        let data = "1. 0 TeraGas";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("1. 0".to_owned())
            ))
        )
    }

    #[test]
    fn decimal_tgas() {
        let data = "0.5 TGas";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(gas, Ok(UncGas::from_ggas(500)))
    }

    #[test]
    fn incorect_currency() {
        let data = "0 pas";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(gas, Err(UncGasError::IncorrectUnit(data.to_owned())))
    }

    #[test]
    fn without_currency() {
        let data = "0";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(gas, Err(UncGasError::IncorrectUnit("0".to_owned())))
    }

    #[test]
    fn invalid_whole() {
        let data = "-1 TeraGas";
        let gas: Result<UncGas, UncGasError> = FromStr::from_str(data);
        assert_eq!(
            gas,
            Err(UncGasError::IncorrectNumber(
                DecimalNumberParsingError::InvalidNumber("-1".to_owned())
            ))
        )
    }

    #[test]
    fn test_from_str_f64_gas_without_int() {
        let unc_gas = UncGas::from_str(".055ggas").unwrap_err();
        assert_eq!(
            unc_gas,
            UncGasError::IncorrectNumber(DecimalNumberParsingError::InvalidNumber(
                ".055".to_string()
            ))
        );
    }

    #[test]
    fn test_from_str_without_unit() {
        let unc_gas = UncGas::from_str("100").unwrap_err();
        assert_eq!(unc_gas, UncGasError::IncorrectUnit("100".to_string()));
    }

    #[test]
    fn test_from_str_incorrect_unit() {
        let unc_gas = UncGas::from_str("100 UAH").unwrap_err();
        assert_eq!(unc_gas, UncGasError::IncorrectUnit("100 UAH".to_string()));
    }

    #[test]
    fn test_from_str_invalid_double_dot() {
        let unc_gas = UncGas::from_str("100.55.").unwrap_err();
        assert_eq!(unc_gas, UncGasError::IncorrectUnit("100.55.".to_string()));
    }

    #[test]
    fn test_from_str_large_fractional_part() {
        let unc_gas = UncGas::from_str("100.1111122222333 ggas").unwrap_err(); // 13 digits after "."
        assert_eq!(
            unc_gas,
            UncGasError::IncorrectNumber(DecimalNumberParsingError::LongFractional(
                "1111122222333".to_string()
            ))
        );
    }

    #[test]
    fn test_from_str_large_int_part() {
        let unc_gas = UncGas::from_str("200123456789123.0 tgas").unwrap_err();
        assert_eq!(
            unc_gas,
            UncGasError::IncorrectNumber(DecimalNumberParsingError::LongWhole(
                "200123456789123".to_string()
            ))
        );
    }

    #[test]
    fn test_from_str_negative_value() {
        let unc_gas = UncGas::from_str("-100 ggas").unwrap_err();
        assert_eq!(
            unc_gas,
            UncGasError::IncorrectNumber(DecimalNumberParsingError::InvalidNumber(
                "-100".to_string()
            ))
        );
    }

    #[test]
    fn unc_gas_from_str_currency_tgas() {
        assert_eq!(
            UncGas::from_str("10 tgas").unwrap(),
            UncGas::from_gas(10_000_000_000_000) // 14 digits
        );
        assert_eq!(
            UncGas::from_str("10.055TERAGAS").unwrap(),
            UncGas::from_gas(10_055_000_000_000) // 14 digits
        );
    }

    #[test]
    fn unc_gas_from_str_currency_gigagas() {
        assert_eq!(
            UncGas::from_str("10 gigagas").unwrap(),
            UncGas::from_gas(10_000_000_000) // 11 digits
        );
        assert_eq!(
            UncGas::from_str("10GGAS ").unwrap(),
            UncGas::from_gas(10_000_000_000) // 11 digits
        );
    }

    #[test]
    fn unc_gas_from_str_f64_tgas() {
        assert_eq!(
            UncGas::from_str("0.000001 tgas").unwrap(),
            UncGas::from_gas(1_000_000) // 7 digits
        );
    }
}
