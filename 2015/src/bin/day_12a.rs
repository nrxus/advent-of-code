use serde_json::Value;

fn solve(input: &str) -> i32 {
    let v: Value = serde_json::from_str(input).expect("invalid JSON");
    sum(&v)
}

fn sum(value: &Value) -> i32 {
    match value {
        Value::Array(values) => values.iter().map(|v| sum(v)).sum(),
        Value::Object(object) => object.values().map(|v| sum(v)).sum(),
        Value::Number(number) if number.is_i64() => number.as_i64().unwrap() as i32,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn array() {
        assert_eq!(solve(r#"[1,2,3]"#), 6);
    }

    #[test]
    fn object() {
        assert_eq!(solve(r#"{"a":2,"b":4}"#), 6);
    }

    #[test]
    fn nested_array() {
        assert_eq!(solve(r#"[[[3]]]"#), 3);
    }

    #[test]
    fn nested_object() {
        assert_eq!(solve(r#"{"a":{"b":4},"c":-1}"#), 3);
    }

    #[test]
    fn zero_sum_array() {
        assert_eq!(solve(r#"{"a":[-1,1]}"#), 0);
    }

    #[test]
    fn zero_sum_object() {
        assert_eq!(solve(r#"[-1,{"a":1}]"#), 0);
    }

    #[test]
    fn empty_array() {
        assert_eq!(solve(r#"[]"#), 0);
    }

    #[test]
    fn empty_object() {
        assert_eq!(solve(r#"{}"#), 0);
    }
}

common::read_main!();
