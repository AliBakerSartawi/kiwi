use crate::{parser::ValueType, store::Value};

pub enum ParseError {
    MissingKey,
    MissingValue,
    CannotBeParsedAs(String, ValueType),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::MissingKey => write!(f, "No key provided"),
            ParseError::MissingValue => write!(f, "No value provided"),
            ParseError::CannotBeParsedAs(raw_value, to_type) => {
                write!(f, "{raw_value} cannot be parsed as a ({to_type})")
            }
        }
    }
}

pub fn get_type_from_key(key: &str) -> ValueType {
    match key.split('-').next() {
        Some(t) => match t.to_lowercase().as_str() {
            "str" => ValueType::Str,
            "int" => ValueType::Int,
            "float" => ValueType::Float,
            "bool" => ValueType::Bool,
            _ => ValueType::Str,
        },
        None => ValueType::Str,
    }
}

pub fn parse_raw_value(raw_value: &str, to_type: ValueType) -> Result<Value, String> {
    let parsed = match to_type {
        ValueType::Str => Value::Str(raw_value.to_string()),
        ValueType::Int => raw_value.parse::<i64>().map(Value::Int).map_err(|_| {
            ParseError::CannotBeParsedAs(raw_value.to_string(), to_type).to_string()
        })?,
        ValueType::Float => raw_value.parse::<f64>().map(Value::Float).map_err(|_| {
            ParseError::CannotBeParsedAs(raw_value.to_string(), to_type).to_string()
        })?,
        ValueType::Bool => match raw_value.to_lowercase().as_str() {
            // Also handle boolean value case-insensitively
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => {
                return Err(ParseError::CannotBeParsedAs(raw_value.to_string(), to_type).to_string())
            }
        },
    };

    Ok(parsed)
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::*;

    #[test]
    fn test_get_type_from_key() {
        for r#type in ValueType::iter() {
            match r#type {
                ValueType::Str => {
                    // Invalid type or unintended type prefix
                    assert_eq!(get_type_from_key("x-key"), ValueType::Str);
                    // Unspecified type
                    assert_eq!(get_type_from_key("key"), ValueType::Str);
                    assert_eq!(get_type_from_key("str-key"), ValueType::Str);
                }
                ValueType::Int => assert_eq!(get_type_from_key("int-key"), ValueType::Int),
                ValueType::Float => assert_eq!(get_type_from_key("float-key"), ValueType::Float),
                ValueType::Bool => assert_eq!(get_type_from_key("bool-key"), ValueType::Bool),
            }
        }
    }

    #[test]
    fn test_parse_raw_value() {
        for r#type in ValueType::iter() {
            match r#type {
                ValueType::Str => {
                    assert_eq!(
                        parse_raw_value("value", ValueType::Str),
                        Ok(Value::Str("value".to_string()))
                    );
                }
                ValueType::Int => {
                    assert_eq!(parse_raw_value("42", ValueType::Int), Ok(Value::Int(42)));
                    assert_eq!(
                        parse_raw_value("invalid", ValueType::Int),
                        Err(
                            ParseError::CannotBeParsedAs("invalid".to_string(), ValueType::Int)
                                .to_string()
                        )
                    );
                }
                ValueType::Float => {
                    assert_eq!(
                        parse_raw_value("42.0", ValueType::Float),
                        Ok(Value::Float(42.0))
                    );
                    assert_eq!(
                        parse_raw_value("invalid", ValueType::Float),
                        Err(
                            ParseError::CannotBeParsedAs("invalid".to_string(), ValueType::Float)
                                .to_string()
                        )
                    );
                }
                ValueType::Bool => {
                    assert_eq!(
                        parse_raw_value("true", ValueType::Bool),
                        Ok(Value::Bool(true))
                    );
                    assert_eq!(
                        parse_raw_value("false", ValueType::Bool),
                        Ok(Value::Bool(false))
                    );

                    assert_eq!(
                        parse_raw_value("invalid", ValueType::Bool),
                        Err(
                            ParseError::CannotBeParsedAs("invalid".to_string(), ValueType::Bool)
                                .to_string()
                        )
                    );
                }
            }
        }
    }
}
