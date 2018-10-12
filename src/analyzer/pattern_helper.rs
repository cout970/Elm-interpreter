use analyzer::environment::Environment;
use types::Literal;
use types::Pattern;
use types::Type;
use types::Value;
use util::name_sequence::NameSequence;
use util::StringConversion;

pub fn add_pattern_variables(env: &mut Environment, pattern: &Pattern) -> Result<(), String> {
    match pattern {
        Pattern::Var(n) => {
            env.add_variable(&n, Type::Var("a".s()))
        }
        Pattern::Record(ref items) => {
            for p in items {
                env.add_variable(p, Type::Var(p.to_owned()))
            }
        }
        Pattern::Adt(_, ref items) => {
            for p in items {
                add_pattern_variables(env, p)?;
            }
        }
        Pattern::Tuple(ref items) => {
            for p in items {
                add_pattern_variables(env, p)?;
            }
        }
        Pattern::List(ref items) => {
            for p in items {
                add_pattern_variables(env, p)?;
            }
        }
        Pattern::Literal(_) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(_, ref a, ref b) => {
            add_pattern_variables(env, a)?;
            add_pattern_variables(env, b)?;
        }
    }

    Ok(())
}

pub fn add_pattern_values(env: &mut Environment, pattern: &Pattern, value: &Value) -> Result<(), String> {
    match pattern {
        Pattern::Var(n) => {
            env.add(&n, value.clone());
        }
        Pattern::Adt(_, ref items) => {
            if let Value::Adt(_, vars) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(format!("Expected Adt but found: {}", value));
            }
        }
        Pattern::Tuple(ref items) => {
            if let Value::Tuple(vars) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(format!("Expected Tuple but found: {}", value));
            }
        }
        Pattern::List(ref items) => {
            if let Value::List(vars) = value {
                for (patt, val) in items.iter().zip(vars) {
                    add_pattern_values(env, patt, val)?;
                }
            } else {
                return Err(format!("Expected List but found: {}", value));
            }
        }
        Pattern::Record(ref items) => {
            if let Value::Record(vars) = value {
                for patt in items {
                    let (name, val) = vars.iter()
                        .find(|(name, _)| name == patt)
                        .ok_or(format!("Unable to find field {} in {}", patt, value))?;

                    env.add(name, val.clone());
                }
            } else {
                return Err(format!("Expected Record but found: {}", value));
            }
            for p in items {
                env.add_variable(p, Type::Var(p.to_owned()))
            }
        }
        Pattern::Literal(_) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(op, ref a, ref b) => {
            if op == "::" {
                if let Value::List(vars) = value {
                    if vars.len() == 0 {
                        return Err(format!("Expected Non Empty List but  it was empty"));
                    }

                    let first = vars[0].clone();
                    let mut rest: Vec<Value> = Vec::new();
                    for i in 1..vars.len() {
                        rest.push(vars[i].clone());
                    }

                    add_pattern_values(env, a, &first)?;
                    add_pattern_values(env, b, &Value::List(rest))?;
                } else {
                    return Err(format!("Expected List but found: {}", value));
                }
            } else {
                return Err(format!("Unknown operator pattern '{}'", op));
            }
        }
    }

    Ok(())
}