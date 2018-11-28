use ast::Type;

pub fn calculate_common_type(types: &[Type]) -> Result<&Type, (&Type, &Type)> {
    let first = types.first().unwrap();

    for i in 1..types.len() {
        if !is_assignable(first, &types[i]) {
            return Err((first, &types[i]));
        }
    }
    Ok(first)
}

pub fn is_assignable(expected: &Type, found: &Type) -> bool {
    if expected == found { return true; }

    if let Type::Var(_) = found {
        match expected {
            Type::Var(_) => (),
            _ => {
                return true;
            }
        }
    }

    match expected {
        Type::Var(name) => {
            match name.as_str() {
                "number" => {
                    match found {
                        Type::Var(_) => true,
                        Type::Tag(ty_name, _) => ty_name == "Int" || ty_name == "Float",
                        _ => false
                    }
                }
                _ => true
            }
        }
        Type::Tag(name, sub) => {
            match found {
                Type::Tag(ty_name, ty_sub) => {
                    ty_name == name && sub.iter().zip(ty_sub).all(|(a, b)| is_assignable(a, b))
                }
                _ => false
            }
        }
        Type::Fun(input, output) => {
            match found {
                Type::Fun(a, b) => is_assignable(input, a) && is_assignable(output, b),
                _ => false
            }
        }
        Type::Tuple(sub) => {
            match found {
                Type::Tuple(ty_sub) => {
                    sub.iter().zip(ty_sub).all(|(a, b)| is_assignable(a, b))
                }
                _ => false
            }
        }
        Type::Record(entries) => {
            match found {
                Type::Record(entries_ty) => {
                    entries.iter().all(|(name, ty)| entries_ty.iter().any(|(n, t)|
                        n == name && is_assignable(ty, t)
                    ))
                }
                _ => false
            }
        }
        Type::RecExt(name, entries) => {
            match found {
                Type::RecExt(name_ty, entries_ty) => {
                    name == name_ty && entries.iter()
                        .all(|(name, ty)| entries_ty.iter().any(|(n, t)|
                            n == name && is_assignable(ty, t)
                        ))
                }
                _ => false
            }
        }
        _ => false
    }
}