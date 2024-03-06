#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UncGasError {
    IncorrectNumber(crate::utils::DecimalNumberParsingError),
    IncorrectUnit(String),
}

impl std::error::Error for UncGasError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            UncGasError::IncorrectNumber(err) => Some(err),
            UncGasError::IncorrectUnit(_) => None,
        }
    }
}
