use std::collections::HashMap;
use std::sync::Arc;

use analyzer::type_check_function;
use ast::*;
use ast::Definition;
use errors::*;
use interpreter::builtins::builtin_adt_constructor;
use interpreter::dynamic_env::DynamicEnv;
use interpreter::expression_eval::eval_expr;
use types::Adt;
use types::AdtVariant;
use types::Function;
use types::next_fun_id;
use types::Value;
use util::build_fun_type;
use util::create_vec_inv;
use util::qualified_name;

pub fn eval_stm(env: &mut DynamicEnv, stm: &Statement) -> Result<Option<Value>, RuntimeError> {
    match stm {
        Statement::Alias(name, _, ty) => {
            env.types.add_definition(name, ty.clone());
        }
        Statement::Adt(name, vars, variants) => {
            let type_vars: Vec<Type> = vars.iter()
                .map(|v| Type::Var(v.to_owned()))
                .collect();

            let variant_data = variants.iter().map(|(name, types)| {
                AdtVariant {
                    name: name.clone(),
                    types: types.clone(),
                }
            }).collect();

            let adt = Arc::new(Adt {
                name: name.clone(),
                types: vars.clone(),
                variants: variant_data,
            });

            let ty = Type::Tag(name.clone(), type_vars);

            env.types.add_definition(name, ty.clone());

            for (var_name, params) in variants {
                let var_ty = build_fun_type(&create_vec_inv(params, ty.clone()));

                let value = if params.is_empty() {
                    Value::Adt(var_name.clone(), vec![], adt.clone())
                } else {
                    let fun_ty = Type::Fun(
                        Box::from(Type::Tag(var_name.clone(), vec![])),
                        Box::from(var_ty.clone()),
                    );

                    Value::Fun {
                        args: vec![Value::Adt(var_name.clone(), vec![], adt.clone())],
                        arg_count: (params.len() + 1) as u32,
                        // TODO add captures?
                        captures: HashMap::new(),
                        fun: Arc::new(Function::External(next_fun_id(), builtin_adt_constructor(), fun_ty)),
                    }
                };

                env.add(var_name, value, var_ty);
            }
        }
        Statement::Port(_name, _ty) => {
            // TODO
        }
        Statement::Infix(_, _, _, _) => {
            // TODO
        }
        Statement::Def(def) => {
            let def_ty = type_check_function(&mut env.types, def)
                .map_err(|e| RuntimeError::IncorrectDefType(e))?;

            let Definition { name, patterns, expr, .. } = &def;

            let value = Value::Fun {
                args: vec![],
                arg_count: patterns.len() as u32,
                captures: extract_captures(env, expr),
                fun: Arc::new(Function::Expr(next_fun_id(), patterns.clone(), expr.clone(), def_ty.clone())),
            };

            let ret = if patterns.len() == 0 { eval_expr(env, expr)? } else { value };

            env.add(name, ret.clone(), def_ty);

            return Ok(Some(ret));
        }
    }
    Ok(None)
}

pub fn extract_captures(env: &mut DynamicEnv, expr: &Expr) -> HashMap<String, Value> {
    let mut map = HashMap::new();
    traverse_expr(&mut map, env, expr);
    dbg!(map)
}

