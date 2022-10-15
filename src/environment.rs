use std::{cell::RefCell, collections::HashMap};

use crate::interpreter::LoxValue;

pub struct Environment {
    values: RefCell<HashMap<String, LoxValue>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn define(&self, name: String, value: Option<LoxValue>) {
        if let Some(val) = value {
            self.values.borrow_mut().insert(name, val);
        }
    }

    pub fn get(&self, name: &str) -> Option<LoxValue> {
        let values = self.values.borrow();
        return match values.get(name) {
            Some(val) => Some(val.clone()),
            _ => panic!("Undefined variable {}.", name),
        };
    }

    pub fn assign(&self, name: String, value: &LoxValue) {
        let mut values = self.values.borrow_mut();
        match values.get(&name) {
            Some(_) => {
                values.insert(name, value.clone());
            }
            _ => panic!("Undefined variable {}.", name),
        }
    }
}
