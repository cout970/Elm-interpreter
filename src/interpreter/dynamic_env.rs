use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;

use analyzer::static_env::StaticEnv;
use ast::Type;
use Runtime;
use types::ExternalFunc;
use types::FunCall;
use types::Value;
use util::build_fun_type;
use util::builtin_fun_of;
use util::OptionExt;
use util::StringConversion;

#[derive(Clone, Debug)]
pub struct RuntimeStack {
    frames: Vec<StackFrame>
}

#[derive(Clone, Debug)]
struct StackFrame {
    values: HashMap<String, Value>
}

impl RuntimeStack {
    pub fn new() -> Self {
        RuntimeStack {
            frames: vec![StackFrame { values: HashMap::new() }]
        }
    }

    pub fn add(&mut self, name: &str, val: Value) {
        self.frames.last_mut().unwrap().values.insert(name.to_owned(), val);
    }

    pub fn find(&self, name: &str) -> Option<Value> {
        for frame in self.frames.iter().rev() {
            let opt = frame.values.get(name);
            if opt.is_some() {
                return opt.cloned();
            }
        }
        None
    }

    pub fn enter_block(&mut self) {
        self.frames.push(StackFrame { values: HashMap::new() });
    }

    pub fn exit_block(&mut self) {
        self.frames.pop().expect("Tried to pop all the stack frames!");
    }
}

#[derive(Clone, PartialEq)]
pub struct DynamicEnv {
    pub types: StaticEnv,
    values: Vec<HashMap<String, Value>>,
    cache: HashMap<FunCall, Value>,
}

impl DynamicEnv {
    pub fn new() -> Self {
        DynamicEnv {
            types: StaticEnv::new(),
            values: vec![HashMap::new()],
            cache: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: &str, val: Value, ty: Type) {
        self.types.add_definition(name, ty);
        self.values.last_mut().unwrap().insert(name.to_owned(), val);
    }

    pub fn find(&self, name: &str) -> Option<(Value, Type)> {
        for map in self.values.iter().rev() {
            let opt = map.get(name).cloned();
            if let Some(_) = &opt {
                return opt.zip(self.types.find_definition(name));
            }
        }
        None
    }

    pub fn enter_block(&mut self) {
        self.types.enter_block();
        self.values.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.types.exit_block();
        self.values.pop().expect("Tried to pop the global environment");
    }

    pub fn get_from_cache(&self, call: &FunCall) -> Option<Value> {
        self.cache.get(call).cloned()
    }

    pub fn add_to_cache(&mut self, call: FunCall, value: Value) {
        self.cache.insert(call, value);
    }

    pub fn default_lang_env() -> DynamicEnv {
        let env = DynamicEnv::new();
        env
    }
}

impl Debug for DynamicEnv {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "DynamicEnv{{\ntypes: {:?},\nvalues:{:#?},\ncache: {:?}\n}}", self.types, self.values, self.cache)
    }
}