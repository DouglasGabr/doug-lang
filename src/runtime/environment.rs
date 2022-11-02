use std::collections::{hash_map::Entry, HashMap, HashSet};

use super::values::RuntimeValue;

pub struct Environment {
    parent: Option<Box<Environment>>,
    variables: HashMap<String, RuntimeValue>,
    constants: HashSet<String>,
}

impl Environment {
    pub fn new(parent_environment: Option<Environment>) -> Self {
        Self {
            parent: parent_environment.map(Box::new),
            variables: HashMap::new(),
            constants: HashSet::new(),
        }
    }

    pub fn declare_variable(
        &mut self,
        name: String,
        value: RuntimeValue,
        constant: bool,
    ) -> RuntimeValue {
        match self.variables.entry(name) {
            Entry::Vacant(entry) => {
                if constant {
                    self.constants.insert(entry.key().to_string());
                }
                *entry.insert(value)
            }
            Entry::Occupied(entry) => {
                panic!("Variable \"{}\" already declared", entry.key());
            }
        }
    }

    pub fn assign_variable(&mut self, name: &str, value: RuntimeValue) -> RuntimeValue {
        if let Some(variable) = self.variables.get_mut(name) {
            if self.constants.contains(name) {
                panic!("Cannot assign to constant variable \"{}\"", name);
            }
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
