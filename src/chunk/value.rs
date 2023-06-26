#[derive(Debug)]
pub enum Value {
    Double(f64),
    String(String),
    Null,
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Double(d) => d.to_string(),
            Value::Null => String::from("null"),
            Value::String(s) => s.clone(),
        }
    }
}

impl Value {
    pub fn add(&self, right: &Value) -> Value {
        if self.type_eq(right) {
            match self {
                Value::Double(left) => {
                    Value::Double(left + right.get_double_val().unwrap())
                },
                Value::String(s) => Value::String(format!("{}{}", s, right.get_string_val().unwrap())),
                Value::Null => Value::Null,
            }
        }else {
            Value::Null
        }
    }

    pub fn sub(&self, right: &Value) -> Value {
        if self.type_eq(right) {
            match self {
                Value::Double(left) => {
                    Value::Double(left - right.get_double_val().unwrap())
                },
                Value::Null => Value::Null,
                Value::String(_) => Value::Null,
            }
        }else {
            Value::Null
        }
    }

    pub fn multiply(&self, right: &Value) -> Value {
        if self.type_eq(right) {
            match self {
                Value::Double(left) => {
                    Value::Double(left * right.get_double_val().unwrap())
                },
                Value::Null => Value::Null,
                Value::String(_) => Value::Null,
            }
        }else {
            Value::Null
        }
    }

    pub fn divide(&self, right: &Value) -> Value {
        if self.type_eq(right) {
            match self {
                Value::Double(left) => {
                    let a = right.get_double_val().unwrap();
                    if a == 0.0 {
                        Value::Null
                    }else {
                        Value::Double(left / a)
                    }
                },
                Value::Null => Value::Null,
                Value::String(_) => Value::Null,
            }
        }else {
            Value::Null
        }
    }

    fn get_double_val(&self) -> Option<f64> {
        match self {
            Value::Double(v) => Some(*v),
            _ => None,
        }
    }

    fn get_string_val(&self) -> Option<String> {
        match self {
            Value::String(s) => Some(s.clone()),
            _ => None,
        }
    }

    pub fn type_eq(&self, other: &Value) -> bool {
        self.to_u32() == other.to_u32()
    }

    fn to_u32(&self) -> u32 {
        match self {
            Value::Double(_) => 0,
            Value::Null => 1,
            Value::String(_) => 2,
        }
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match self {
            Value::Double(d) => Value::Double(*d),
            Value::String(s) => Value::String(s.clone()),
            Value::Null => Value::Null,
        }
    }
}
