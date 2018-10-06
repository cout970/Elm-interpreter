use analyzer::type_analyzer::get_type;
use analyzer::type_analyzer::TypeError;
use std::collections::HashMap;
use types::Type;
use util::StringConversion;
use util::build_fun_type;

#[derive(Clone)]
pub struct Environment {
    blocks: Vec<HashMap<String, Type>>
}

pub fn default_lang_env() -> Environment {
    let mut env = Environment::new();

    env.add_def_type("True", &Type::Tag("Bool".s(), vec![]));
    env.add_def_type("False", &Type::Tag("Bool".s(), vec![]));

    env.add_def_type("::", &build_fun_type(&vec![
        Type::Var("a".s()), Type::Tag("List".s(), vec![Type::Var("a".s())]), Type::Tag("List".s(), vec![Type::Var("a".s())])
    ]));

    env.add_def_type("+", &build_fun_type(&vec![
        Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![])
    ]));
    env.add_def_type("-", &build_fun_type(&vec![
        Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![])
    ]));
    env.add_def_type("*", &build_fun_type(&vec![
        Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![])
    ]));
    env.add_def_type("/", &build_fun_type(&vec![
        Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![]), Type::Tag("number".s(), vec![])
    ]));

    env
}

impl Environment {
    pub fn new() -> Self {
        Self {
            blocks: vec![HashMap::new()],
        }
    }

    pub fn add_def_type(&mut self, name: &str, ty: &Type) {
        self.blocks.last_mut().unwrap().insert(name.to_owned(), ty.clone());
    }

    pub fn get_def_type(&self, name: &str) -> Option<Type> {
        for b in self.blocks.iter().rev() {
            let opt = b.get(name);

            if let Some(t) = opt {
                return Some(t.clone());
            }
        }

        None
    }

    pub fn enter_block(&mut self) {
        self.blocks.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        assert!(self.blocks.len() > 1, "Tried to exit the global block");
        self.blocks.pop().unwrap();
    }
}