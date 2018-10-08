use analyzer::environment::Environment;
use types::Literal;
use types::Pattern;
use types::Type;
use util::name_sequence::NameSequence;
use util::StringConversion;

pub fn pattern_to_type(patt: &Pattern) -> Result<Type, String> {
    match patt {
        Pattern::Var(n) => {
            Ok(Type::Var(n.to_owned()))
        }
        Pattern::Adt(n, items) => {
            let types: Vec<Type> = items.iter()
                .map(|p| pattern_to_type(p))
                .collect::<Result<_, _>>()?;

            Ok(Type::Tag(n.to_owned(), types))
        }
        Pattern::Wildcard => {
            Ok(Type::Var(NameSequence::new().next()))
        }
        Pattern::Unit => {
            Ok(Type::Unit)
        }
        Pattern::Tuple(items) => {
            let types: Vec<Type> = items.iter()
                .map(|p| pattern_to_type(p))
                .collect::<Result<_, _>>()?;

            Ok(Type::Tuple(types))
        }
        Pattern::List(items) => {
            let item_type = if items.is_empty() {
                Type::Var(NameSequence::new().next())
            } else {
                pattern_to_type(items.first().unwrap())?
            };

            Ok(Type::Tag("List".s(), vec![item_type]))
        }
        Pattern::Record(items) => {
            let mut seq = NameSequence::new();
            let entries = items.iter()
                .map(|p| (p.to_owned(), Type::Var(seq.next())))
                .collect();

            Ok(Type::RecExt(seq.next(), entries))
        }
        Pattern::Literal(lit) => {
            match lit {
                Literal::Int(_) => Ok(Type::Tag("Int".s(), vec![])),
                Literal::Float(_) => Ok(Type::Tag("Float".s(), vec![])),
                Literal::String(_) => Ok(Type::Tag("String".s(), vec![])),
                Literal::Char(_) => Ok(Type::Tag("Char".s(), vec![])),
            }
        }
        Pattern::BinaryOp(_, _, _) => {
            Err(format!("Infix pattern cannot be used in this situation"))
        }
    }
}

pub fn add_pattern_variables(env: &mut Environment, pattern: &Pattern) -> Result<(), String> {
    match pattern {
        Pattern::Var(n) => {
            env.add_variable(&n, Type::Var(n.to_owned()))
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
        Pattern::Record(ref items) => {
            for p in items {
                env.add_variable(p, Type::Var(p.to_owned()))
            }
        }
        Pattern::Literal(lit) => {}
        Pattern::Wildcard => {}
        Pattern::Unit => {}
        Pattern::BinaryOp(_, ref a, ref b) => {
            add_pattern_variables(env, a)?;
            add_pattern_variables(env, b)?;
        }
    }

    Ok(())
}