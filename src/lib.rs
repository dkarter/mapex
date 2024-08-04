use serde_json::Value;

pub struct ConvertOptions {
    pub pretty: Option<bool>,
}

pub fn convert(json_str: &str, opts: ConvertOptions) -> String {
    let json: Value = serde_json::from_str(json_str).expect("Invalid JSON");
    if opts.pretty.unwrap() {
        to_pretty_elixir_map(&json, 0)
    } else {
        to_elixir_map(&json)
    }
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

fn to_pretty_elixir_map(value: &Value, indent: usize) -> String {
    match value {
        Value::Object(map) => {
            let outer_indent = create_indent(indent);
            let inner_indent = create_indent(indent + 2);

            let elements: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{}{:?} => {}",
                        inner_indent,
                        k,
                        to_pretty_elixir_map(v, indent + 2)
                    )
                })
                .collect();

            // TODO: this can be more intelligent, and only break new lines when the map is a
            // certain size, but this is good enough for a first pass
            format!("%{{\n{}\n{}}}", elements.join(", "), outer_indent)
        }
        Value::Array(list) => {
            let elements: Vec<String> = list
                .iter()
                .map(|el| to_pretty_elixir_map(el, indent))
                .collect();

            format!("[{}]", elements.join(", "))
        }
        Value::Null => "nil".to_string(),
        Value::String(s) => format!("{:?}", s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
    }
}

fn create_indent(size: usize) -> String {
    std::iter::repeat(' ').take(size).collect()
}
