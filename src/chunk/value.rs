pub enum Value {
    Double(f64),
    Null,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Double(d) => d.to_string(),
            Value::Null => String::from("null"),
        }
    }
}
