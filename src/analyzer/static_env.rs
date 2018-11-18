use ast::Type;
use std::collections::HashMap;
use types::Adt;
use util::name_sequence::NameSequence;
use std::sync::Arc;
use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use types::AdtVariant;

#[derive(Clone, PartialEq)]
pub struct StaticEnv {
    blocks: Vec<Block>,
    pub name_seq: NameSequence,
}

#[derive(Clone, Debug, PartialEq)]
struct Block {
    functions: HashMap<String, Type>,
    alias: HashMap<String, Type>,
    adts: HashMap<String, Arc<Adt>>,
    adt_variants: HashMap<String, Arc<Adt>>,
}

impl StaticEnv {
    pub fn new() -> Self {
        Self {
            blocks: vec![
                Block {
                    functions: HashMap::new(),
                    alias: HashMap::new(),
                    adts: HashMap::new(),
                    adt_variants: HashMap::new(),
                }
            ],
            name_seq: NameSequence::new(),
        }
    }

    pub fn replace(&mut self, name: &str, var: Type) {
        let block = self.blocks.last_mut().unwrap();

        if block.functions.contains_key(name) {
            block.functions.insert(name.to_owned(), var);
        }
    }

    pub fn add_definition(&mut self, name: &str, var: Type) {
        let block = self.blocks.last_mut().unwrap();

        block.functions.insert(name.to_owned(), var);
    }

    pub fn find_definition(&self, name: &str) -> Option<Type> {
        self.search(name, |block| &block.functions)
    }

    pub fn add_alias(&mut self, name: &str, var: Type) {
        let block = self.blocks.last_mut().unwrap();

        block.alias.insert(name.to_owned(), var);
    }

    pub fn find_alias(&self, name: &str) -> Option<Type> {
        self.search(name, |block| &block.alias)
    }

    pub fn add_adt(&mut self, name: &str, var: Arc<Adt>) {
        let block = self.blocks.last_mut().unwrap();

        block.adts.insert(name.to_owned(), var.clone());
        // variants for reverse adt lookup
        for AdtVariant{ name, .. } in &var.variants {
            block.adt_variants.insert(name.to_owned(), var.clone());
        }
    }

    pub fn find_adt(&self, name: &str) -> Option<Arc<Adt>> {
        self.search(name, |block| &block.adts)
    }

    pub fn find_adt_variant(&self, name: &str) -> Option<Arc<Adt>> {
        self.search(name, |block| &block.adt_variants)
    }

    fn search<T: Clone, F: Fn(&Block)->&HashMap<String, T>>(&self, name: &str, func: F) -> Option<T> {
        for block in self.blocks.iter().rev() {
            let opt = func(block).get(name).cloned();

            if let Some(_) = &opt {
                return opt;
            }
        }

        None
    }

    pub fn next_name(&mut self) -> String {
        self.name_seq.next()
    }

    pub fn is_local(&self, name: &str) -> bool {
        let block = self.blocks.last().unwrap();

        block.functions.get(name).is_some()
    }

    pub fn enter_block(&mut self) {
        self.blocks.push(Block {
            functions: HashMap::new(),
            alias: HashMap::new(),
            adts: HashMap::new(),
            adt_variants: HashMap::new(),
        });
    }

    pub fn exit_block(&mut self) {
        self.blocks.pop().expect("Tried to pop the global environment");
    }
}

impl Debug for StaticEnv {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "StaticEnv:\nname_seq: {:?}\n", self.name_seq)?;
        for i in 0..self.blocks.len() {
            let spaces = " ".repeat(i);
            let block = &self.blocks[i];

            for (name, ty) in &block.alias {
                write!(f, "{}Alias '{}' : {}\n", spaces, name, ty)?;
            }

            for (name, adt) in &block.adts {
                write!(f, "{}Adt '{}' : {:?}\n", spaces, name, adt)?;
            }

            for (name, ty) in &block.functions {
                write!(f, "{}Def '{}' : {}\n", spaces, name, ty)?;
            }
        }
        Ok(())
    }
}