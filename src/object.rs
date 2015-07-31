use super::Key;
use value::Value;

use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Object {
    values: HashMap<Key, Value>
}

impl Object {
    pub fn get<T: Into<String>>(&self, key: T) -> Option<&Value> {
        self.values.get(&key.into())
    }
}

impl From<Vec<(Key, Value)>> for Object {
    fn from(vec: Vec<(Key, Value)>) -> Self {
        let mut obj = Object { values: HashMap::with_capacity(vec.len()) };

        for elem in vec {
            obj.values.insert(elem.0, elem.1);
        }

        obj
    }
}

