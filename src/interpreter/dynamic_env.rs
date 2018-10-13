use analyzer::static_env::StaticEnv;
use std::collections::HashMap;
use types::Type;
use types::Value;
use util::build_fun_type;
use util::builtin_fun_of;
use util::OptionExt;
use util::StringConversion;

#[derive(Clone)]
pub struct DynamicEnv {
    pub types: StaticEnv,
    values: HashMap<String, Value>,
    saved: Vec<Vec<String>>,
}

impl DynamicEnv {
    pub fn new() -> Self {
        DynamicEnv {
            types: StaticEnv::new(),
            values: HashMap::new(),
            saved: vec![vec![]],
        }
    }

    pub fn add(&mut self, name: &str, val: Value, ty: Type) {
        self.types.add(name, ty);
        self.values.insert(name.to_owned(), val);
        self.saved.last_mut().unwrap().push(name.to_owned());
    }

    pub fn find(&self, name: &str) -> Option<(Value, Type)> {
        self.values.get(name).map(|i| i.clone()).zip(self.types.find(name))
    }

    pub fn enter_block(&mut self) {
        self.types.enter_block();
        self.saved.push(vec![]);
    }

    pub fn exit_block(&mut self) {
        self.types.exit_block();
        let vec = self.saved.pop().expect("Tried to pop the global environment");
        for val in vec {
            self.values.remove(&val);
        }
    }

    pub fn default_lang_env() -> DynamicEnv {
        let mut env = DynamicEnv::new();


        let num_ty = build_fun_type(&vec![
            Type::Var("number".s()), Type::Var("number".s()), Type::Var("number".s())
        ]);
        let int_ty = build_fun_type(&vec![
            Type::Tag("Int".s(), vec![]), Type::Tag("Int".s(), vec![]), Type::Tag("Int".s(), vec![])
        ]);
        let float_ty = build_fun_type(&vec![
            Type::Tag("Float".s(), vec![]), Type::Tag("Float".s(), vec![]), Type::Tag("Float".s(), vec![])
        ]);

        env.add("+", builtin_fun_of(1, num_ty.clone()), num_ty.clone());
        env.add("-", builtin_fun_of(2, num_ty.clone()), num_ty.clone());
        env.add("*", builtin_fun_of(3, num_ty.clone()), num_ty.clone());
        env.add("/", builtin_fun_of(4, float_ty.clone()), float_ty.clone());
        env.add("//", builtin_fun_of(5, int_ty.clone()), int_ty.clone());

        env
    }
}