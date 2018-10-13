use std::collections::HashMap;
use types::Type;

#[derive(Clone)]
pub struct StaticEnv {
    variables: HashMap<String, Type>,
    saved: Vec<Vec<String>>,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
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