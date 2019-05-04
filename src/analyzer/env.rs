use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use ast::{Type, TypeAlias};
use util::name_sequence::NameSequence;

#[derive(Debug, Clone)]
pub struct Env {
    blocks: Vec<HashMap<String, Type>>,
    type_alias: HashMap<String, TypeAlias>,
    canonical_type_names: HashMap<String, String>,
    generator: NameSequence,
    number_gen: NameSequence,
    save: Vec<(u32, u32)>,
}

impl Env {
    pub fn new() -> Self {
        Env {
            blocks: vec![HashMap::new()],
            type_alias: HashMap::new(),
            canonical_type_names: HashMap::new(),
            generator: NameSequence::new(),
            number_gen: NameSequence::new(),
            save: vec![],
        }
    }

    pub fn set_type_alias(&mut self, alias: TypeAlias) {
        self.type_alias.insert(alias.name.clone(), alias);
    }

    pub fn get_type_alias(&self, name: &str) -> Option<&TypeAlias> {
        self.type_alias.get(name)
    }

    pub fn set_canonical_type_name(&mut self, name: &str, canonical: String) {
        self.canonical_type_names.insert(name.to_string(), canonical);
    }

    pub fn get_canonical_type_name(&self, name: &str) -> Option<&str> {
        self.canonical_type_names.get(name).map(|it| it.as_str())
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        for block in self.blocks.iter().rev() {
            if let Some(ty) = block.get(name) {
                return Some(ty);
            }
        }

        None
    }

    pub fn set(&mut self, name: &str, ty: Type) {
        self.blocks.last_mut().unwrap().insert(name.to_string(), ty);
    }

    pub fn next_type(&mut self) -> Type {
        Type::Var(self.generator.next())
    }

    pub fn next_number_type(&mut self) -> Type {
        Type::Var(self.number_gen.next_with_prefix("number"))
    }

    pub fn next_comparable_type(&mut self) -> Type {
        Type::Var(self.number_gen.next_with_prefix("comparable"))
    }

    pub fn next_appendable_type(&mut self) -> Type {
        Type::Var(self.number_gen.next_with_prefix("appendable"))
    }

    pub fn block<T, F, E>(&mut self, mut func: F) -> Result<T, E>
        where F: FnMut(&mut Self) -> Result<T, E>
    {
        self.enter_block();
        let i = func(self);
        self.exit_block();
        i
    }

    pub fn enter_block(&mut self) {
        self.blocks.push(HashMap::new());
    }

    pub fn exit_block(&mut self) {
        self.blocks.pop().expect("Tried to pop the global environment");
    }
}

impl Display for Env {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        writeln!(f, "Env:")?;
        for (i, block) in self.blocks.iter().enumerate() {
            let mut pad = String::new();

            for _ in 0..(i * 2) {
                pad.push(' ');
            }

            writeln!(f, "# Block {}", i)?;
            for (k, v) in block {
                writeln!(f, "{}{} => {}", pad, k, v)?;
            }
        }
        Ok(())
    }
}