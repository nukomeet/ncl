use object::Object;

#[derive(Debug, PartialEq)]
pub enum Value {
    Str(String),
    Num(i64),
    Bool(bool),
    // Array(Vec<Box<Value>>),
    Object(Box<Object>)
}

impl From<String> for Value {
    fn from(string: String) -> Self {
        Value::Str(string)
    }
}

impl<'a> From<&'a str> for Value {
    fn from(string: &'a str) -> Self {
        Value::Str(string.into())
    }
}

impl From<i64> for Value {
    fn from(num: i64) -> Self {
        Value::Num(num)
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<Object> for Value {
    fn from(obj: Object) -> Self {
        Value::Object(Box::new(obj))
    }
}

impl<'a> Into<Option<String>> for &'a Value {
    fn into(self) -> Option<String> {
        match self {
            &Value::Str(ref s) => Some(s.clone()),
            _ => None
        }
    }
}

impl<'a> Into<Option<i64>> for &'a Value {
    fn into(self) -> Option<i64> {
        match self {
            &Value::Num(n) => Some(n),
            _ => None
        }
    }
}

impl<'a> Into<Option<bool>> for &'a Value {
    fn into(self) -> Option<bool> {
        match self {
            &Value::Bool(b) => Some(b),
            _ => None
        }
    }
}
