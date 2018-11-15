use analyzer::static_env::StaticEnv;

mod basics;

pub fn register_core(env: &mut StaticEnv){
    basics::register_basics(env)
}