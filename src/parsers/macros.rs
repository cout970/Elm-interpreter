#[macro_export]
macro_rules! tk {
    ($i:expr, $token: expr) => (
        {
            use nom::*;
            use nom::verbose_errors::Context;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = $i.read_tk();

                if look == $token {
                    Ok(($i.next(1), look.clone()))
                } else {
                    Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::ExpectedToken($token, $i.read_info())
                    ))))
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
            use tokenizer::Token;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = $i.read_tk();

                if let Token::Id(str) = look {
                    Ok(($i.next(1), str.clone()))
                } else {
                    Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::ExpectedId($i.read_info())
                    ))))
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
            use tokenizer::Token;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = $i.read_tk();

                if let Token::UpperId(str) = look {
                    Ok(($i.next(1), str.clone()))
                } else {
                    Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::ExpectedUpperId($i.read_info())
                    ))))
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
            use tokenizer::Token;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = $i.read_tk();

                if let Token::BinaryOperator(str) = look {
                  Ok(($i.next(1), str.clone()))
                } else {
                    if let Token::PrefixMinus = look {
                        Ok(($i.next(1), "-".to_owned()))
                    } else {
                        Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                             SyntaxError::ExpectedBinaryOperator($i.read_info())
                        ))))
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
            use tokenizer::Token;

            if $i.len() == 0 {
                Err(Err::Incomplete(Needed::Size(1)))
            } else {
                let look = $i.read_tk();

                if let Token::PrefixMinus = look {
                    Ok(($i.next(1), ()))
                } else {
                    Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::ExpectedToken(Token::PrefixMinus, $i.read_info())
                    ))))
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
                let look = $i.read_tk();

                match look {
                   LitInt(value) => Ok(($i.next(1), Literal::Int(value))),
                   LitFloat(value) => Ok(($i.next(1), Literal::Float(value))),
                   LitChar(value) => Ok(($i.next(1), Literal::Char(value))),
                   LitString(value) => Ok(($i.next(1), Literal::String(value))),
                   _ => Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::ExpectedLiteral($i.read_info())
                   ))))
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
        use tokenizer::Token;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = $i.read_tk();

            if look == Token::Indent($count) {
                Ok(($i.next(1), ()))
            } else {
                 Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                    SyntaxError::ExpectedToken(Token::Indent($count), $i.read_info())
                ))))
            }
        }
    }
    );

    ($i:expr,) => (
    {
        use nom::*;
        use nom::verbose_errors::Context;
        use tokenizer::Token;

        if $i.len() == 0 {
            Err(Err::Incomplete(Needed::Size(1)))
        } else {
            let look = $i.read_tk();

            if let Token::Indent(count) = look {
                Ok(($i.next(1), count))
            } else {
                 Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                    SyntaxError::ExpectedToken(Token::Indent(0), $i.read_info())
                ))))
            }
        }
    }
    );
}

#[macro_export]
macro_rules! indent_except {
    ($i:expr, $levels: expr) => (
    {
        use nom::*;
        use nom::verbose_errors::Context;
        use tokenizer::Token;

        if $i.len() > 0 {
            let look = $i.read_tk();

            if let Token::Indent(count) = look {
                if $levels.contains( &(count as usize)) {
                    Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                        SyntaxError::InvalidIndentation(($levels).clone(), count as usize)
                    ))))
                } else {
                    Ok(($i.next(1), count))
                }
            } else {
                Err(Err::Error(Context::Code( ($i).clone(), ErrorKind::Custom(
                    SyntaxError::ExpectedToken(Token::Indent(0), $i.read_info())
                ))))
            }
        } else {
            Err(Err::Incomplete(Needed::Size(1)))
        }
    }
    );
}

// The macro many1!() is broken and cannot be used with TokenStream, just &[u8] or &[Token] because they implement Copy
#[macro_export]
macro_rules! one_or_more (
  ($i:expr, $submac:ident!( $($args:tt)* )) => (
    {
      use nom::lib::std::result::Result::*;
      use nom::Err;

      use nom::InputLength;
      let i_ = $i.clone();
      match $submac!(i_, $($args)*) {
        Err(Err::Error(_))      => Err(Err::Error(
          error_position!(($i), nom::ErrorKind::Many1)
        )),
        Err(Err::Failure(_))      => Err(Err::Failure(
          error_position!(($i), nom::ErrorKind::Many1)
        )),
        Err(i) => Err(i),
        Ok((i1,o1))   => {
          let mut res    = nom::lib::std::vec::Vec::with_capacity(4);
          res.push(o1);
          let mut input  = i1;
          let mut error = nom::lib::std::option::Option::None;
          loop {
            let input_ = input.clone();
            match $submac!(input_, $($args)*) {
              Err(Err::Error(_))                    => {
                break;
              },
              Err(e) => {
                error = nom::lib::std::option::Option::Some(e);
                break;
              },
              Ok((i, o)) => {
                if i.input_len() == input.input_len() {
                  break;
                }
                res.push(o);
                input = i;
              }
            }
          }

          match error {
            nom::lib::std::option::Option::Some(e) => Err(e),
            nom::lib::std::option::Option::None    => Ok((input, res))
          }
        }
      }
    }
  );
  ($i:expr, $f:expr) => (
    one_or_more!($i, call!($f));
  );
);


// To reduce boilerplate code
#[macro_export]
macro_rules! rule (
    (#$($args:tt)*) => (
        named_attr!(#$($args)*);
    );
    ($name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        fn $name( i: Tk ) -> nom::IResult<Tk, $o, SyntaxError> {
            $submac!(i, $($args)*)
        }
    );
    (pub $name:ident<$o:ty>, $submac:ident!( $($args:tt)* )) => (
        pub fn $name( i: Tk ) -> nom::IResult<Tk, $o, SyntaxError> {
            $submac!(i, $($args)*)
        }
    );
);

#[macro_export]
macro_rules! method_rule (
  // Non-public immutable self
  ($name:ident<$a:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( $self_: $a, i: Tk ) -> ($a, nom::IResult<Tk,$o,SyntaxError>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  // Public immutable self
  (pub $name:ident<$a:ty,$o:ty>, $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( $self_: $a,i: Tk ) -> ($a, nom::IResult<Tk,$o,SyntaxError>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  // Non-public mutable self
  ($name:ident<$a:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    fn $name( mut $self_: $a, i: Tk ) -> ($a, nom::IResult<Tk,$o,SyntaxError>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
  // Public mutable self
  (pub $name:ident<$a:ty,$o:ty>, mut $self_:ident, $submac:ident!( $($args:tt)* )) => (
    #[allow(unused_variables)]
    pub fn $name( mut $self_: $a,i: Tk ) -> ($a, nom::IResult<Tk,$o,SyntaxError>)  {
      let result = $submac!(i, $($args)*);
      ($self_, result)
    }
  );
);