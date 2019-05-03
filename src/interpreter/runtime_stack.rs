use std::collections::HashMap;
use std::fmt::Write;

use types::Value;

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

    pub fn debug(&self) -> String {
        let mut msg = String::new();

        for (i, frame) in self.frames.iter().enumerate() {
            writeln!(&mut msg, "Frame #{}", i).unwrap();
            for (name, value) in &frame.values {
                writeln!(&mut msg, "  {} = {}", name, value).unwrap();
            }
        }

        msg
    }
}