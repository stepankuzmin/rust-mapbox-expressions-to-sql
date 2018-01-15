#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::collections::HashMap;
use serde_json::Value;

mod error;
use error::Error;

type Handle = fn(Vec<String>) -> String;

fn has(operands: Vec<String>) -> String {
    format!("{} IS NOT NULL", operands[0].trim_matches('\''))
}

fn has_not(operands: Vec<String>) -> String {
    format!("{} IS NULL", operands[0].trim_matches('\''))
}

fn eq(operands: Vec<String>) -> String {
    format!("{} = {}", operands[0].trim_matches('\''), operands[1])
}

fn not_eq(operands: Vec<String>) -> String {
    format!("{} <> {}", operands[0].trim_matches('\''), operands[1])
}

fn gt(operands: Vec<String>) -> String {
    format!("{} > {}", operands[0].trim_matches('\''), operands[1])
}

fn gte(operands: Vec<String>) -> String {
    format!("{} >= {}", operands[0].trim_matches('\''), operands[1])
}

fn lt(operands: Vec<String>) -> String {
    format!("{} < {}", operands[0].trim_matches('\''), operands[1])
}

fn lte(operands: Vec<String>) -> String {
    format!("{} <= {}", operands[0].trim_matches('\''), operands[1])
}

fn is_in(operands: Vec<String>) -> String {
    let (key, values) = operands.split_first().unwrap();
    format!("{} IN ({})", key.trim_matches('\''), values.join(", "))
}

fn not_in(operands: Vec<String>) -> String {
    let (key, values) = operands.split_first().unwrap();
    format!("{} NOT IN ({})", key.trim_matches('\''), values.join(", "))
}

fn all(operands: Vec<String>) -> String {
    operands.join(" AND ")
}

fn any(operands: Vec<String>) -> String {
    operands.join(" OR ")
}

lazy_static! {
    static ref OPERATORS: HashMap<&'static str, Handle> = {
        let mut m = HashMap::new();
        m.insert("has", has as Handle);
        m.insert("!has", has_not as Handle);
        m.insert("==", eq as Handle);
        m.insert("!=", not_eq as Handle);
        m.insert(">", gt as Handle);
        m.insert(">=", gte as Handle);
        m.insert("<", lt as Handle);
        m.insert("<=", lte as Handle);
        m.insert("in", is_in as Handle);
        m.insert("!in", not_in as Handle);
        m.insert("all", all as Handle);
        m.insert("any", any as Handle);
        m
    };
}

pub fn to_sql(expression: &Value) -> Result<String, Error> {
    if !expression.is_array() {
        return match expression.clone() {
            Value::String(str) => Ok(format!("'{}'", str)),
            _ => {
                let operand: String = serde_json::to_string(&expression).unwrap();
                Ok(operand)
            }
        };
    }

    let array = expression.as_array().unwrap().as_slice();
    let (operator, operands) = array.split_first().unwrap();
    
    let f = OPERATORS.get(operator.as_str().unwrap()).unwrap();

    let str_operands: Vec<String> = operands
        .into_iter()
        .map(|operand| to_sql(operand).unwrap())
        .collect();

    Ok(f(str_operands))
}

pub fn parse(expression: &str) -> Result<String, Error> {
    let value: Value = serde_json::from_str(expression).unwrap();
    to_sql(&value)
}

#[cfg(test)]
mod tests {
    use super::parse;

    #[test]
    fn test_existential() {
        assert_eq!(parse(r#"["has", "key"]"#).unwrap(), "key IS NOT NULL");
        assert_eq!(parse(r#"["!has", "key"]"#).unwrap(), "key IS NULL");
    }

    #[test]
    fn test_comparison() {
        assert_eq!(parse(r#"["==", "key", 42]"#).unwrap(), r#"key = 42"#);
        assert_eq!(parse(r#"["==", "key", "value"]"#).unwrap(), "key = 'value'");
        assert_eq!(parse(r#"["!=", "key", "value"]"#).unwrap(), "key <> 'value'");
        assert_eq!(parse(r#"[">", "key", "value"]"#).unwrap(), "key > 'value'");
        assert_eq!(parse(r#"[">=", "key", "value"]"#).unwrap(), "key >= 'value'");
        assert_eq!(parse(r#"["<", "key", "value"]"#).unwrap(), "key < 'value'");
        assert_eq!(parse(r#"["<=", "key", "value"]"#).unwrap(), "key <= 'value'");
    }
    
    #[test]
    fn test_set_membership() {
        assert_eq!(parse(r#"["in", "key", "v0", "v1", "v2"]"#).unwrap(), "key IN ('v0', 'v1', 'v2')");
        assert_eq!(parse(r#"["!in", "key", "v0", "v1", "v2"]"#).unwrap(), "key NOT IN ('v0', 'v1', 'v2')");
    }

    #[test]
    fn test_combining() {
        assert_eq!(
            parse(r#"["all", ["==", "key0", "value0"], ["==", "key1", "value1"], ["==", "key2", "value2"]]"#).unwrap(),
            r#"key0 = 'value0' AND key1 = 'value1' AND key2 = 'value2'"#
        );

        assert_eq!(
            parse(r#"["any", ["==", "key0", "value0"], ["==", "key1", "value1"], ["==", "key2", "value2"]]"#).unwrap(),
            r#"key0 = 'value0' OR key1 = 'value1' OR key2 = 'value2'"#
        );
    }
}
