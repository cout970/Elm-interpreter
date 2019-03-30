use std::collections::HashMap;

use analyzer::Analyzer;
use analyzer::dependency_sorter::sort_statements;
use ast::AdtExposing;
use ast::Exposing;
use ast::Import;
use ast::ModuleExposing;
use ast::Statement;
use ast::Type;
use errors::ElmError;
use errors::LoaderError;
use errors::RuntimeError;
use errors::TypeError;
use loader::Declaration;
use loader::LoadedModule;

impl Analyzer {
    pub fn analyze_module_imports(&mut self, modules: &HashMap<String, LoadedModule>, imports: &Vec<Import>) -> Result<(), ElmError> {
        for import in imports {
            let name = import.path.join(".");
            let module = modules.get(&name)
                .ok_or(ElmError::Loader { info: LoaderError::MissingImport { name } })?;

            match (&import.alias, &import.exposing) {
                (None, Some(me)) => {
                    let decls = match me {
                        ModuleExposing::Just(exp) => {
                            Self::get_exposed_decls(&module.declarations, exp)
                                .map_err(|e| ElmError::Interpreter { info: e })?
                        }
                        ModuleExposing::All => {
                            module.declarations.clone()
                        }
                    };

                    for decl in &decls {
                        match decl {
                            Declaration::Def(name, ty) => self.env.add_definition(name, ty.clone()),
                            Declaration::Alias(name, ty) => self.env.add_alias(name, ty.clone()),
                            Declaration::Adt(name, adt) => self.env.add_adt(name, adt.clone()),
                        }
                    }
                }
                (Some(_), None) => {
                    // TODO alias import
                    unimplemented!()
                }
                _ => {
                    panic!("Invalid combination of alias and exposing for import: {:?}", import)
                }
            }
        }
        Ok(())
    }

    pub fn analyze_module_declarations(&mut self, statements: &Vec<Statement>) -> Result<Vec<Declaration>, Vec<TypeError>> {
        let statements = sort_statements(statements)
            .map_err(|e| vec![TypeError::CyclicStatementDependency(e)])?;

        let mut declarations = vec![];
        let mut errors = vec![];

        for stm in statements {
            match self.analyze_statement(stm) {
                Ok(decls) => {
                    for decl in decls.into_iter() {
                        declarations.push(decl.clone());
                        match decl {
                            Declaration::Def(name, ty) => {
                                self.env.add_definition(&name, ty);
                            }
                            Declaration::Alias(name, ty) => {
                                self.env.add_alias(&name, ty);
                            }
                            Declaration::Adt(name, adt) => {
                                self.env.add_adt(&name, adt);
                            }
                        }
                    }
                }
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        if errors.is_empty() {
            Ok(declarations)
        } else {
            Err(errors)
        }
    }

    fn get_exposed_decls(all_decls: &Vec<Declaration>, exposed: &Vec<Exposing>) -> Result<Vec<Declaration>, RuntimeError> {
        let mut exposed_decls = Vec::new();

        for exp in exposed.iter() {
            match exp {
                Exposing::Adt(name, adt_exp) => {
                    match adt_exp {
                        AdtExposing::Variants(variants) => {
                            for it in all_decls.iter() {
                                if let Declaration::Def(variant_name, ty) = it {
                                    if variants.contains(variant_name) {
                                        if let Type::Tag(tag_name, _) = Self::get_fun_return(ty) {
                                            if &tag_name == name {
                                                exposed_decls.push(it.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        AdtExposing::All => {
                            for it in all_decls.iter() {
                                if let Declaration::Def(_, ty) = it {
                                    if let Type::Tag(tag_name, _) = Self::get_fun_return(ty) {
                                        if &tag_name == name {
                                            exposed_decls.push(it.clone());
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let decl = all_decls.iter()
                        .find(|decl| {
                            if let Declaration::Adt(adt_name, _) = decl {
                                adt_name == name
                            } else {
                                false
                            }
                        })
                        .map(|decl| decl.clone())
                        .ok_or_else(|| RuntimeError::MissingExposing(name.clone(), all_decls.clone()))?;

                    exposed_decls.push(decl);
                }
                Exposing::Type(name) => {
                    let decl = all_decls.iter()
                        .find(|decl| {
                            if let Declaration::Alias(alias_name, _) = decl {
                                alias_name == name
                            } else if let Declaration::Adt(adt_name, _) = decl {
                                adt_name == name
                            } else {
                                false
                            }
                        })
                        .map(|decl| decl.clone())
                        .ok_or_else(|| RuntimeError::MissingExposing(name.clone(), all_decls.clone()))?;

                    exposed_decls.push(decl);
                }
                Exposing::BinaryOperator(name) => {
                    let decl = all_decls.iter()
                        .find(|decl| {
                            if let Declaration::Def(def_name, _) = decl {
                                def_name == name
                            } else {
                                false
                            }
                        })
                        .map(|decl| decl.clone());

                    if let Some(decl) = decl {
                        exposed_decls.push(decl);
                    }
                }
                Exposing::Definition(name) => {
                    let decl = all_decls.iter()
                        .find(|decl| {
                            if let Declaration::Def(def_name, _) = decl {
                                def_name == name
                            } else {
                                false
                            }
                        })
                        .map(|decl| decl.clone())
                        .ok_or_else(|| RuntimeError::MissingExposing(name.clone(), all_decls.clone()))?;

                    exposed_decls.push(decl);
                }
            }
        }

        Ok(exposed_decls)
    }

    fn get_fun_return(ty: &Type) -> Type {
        if let Type::Fun(_, ty) = ty {
            Self::get_fun_return(&*ty)
        } else {
            ty.clone()
        }
    }
}