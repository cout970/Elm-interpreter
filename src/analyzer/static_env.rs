use std::collections::HashMap;
use types::Type;
use util::name_sequence::NameSequence;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticEnv {
    variables: Vec<HashMap<String, Type>>,
    pub name_seq: NameSequence,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            variables: vec![HashMap::new()],
            name_seq: NameSequence::new(),
        }
    }

    pub fn add(&mut self, name: &str, var: Type) {
        self.variables.last_mut().unwrap().insert(name.to_owned(), var);
    }

    pub fn replace(&mut self, name: &str, var: Type) {
        if self.variables.last().unwrap().contains_key(name) {
            self.variables.last_mut().unwrap().insert(name.to_owned(), var);
        }
    }

    pub fn find(&self, name: &str) -> Option<Type> {
        for map in self.variables.iter().rev() {
            let opt = map.get(name).cloned();

            if let Some(_) = &opt {
                return opt;
            }
        }

        None
    }

    pub fn next_name(&mut self) -> String {
        self.name_seq.next()
    }

    pub fn is_local(&self, name: &str) -> bool {
        self.variables.last().unwrap().get(name).is_some()
    }

    pub fn enter_block(&mut self) {
        self.variables.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.variables.pop().expect("Tried to pop the global environment");
    }
}