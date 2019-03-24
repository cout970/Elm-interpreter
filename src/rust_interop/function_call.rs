use std::any::Any;
use std::any::TypeId;

use rust_interop::InteropError;

pub trait FunctionCall {
    fn call_fn_raw(&mut self, name: &str, args: Vec<&mut Any>, arg_types: Vec<TypeId>) -> Result<Box<Any>, InteropError>;
}

pub trait CallFn<ARGS> {
    fn call_fn(&mut self, name: &str, args: ARGS) -> Result<Box<Any>, InteropError>;
}

macro_rules! impl_args {
    ($($p:ident),*) => {
        impl<'a, T, $($p),*> CallFn<($(&'a mut $p,)*)> for T
        where
            T: FunctionCall,
            $($p: Any + Clone),*
        {

            #[allow(non_snake_case, dead_code, unused_mut, unused)]
            fn call_fn(&mut self, name: &str, args: ($(&'a mut $p,)*)) -> Result<Box<Any>, InteropError> {
                let ($($p,)*) = args;

                let mut values = Vec::new();
                $(values.push($p as &mut Any);)*

                let mut types = Vec::new();
                $(types.push(TypeId::of::<$p>());)*

                self.call_fn_raw(name, values, types)
            }
        }

        impl_args!(@pop $($p),*);
    };
    (@pop) => {
    };
    (@pop $head:ident $(, $tail:ident)*) => {
        impl_args!($($tail),*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl_args!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);


#[cfg(test)]
mod tests {
    use super::*;

    struct Test {}

    impl FunctionCall for Test {
        fn call_fn_raw(&mut self, name: &str, args: Vec<&mut Any>, arg_types: Vec<TypeId>) -> Result<Box<Any>, InteropError> {
            assert_eq!(name, "test_function");
            println!("{:?}", args);
            println!("{:?}", arg_types);
            Ok(Box::new(42))
        }
    }

    #[test]
    fn call_function() {
        let mut e = Test{};

        e.call_fn("test_function", (&mut 1,)).unwrap();
        e.call_fn("test_function", (&mut 1, &mut 2)).unwrap();
        e.call_fn("test_function", (&mut 1, &mut 2, &mut 3)).unwrap();
    }
}
