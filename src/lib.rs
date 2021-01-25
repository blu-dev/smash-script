#![feature(proc_macro_hygiene)]
#![feature(asm)]
#[macro_use]
pub mod macros;

pub use lua_macro::*;
type ScriptBootstrapperFunc = unsafe fn(&mut smash::lua2cpp::L2CFighterCommon, &mut smash::lib::utility::Variadic);

#[macro_export]
macro_rules! replace_scripts {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_script!($func);
        )*
    };
}

extern "Rust" {
    pub fn replace_lua_script(fighter: &'static str, script: smash::phx::Hash40, func: ScriptBootstrapperFunc);
}