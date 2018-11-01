use std::collections::HashMap;
use ast::Type;
use util::name_sequence::NameSequence;

#[derive(Clone, Debug, PartialEq)]
pub struct StaticEnv {
    functions: Vec<HashMap<String, Type>>,
    types: Vec<HashMap<String, Type>>,
    pub name_seq: NameSequence,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            functions: vec![HashMap::new()],
            types: vec![HashMap::new()],
            name_seq: NameSequence::new(),
        }
    }

    pub fn add(&mut self, name: &str, var: Type) {
        self.functions.last_mut().unwrap().insert(name.to_owned(), var);
    }

    pub fn replace(&mut self, name: &str, var: Type) {
        if self.functions.last().unwrap().contains_key(name) {
            self.functions.last_mut().unwrap().insert(name.to_owned(), var);
        }
    }

    pub fn find(&self, name: &str) -> Option<Type> {
        for map in self.functions.iter().rev() {
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
        self.functions.last().unwrap().get(name).is_some()
    }

    pub fn enter_block(&mut self) {
        self.functions.push(HashMap::new());
        self.types.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.functions.pop().expect("Tried to pop the global environment");
        self.types.pop().expect("Tried to pop the global environment");
    }
}