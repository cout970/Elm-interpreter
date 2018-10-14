use analyzer::static_env::StaticEnv;
use std::collections::HashMap;
use types::Type;
use types::Value;
use util::build_fun_type;
use util::builtin_fun_of;
use util::OptionExt;
use util::StringConversion;

#[derive(Clone, Debug, PartialEq)]
pub struct DynamicEnv {
    pub types: StaticEnv,
    values: Vec<HashMap<String, Value>>,
}

impl DynamicEnv {
    pub fn new() -> Self {
        DynamicEnv {
            types: StaticEnv::new(),
            values: vec![HashMap::new()],
        }
    }

    pub fn add(&mut self, name: &str, val: Value, ty: Type) {
        self.types.add(name, ty);
        self.values.last_mut().unwrap().insert(name.to_owned(), val);
    }

    pub fn find(&self, name: &str) -> Option<(Value, Type)> {
        for map in self.values.iter().rev() {
            let opt = map.get(name).cloned();
            if let Some(_) = &opt {
                return opt.zip(self.types.find(name));
            }
        }
        None
    }

    pub fn enter_block(&mut self) {
        println!("Enter block: {}", self.values.len());
        self.types.enter_block();
        self.values.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.types.exit_block();
        self.values.pop().expect("Tried to pop the global environment");
        println!("Exit block: {}", self.values.len());
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