fn traverse_expr(result: &mut HashMap<String, Value>, env: &mut DynamicEnv, expr: &Expr) {
    match expr {
        Expr::QualifiedRef(_, path, name) => {
            let full_name = qualified_name(path, name);

            if let Some(value) = env.find(name).map(|(val, _)| val) {
                result.insert(full_name, value);
            }
        }
        Expr::Ref(_, name) => {
            if let Some(value) = env.find(name).map(|(val, _)| val) {
                result.insert(name.to_string(), value);
            }
        }
        Expr::OpChain(_, list, _)
        | Expr::Tuple(_, list)
        | Expr::List(_, list) => {
            for expr in list {
                traverse_expr(result, env, expr);
            }
        }
        Expr::Record(_, records)
        | Expr::RecordUpdate(_, _, records) => {
            for (_, expr) in records {
                traverse_expr(result, env, expr);
            }
        }
        Expr::RecordField(_, box_expr, _) => {
            traverse_expr(result, env, box_expr.as_ref());
        }
        Expr::If(_, a, b, c) => {
            traverse_expr(result, env, a.as_ref());
            traverse_expr(result, env, b.as_ref());
            traverse_expr(result, env, c.as_ref());
        }
        Expr::Case(_, a, entries) => {
            traverse_expr(result, env, a.as_ref());
            for (_, expr) in entries {
                traverse_expr(result, env, expr);
            }
        }
        Expr::Lambda(_, _, box_expr) => {
            traverse_expr(result, env, box_expr.as_ref());
        }
        Expr::Application(_, a, b) => {
            traverse_expr(result, env, a.as_ref());
            traverse_expr(result, env, b.as_ref());
        }
        Expr::Let(_, decls, box_expr) => {
            traverse_expr(result, env, box_expr.as_ref());
            for decl in decls {
                match decl {
                    LetDeclaration::Def(_) => {
                        // TODO
                    }
                    LetDeclaration::Pattern(_, expr) => {
                        traverse_expr(result, env, expr);
                    }
                }
            }
        }
        _ => {
            // ignored
        }
    }
}

#[cfg(test)]
mod tests {
    use analyzer::type_of_value;
    use parsers::from_code;
    use parsers::from_code_stm;
    use util::StringConversion;

    use super::*;

    fn formatted(env: &mut DynamicEnv, stm: &Statement) -> String {
        let result = eval_stm(env, stm);
        let option = result.unwrap();
        let value = option.unwrap();
        let ty = type_of_value(&value);

        format!("{} : {}", value, ty)
    }

    fn formatted_expr(env: &mut DynamicEnv, expr: &Expr) -> String {
        let result = eval_expr(env, expr);
        let value = result.unwrap();
        let ty = type_of_value(&value);

        format!("{} : {}", value, ty)
    }

    #[test]
    fn check_constant() {
        let stm = from_code_stm(b"x = 1");
        let mut env = DynamicEnv::new();

        assert_eq!(formatted(&mut env, &stm), "1 : number".s());
    }

    #[test]
    fn check_identity() {
        let stm = from_code_stm(b"id value = value");
        let mut env = DynamicEnv::new();

        assert_eq!(formatted(&mut env, &stm), "<function> : a -> a".s());
    }

    #[test]
    fn check_recursivity() {
        let stm = from_code_stm(b"fib num = case num of \n 0 -> 0\n 1 -> 1\n _ -> fib (num - 1) + fib (num - 2)");
        let mut env = DynamicEnv::default_lang_env();

        assert_eq!(formatted(&mut env, &stm), "<function> : Int -> number".s());
    }

    #[test]
    fn check_adt() {
        let decl = from_code_stm(b"type Adt = A | B");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &from_code(b"A")), "A : Adt".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"B")), "B : Adt".s());
    }

    #[test]
    fn check_adt2() {
        let decl = from_code_stm(b"type Adt a = A a | B Int");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &from_code(b"A")), "<function> : a -> Adt a".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"B")), "<function> : Int -> Adt a".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"A 1")), "A 1 : Adt number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"B 1")), "B 1 : Adt a".s());
    }

    #[test]
    fn check_fib() {
        let decl = from_code_stm(b"fib num = case num of \n0 -> 0 \n1 -> 1 \n_ -> fib (num - 1) + fib (num - 2)");
        let mut env = DynamicEnv::default_lang_env();

        eval_stm(&mut env, &decl).unwrap();

        assert_eq!(formatted_expr(&mut env, &from_code(b"fib")), "<function> : Int -> number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 0")), "0 : number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 1")), "1 : number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 2")), "1 : number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 3")), "2 : number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 4")), "3 : number".s());
        assert_eq!(formatted_expr(&mut env, &from_code(b"fib 5")), "5 : number".s());
    }
}