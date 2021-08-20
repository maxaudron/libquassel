use druid::text::{Formatter, Validation, ValidationError};

pub struct U16Formatter;

impl Formatter<u16> for U16Formatter {
    fn format(&self, value: &u16) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, _sel: &druid::text::Selection) -> Validation {
        if input.is_empty() {
            return Validation::success();
        }

        if input.len() > 6 {
            return Validation::failure(U16ValidationError::WrongNumberOfCharacters);
        }

        match input.parse::<u16>() {
            Ok(_) => Validation::success(),
            Err(err) => Validation::failure(err),
        }
    }

    fn value(&self, input: &str) -> Result<u16, ValidationError> {
        if input.is_empty() || input.len() > 5 {
            return Err(ValidationError::new(
                U16ValidationError::WrongNumberOfCharacters,
            ));
        }

        input.parse().map_err(|err| ValidationError::new(err))
    }
}

#[derive(Debug, Clone)]
pub enum U16ValidationError {
    WrongNumberOfCharacters,
}

impl std::fmt::Display for U16ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::any::type_name_of_val(self))
    }
}

impl std::error::Error for U16ValidationError {}
