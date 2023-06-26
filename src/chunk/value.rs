#[derive(Debug)]
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

impl Value {
    pub fn add(&self, right: &Value) -> Value {
        if self.type_eq(right) {
            match self {
                Value::Double(left) => {
                    Value::Double(left + right.get_double_val().unwrap())
                },
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

    pub fn type_eq(&self, other: &Value) -> bool {
        match self {
            Value::Double(_) => {
                match other {
                    Value::Double(_) => true,
                    _ => false,
                }
            },
            Value::Null => {
                match other {
                    Value::Null => true,
                    _ => false,
                }
            }
        }
    }
}

impl Copy for Value {}

impl Clone for Value {
    fn clone(&self) -> Self {
        *self
    }
}
