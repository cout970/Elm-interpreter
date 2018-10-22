
#[macro_export]
macro_rules! tk {
    ($i:expr, $token: expr) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if look == &$token {
                  Ok((&$i[1..], look.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! id {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::Id(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! upper_id {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::UpperId(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                  Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! binop {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::BinaryOperator(str) = look {
                  Ok((&$i[1..], str.clone()))
                } else {
                    if let Token::PrefixMinus = look {
                        Ok((&$i[1..], "-".to_owned()))
                    } else {
                        Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                    }
                }
            }
        }
    );
}

#[macro_export]
macro_rules! minus {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                if let Token::PrefixMinus = look {
                    Ok((&$i[1..], ()))
                } else {
                    Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! literal {
    ($i:expr,) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = &$i[0];

                match look {
                   LitInt(value) => Ok((&$i[1..], Literal::Int(*value))),
                   LitFloat(value) => Ok((&$i[1..], Literal::Float(*value))),
                   LitChar(value) => Ok((&$i[1..], Literal::Char(*value))),
                   LitString(value) => Ok((&$i[1..], Literal::String(value.clone()))),
                   _ => Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                }
            }
        }
    );
}

#[macro_export]
macro_rules! indent {
    ($i:expr, $count: expr) => (
    {
        use nom::*;
        use nom::verbose_errors::Context;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = &$i[0];

            if look == &Token::Indent($count) {
                Ok((&$i[1..], ()))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    }
    );

    ($i:expr,) => (
    {
        use nom::*;
        use nom::verbose_errors::Context;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = &$i[0];

            if let Token::Indent(count) = look {
                Ok((&$i[1..], *count))
            } else {
                Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
            }
        }
    }
    );
}

macro_rules! indent_except {
    ($i:expr, $levels: expr) => (
    {
        use nom::*;
        use nom::verbose_errors::Context;

        if $i.len() > 0 {
            let look = &$i[0];

            if let Token::Indent(count) = look {
                if $levels.contains( &(*count as usize)) {
                    Err(Err::Error(Context::Code($i, ErrorKind::Custom(0))))
                } else {
                    Ok((&$i[1..], * count))
                }
            } else {
                Err(Err::Error(Context::Code( $i, ErrorKind::Custom(0))))
            }
        } else {
            Err(Err::Incomplete(Needed::Size(1)))
        }
    }
    );
}