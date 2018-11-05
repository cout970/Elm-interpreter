/*
 * Copied (with modifications) from https://github.com/jonathandturner/rhai/blob/master/src/fn_register.rs
 * which has MIT/Apache-2.0 license
 */

use std::any::TypeId;
use std::any::Any;
use rust_interop::FnAny;
use rust_interop::InteropError;

pub trait FunctionRegister {
    fn register_fn_raw(&mut self, name: String, args: Option<Vec<TypeId>>, boxed: Box<FnAny>);
}

pub trait RegisterFn<FN, ARGS, RET> {
    fn register_fn(&mut self, name: &str, f: FN);
}

pub struct Ref<A>(A);

pub struct Mut<A>(A);

macro_rules! count_args {
    () => {0usize};
    ($head:ident $($tail:ident)*) => {1usize + count_args!($($tail)*)};
}

macro_rules! def_register {
    () => {
        def_register!(imp);
    };
    (imp $($par:ident => $mark:ty => $param:ty => $clone:expr),*) => {
        impl<$($par,)* FN, RET, T> RegisterFn<FN, ($($mark,)*), RET> for T
        where
            $($par: Any + Clone,)*
            FN: Fn($($param),*) -> RET + 'static,
            RET: Any,
            T: FunctionRegister
        {
            #[allow(non_snake_case, dead_code, unused_mut, unused)]
            fn register_fn(&mut self, name: &str, f: FN) {
                let fun = move |mut args: Vec<&mut Any>| {
                    // Check for length at the beginning to avoid
                    // per-element bound checks.
                    if args.len() != count_args!($($par)*) {
                        return Err(InteropError::FunctionArgMismatch);
                    }

                    let mut drain = args.drain(..);
                    $(
                    // Downcast every element, return in case of a type mismatch
                    let $par = ((*drain.next().unwrap()).downcast_mut() as Option<&mut $par>)
                        .ok_or(InteropError::FunctionArgMismatch)?;
                    )*

                    // Call the user-supplied function using ($clone) to
                    // potentially clone the value, otherwise pass the reference.
                    Ok(Box::new(f($(($clone)($par)),*)) as Box<Any>)
                };
                self.register_fn_raw(name.to_owned(), Some(vec![$(TypeId::of::<$par>()),*]), Box::new(fun));
            }
        }

    };
    ($p0:ident $(, $p:ident)*) => {
        def_register!(imp $p0 => $p0 => $p0 => Clone::clone $(, $p => $p => $p => Clone::clone)*);
        def_register!(imp $p0 => Ref<$p0> => &$p0 => |x| { x } $(, $p => $p => $p => Clone::clone)*);
        def_register!(imp $p0 => Mut<$p0> => &mut $p0 => |x| { x } $(, $p => $p => $p => Clone::clone)*);

        def_register!($($p),*);
    };
}

#[cfg_attr(rustfmt, rustfmt_skip)]
def_register!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);



#[cfg(test)]
mod tests {
    use super::*;

    struct Test {}

    impl FunctionRegister for Test {
        fn register_fn_raw(&mut self, name: String, args: Option<Vec<TypeId>>, _boxed: Box<FnAny>) {
            assert_eq!(name, "test_function");
            assert_eq!(args, Some(vec![TypeId::of::<i32>()]));
        }
    }

    #[test]
    fn register_function() {
        let mut e = Test{};

        e.register_fn("test_function", test_function);
    }

    fn test_function(a: i32) -> i32 {
        a
    }
}