use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::cell::UnsafeCell;

#[derive(Clone, Debug)]
pub enum EvalAltResult {
    ErrorFunctionArgMismatch,
    ErrorMismatchOutputType,
    ErrorFunctionNotFound(String),
}

pub trait FunArgs<'a> {
    fn into_vec(self) -> Vec<&'a mut Any>;
    fn get_types(&self) -> Vec<TypeId>;
}

macro_rules! impl_args {
    ($($p:ident),*) => {
        impl<'a, $($p),*> FunArgs<'a> for ($(&'a mut $p,)*)
        where
            $($p: Any + Clone),*
        {
            #[allow(non_snake_case, dead_code, unused_mut, unused)]
            fn into_vec(self) -> Vec<&'a mut Any> {
                let ($($p,)*) = self;

                let mut v = Vec::new();
                $(v.push($p as &mut Any);)*

                v
            }

            #[allow(non_snake_case, dead_code, unused_mut, unused)]
            fn get_types(&self) -> Vec<TypeId> {
                let mut v = Vec::new();
                $(v.push(TypeId::of::<$p>());)*

                v
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

type FnAny = FnMut(Vec<&mut Any>) -> Result<Box<Any>, EvalAltResult>;


#[derive(Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct FnSpec {
    ident: String,
    args: Option<Vec<TypeId>>,
}

pub struct Engine {
    pub fns: HashMap<FnSpec, Box<FnAny>>,
    pub type_names: HashMap<TypeId, String>,
}


impl Engine {
    pub fn new() -> Self {
        Self {
            fns: HashMap::new(),
            type_names: HashMap::new(),
        }
    }

    pub fn register_fn_raw(&mut self, name: String,
                           args: Option<Vec<TypeId>>,
                           boxed: Box<FnAny>) {
        self.fns.insert(FnSpec { ident: name, args }, boxed);
    }

    pub fn call_fn<'a, I, A, T>(&mut self, ident: I, args: A) -> Result<T, EvalAltResult>
        where
            I: Into<String>,
            A: FunArgs<'a>,
            T: Any + Clone,
    {
        let types = args.get_types();
        self.call_fn_raw(ident.into(), args.into_vec(), types)
            .and_then(|b| {
                b.downcast()
                    .map(|b| *b)
                    .map_err(|_| EvalAltResult::ErrorMismatchOutputType)
            })
    }

    pub fn call_fn_raw(
        &mut self,
        ident: String,
        args: Vec<&mut Any>,
        arg_types: Vec<TypeId>,
    ) -> Result<Box<Any>, EvalAltResult> {
        let spec = FnSpec {
            ident: ident.clone(),
            args: Some(arg_types),
        };

        self.fns
            .get_mut(&spec)
            .ok_or_else(|| EvalAltResult::ErrorMismatchOutputType)
            .and_then(move |f| {
                f(args)
            })
    }
}

/*
 * Stolen from https://github.com/jonathandturner/rhai/blob/master/src/fn_register.rs
 * which has MIT/Apache-2.0 license
 */

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
        impl<$($par,)* FN, RET> RegisterFn<FN, ($($mark,)*), RET> for Engine
        where
            $($par: Any + Clone,)*
            FN: Fn($($param),*) -> RET + 'static,
            RET: Any,
        {
            #[allow(non_snake_case, dead_code, unused_mut, unused)]
            fn register_fn(&mut self, name: &str, f: FN) {
                let fun = move |mut args: Vec<&mut Any>| {
                    // Check for length at the beginning to avoid
                    // per-element bound checks.
                    if args.len() != count_args!($($par)*) {
                        return Err(EvalAltResult::ErrorFunctionArgMismatch);
                    }

                    let mut drain = args.drain(..);
                    $(
                    // Downcast every element, return in case of a type mismatch
                    let $par = ((*drain.next().unwrap()).downcast_mut() as Option<&mut $par>)
                        .ok_or(EvalAltResult::ErrorFunctionArgMismatch)?;
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