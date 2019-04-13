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