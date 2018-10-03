use std::collections::HashMap;
use types::Type;
use analyzer::type_analyzer::get_type;
use analyzer::type_analyzer::TypeError;

#[derive(Clone)]
pub struct StaticEnv {
    adts: HashMap<String, Type>,
    defs: HashMap<String, Type>,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            adts: HashMap::new(),
            defs: HashMap::new(),
        }
    }

    pub fn add_def_type(&mut self, name: &str, ty: &Type) {
        self.defs.insert(name.to_owned(), ty.clone());
    }

    pub fn add_adt_type(&mut self, name: &str, ty: &Type) {
        self.adts.insert(name.to_owned(), ty.clone());
    }

    pub fn get_def_type(&self, name: &str) -> Option<Type> {
        self.defs.get(name).map(|t| t.clone())
    }

    pub fn get_adt_type(&self, name: &str) -> Option<Type> {
        self.adts.get(name).map(|t| t.clone())
    }
}