use std::collections::HashMap;

use pretty::RcDoc;
use serde_json::Value as JsonValue;

pub struct ConvertOptions {
    pub pretty: Option<bool>,
    pub atom_keys: Option<bool>,
}

pub fn convert(json_str: &str, opts: ConvertOptions) -> String {
    let json: JsonValue = serde_json::from_str(json_str).expect("Invalid JSON");
    let elixir_map = ElixirValue::from_json(&json, opts.atom_keys.unwrap());
    if opts.pretty.unwrap() {
        elixir_map.to_pretty(5)
    } else {
        elixir_map.to_pretty(5)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ElixirKey {
    Atom(String),
    Str(String),
}

#[derive(Debug, Clone)]
enum ElixirValue {
    Map(HashMap<ElixirKey, ElixirValue>),
    List(Vec<ElixirValue>),
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
}

// TODO:
// - [ ] why is this not breaking the lines when the width is only 5
// - [ ] try this library instead: https://crates.io/crates/elegance

impl ElixirKey {
    pub fn to_doc(&self) -> RcDoc {
        match *self {
            ElixirKey::Atom(ref atom) => {
                // protect against atoms that contain illegal atom syntax by wrapping them with a
                // string
                let atom_doc = if atom.contains(" ") {
                    RcDoc::text("\"")
                        .append(RcDoc::as_string(atom))
                        .append(RcDoc::text("\""))
                } else {
                    RcDoc::as_string(atom)
                };

                atom_doc.append(RcDoc::text(":")).append(RcDoc::space())
            }

            ElixirKey::Str(ref str) => RcDoc::text("\"")
                .append(RcDoc::as_string(str))
                .append(RcDoc::text("\""))
                .append(RcDoc::space())
                .append(RcDoc::text("=>"))
                .append(RcDoc::space()),
        }
    }
}

impl ElixirValue {
    fn from_json(value: &JsonValue, atom_keys: bool) -> Self {
        match value {
            JsonValue::Object(map) => {
                let mut elixir_map = HashMap::new();
                for (k, v) in map {
                    let key = if atom_keys {
                        ElixirKey::Atom(k.clone())
                    } else {
                        ElixirKey::Str(k.clone())
                    };

                    let value = ElixirValue::from_json(v, atom_keys);
                    elixir_map.insert(key, value);
                }
                ElixirValue::Map(elixir_map)
            }
            JsonValue::Array(vec) => {
                let elixir_vec = vec
                    .iter()
                    .map(|v| ElixirValue::from_json(v, atom_keys))
                    .collect();
                ElixirValue::List(elixir_vec)
            }
            JsonValue::String(s) => ElixirValue::Str(s.clone()),
            JsonValue::Number(num) => {
                if let Some(i) = num.as_i64() {
                    ElixirValue::Int(i)
                } else if let Some(f) = num.as_f64() {
                    ElixirValue::Float(f)
                } else {
                    panic!("Unexpected number type")
                }
            }
            JsonValue::Bool(b) => ElixirValue::Bool(*b),
            JsonValue::Null => ElixirValue::Null,
        }
    }

    pub fn to_doc(&self) -> RcDoc {
        match *self {
            ElixirValue::Str(ref str) => {
                RcDoc::text("\"").append(RcDoc::as_string(str)).append("\"")
            }

            ElixirValue::Map(ref map) => RcDoc::text("%{")
                .append(
                    RcDoc::intersperse(
                        map.into_iter().map(|(k, v)| k.to_doc().append(v.to_doc())),
                        RcDoc::text(",").append(RcDoc::space()),
                    )
                    .nest(2)
                    .group(),
                )
                .append(RcDoc::text("}")),

            ElixirValue::List(ref list) => RcDoc::text("[")
                .append(
                    RcDoc::intersperse(
                        list.into_iter().map(|v| v.to_doc()),
                        RcDoc::text(",").append(RcDoc::space()),
                    )
                    .nest(2)
                    .group(),
                )
                .append(RcDoc::text("]")),

            ElixirValue::Bool(ref bool) => RcDoc::as_string(bool),

            ElixirValue::Int(ref int) => RcDoc::as_string(int),

            ElixirValue::Float(ref float) => RcDoc::as_string(float),

            ElixirValue::Null => RcDoc::text("nil"),
        }
    }

    pub fn to_pretty(&self, width: usize) -> String {
        let mut w = Vec::new();
        self.to_doc().render(width, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}
