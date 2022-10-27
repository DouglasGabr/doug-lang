use std::collections::{hash_map::Entry, HashMap};

use super::values::RuntimeValue;

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
}

impl Environment {
    pub fn new(parent_environment: Option<Environment>) -> Self {
        Self {
            parent: parent_environment.map(Box::new),
            variables: HashMap::new(),
        }
    }

    pub fn declare_variable(&mut self, name: String, value: RuntimeValue) -> RuntimeValue {
        match self.variables.entry(name) {
            Entry::Vacant(entry) => *entry.insert(value),
            Entry::Occupied(entry) => {
                panic!("Variable \"{}\" already declared", entry.key());
            }
        }
    }

    pub fn assign_variable(&mut self, name: &str, value: RuntimeValue) -> RuntimeValue {
        if let Some(variable) = self.variables.get_mut(name) {
            *variable = value;
            *variable
        } else if let Some(parent) = &mut self.parent {
            parent.assign_variable(name, value)
        } else {
            panic!("Variable \"{}\" not declared", name);
        }
    }

    pub fn lookup_variable(&self, name: &str) -> RuntimeValue {
        if let Some(variable) = self.variables.get(name) {
            *variable
        } else if let Some(parent) = &self.parent {
            parent.lookup_variable(name)
        } else {
            panic!("Variable \"{}\" not declared", name);
        }
    }
}
