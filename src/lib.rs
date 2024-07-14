use serde_json::Value;

pub fn convert(json_str: &str) -> String {
    let json: Value = serde_json::from_str(json_str).expect("Invalid JSON");
    to_elixir_map(&json)
}

fn to_elixir_map(value: &Value) -> String {
    match value {
        Value::Object(map) => {
            let elements: Vec<String> = map
                .iter()
                .map(|(k, v)| format!("{:?} => {}", k, to_elixir_map(v)))
                .collect();

            format!("%{{{}}}", elements.join(", "))
        }
        Value::Array(list) => {
            let elements: Vec<String> = list.iter().map(|el| to_elixir_map(el)).collect();

            format!("[{}]", elements.join(", "))
        }
        Value::Null => "nil".to_string(),
        Value::String(s) => format!("{:?}", s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
    }
}
