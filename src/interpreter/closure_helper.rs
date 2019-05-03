use std::collections::HashMap;
use std::sync::Arc;

use ast::Type;
use interpreter::Interpreter;
use interpreter::runtime_stack::RuntimeStack;
use typed_ast::{LetEntry, TypedPattern};
use typed_ast::TypedDefinition;
use typed_ast::TypedExpr;
use types::Function;
use types::next_fun_id;
use types::Value;

impl Interpreter {
    pub fn create_lambda_closure(env: &mut RuntimeStack, ty: &Type, patterns: &Vec<TypedPattern>, expr: &TypedExpr) -> Value {
        let function = Arc::new(Function::Definition {
            id: next_fun_id(),
            patterns: patterns.clone(),
            expression: expr.clone(),
            captures: Self::extract_captures(env, &expr),
            function_type: ty.clone(),
        });

        Value::Fun {
            arg_count: patterns.len() as u32,
            args: vec![],
            fun: function,
        }
    }

    pub fn create_function_closure(env: &mut RuntimeStack, def: &TypedDefinition) -> Value {
        let function = Arc::new(Function::Definition {
            id: next_fun_id(),
            patterns: def.patterns.clone(),
            expression: def.expr.clone(),
            captures: Self::extract_captures(env, &def.expr),
            function_type: def.header.clone(),
        });

        Value::Fun {
            arg_count: def.patterns.len() as u32,
            args: vec![],
            fun: function,
        }
    }

    pub fn extract_captures(env: &mut RuntimeStack, expr: &TypedExpr) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        Self::traverse_expr(&mut map, env, expr);
        map
    }

    fn traverse_expr(result: &mut HashMap<String, Value>, env: &mut RuntimeStack, expr: &TypedExpr) {
        // TODO avoid capturing internal definitions
        match expr {
            TypedExpr::Ref(_, _, name) => {
                if let Some(value) = env.find(name) {
                    result.insert(name.to_string(), value);
                }
            }
            TypedExpr::Tuple(_, _, list)
            | TypedExpr::List(_, _, list) => {
                for expr in list {
                    Self::traverse_expr(result, env, expr);
                }
            }
            TypedExpr::Record(_, _, records)
            | TypedExpr::RecordUpdate(_, _, _, records) => {
                for (_, expr) in records {
                    Self::traverse_expr(result, env, expr);
                }
            }
            TypedExpr::RecordField(_, _, box_expr, _) => {
                Self::traverse_expr(result, env, box_expr.as_ref());
            }
            TypedExpr::If(_, _, a, b, c) => {
                Self::traverse_expr(result, env, a.as_ref());
                Self::traverse_expr(result, env, b.as_ref());
                Self::traverse_expr(result, env, c.as_ref());
            }
            TypedExpr::Application(_, _, a, b) => {
                Self::traverse_expr(result, env, a.as_ref());
                Self::traverse_expr(result, env, b.as_ref());
            }
            // TODO removed defined variables from captures, case, lambda and let
            TypedExpr::Case(_, _, a, entries) => {
                Self::traverse_expr(result, env, a.as_ref());
                for (_, expr) in entries {
                    Self::traverse_expr(result, env, expr);
                }
            }
            TypedExpr::Lambda(_, _, _, box_expr) => {
                Self::traverse_expr(result, env, box_expr.as_ref());
            }
            TypedExpr::Let(_, _, decls, box_expr) => {
                Self::traverse_expr(result, env, box_expr.as_ref());
                for decl in decls {
                    match decl {
                        LetEntry::Definition(def) => {
                            Self::traverse_expr(result, env, &def.expr);
                        }
                        LetEntry::Pattern(_, expr) => {
                            Self::traverse_expr(result, env, expr);
                        }
                    }
                }
            }
            _ => {
                // ignored
            }
        }
    }
}