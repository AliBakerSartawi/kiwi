use crate::{parser::ValueType, store::Value};

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
        ValueType::Int => raw_value
            .parse::<i64>()
            .map(Value::Int)
            .map_err(|_| format!("{raw_value} cannot be parsed as an integer"))?,
        ValueType::Float => raw_value
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|_| format!("{raw_value} cannot be parsed as a float"))?,
        ValueType::Bool => match raw_value.to_lowercase().as_str() {
            // Also handle boolean value case-insensitively
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => return Err("Invalid boolean value; expected true or false".into()),
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
                        Err("invalid cannot be parsed as an integer".into())
                    );
                }
                ValueType::Float => {
                    assert_eq!(
                        parse_raw_value("42.0", ValueType::Float),
                        Ok(Value::Float(42.0))
                    );
                    assert_eq!(
                        parse_raw_value("invalid", ValueType::Float),
                        Err("invalid cannot be parsed as a float".into())
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
                        Err("Invalid boolean value; expected true or false".into())
                    );
                }
            }
        }
    }
}
