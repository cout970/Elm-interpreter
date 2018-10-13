use std::collections::HashMap;
use types::Type;
use util::name_sequence::NameSequence;

#[derive(Clone, Debug)]
pub struct StaticEnv {
    variables: HashMap<String, Type>,
    pub name_seq: NameSequence,
    saved: Vec<Vec<String>>,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            name_seq: NameSequence::new(),
            saved: vec![vec![]],
        }
    }

    pub fn add(&mut self, name: &str, var: Type) {
        self.saved.last_mut().unwrap().push(name.to_owned());
        self.variables.insert(name.to_owned(), var);
    }

    pub fn find(&self, name: &str) -> Option<Type> {
        self.variables.get(name).cloned()
    }

    pub fn next_name(&mut self) -> String {
        self.name_seq.next()
    }

    pub fn is_local(&self, name: &str) -> bool {
        self.saved[self.saved.len()-1].iter().any(|n| n == name)
    }

    pub fn enter_block(&mut self) {
        self.saved.push(vec![]);
    }

    pub fn exit_block(&mut self) {
        let vec = self.saved.pop().expect("Tried to pop the global environment");
        for var in vec {
            self.variables.remove(&var);
        }
    }
}