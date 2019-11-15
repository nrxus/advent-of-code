use serde_json::Value;

fn solve(input: &str) -> i32 {
    let v: Value = serde_json::from_str(input).expect("invalid JSON");
    sum(&v)
}

fn sum(value: &Value) -> i32 {
    match value {
        Value::Array(values) => values.iter().map(|v| sum(v)).sum(),
        Value::Object(object) => {
            if object
                .values()
                .any(|v| *v == Value::String("red".to_string()))
            {
                0
            } else {
                object.values().map(|v| sum(v)).sum()
            }
        }
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
    fn inner_red_object() {
        assert_eq!(solve(r#"[1,{"c":"red","b":2},3]"#), 4);
    }

    #[test]
    fn everything_red() {
        assert_eq!(solve(r#"{"d":"red","e":[1,2,3,4],"f":5}"#), 0);
    }

    #[test]
    fn red_in_array() {
        assert_eq!(solve(r#"[1,"red",5]"#), 6);
    }
}

common::read_main!();
