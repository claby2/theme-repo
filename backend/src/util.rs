pub fn convert(toml: toml::Value) -> serde_json::Value {
    match toml {
        toml::Value::String(s) => serde_json::Value::String(s),
        toml::Value::Integer(i) => serde_json::Value::Number(i.into()),
        toml::Value::Float(f) => {
            let n = serde_json::Number::from_f64(f).expect("float infinite and nan not allowed");
            serde_json::Value::Number(n)
        }
        toml::Value::Boolean(b) => serde_json::Value::Bool(b),
        toml::Value::Array(arr) => serde_json::Value::Array(arr.into_iter().map(convert).collect()),
        toml::Value::Table(table) => {
            serde_json::Value::Object(table.into_iter().map(|(k, v)| (k, convert(v))).collect())
        }
        toml::Value::Datetime(dt) => serde_json::Value::String(dt.to_string()),
    }
}
