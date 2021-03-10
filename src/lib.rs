#![feature(proc_macro_hygiene)]
#![feature(asm)]
#[macro_use]
pub mod macros;

pub enum ScriptCategory {
    ACMD_EFFECT,
    ACMD_EXPRESSION,
    ACMD_GAME,
    ACMD_SOUND
}

pub use ScriptCategory::*;

pub use lua_macro::*;
type ScriptBootstrapperFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CAgentBase, &mut smash::lib::utility::Variadic);
type StatusFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CFighterBase) -> smash::lib::L2CValue;
type SysLineControlFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CFighterCommon) -> smash::lib::L2CValue;
type SysLineCallbackFunc = unsafe fn(&mut smash::lua2cpp::L2CFighterCommon);
type SysLineWeaponControlFunc = unsafe extern "C" fn(&mut smash::lua2cpp::L2CFighterBase) -> smash::lib::L2CValue;
type SysLineWeaponCallbackFunc = unsafe fn(&mut smash::lua2cpp::L2CFighterBase);

#[macro_export]
macro_rules! replace_scripts {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_script!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_status_scripts {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_status_script!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_fighter_frames {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_fighter_frame!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_weapon_frames {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_weapon_frame!($func);
        )*
    };
}

#[macro_export]
macro_rules! add_fighter_frame_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_sys_line_fighter_callback($func); }
        )*
    };
}

#[macro_export]
macro_rules! add_weapon_frame_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_sys_line_weapon_callback($func); }
        )*
    };
}

#[macro_export]
macro_rules! add_fighter_reset_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_fighter_reset_callback($func); }
        )*
    };
}

#[macro_export]
macro_rules! add_global_reset_callbacks {
    ($($func:ident),* $(,)?) => {
        $(
            unsafe { $crate::add_global_reset_callback($func); }
        )*
    };
}

#[macro_export]
macro_rules! replace_common_statuses {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_common_status!($func);
        )*
    };
}

#[macro_export]
macro_rules! replace_common_symbols {
    ($($func:ident),* $(,)?) => {
        $(
            $crate::replace_common_symbol!($func);
        )*
    };
}

extern "Rust" {
    pub fn replace_lua_script(agent: smash::phx::Hash40, script: smash::phx::Hash40, func: ScriptBootstrapperFunc, category: ScriptCategory);
    pub fn replace_status_script(agent: smash::phx::Hash40, status: smash::lib::LuaConst, condition: smash::lib::LuaConst, func: StatusFunc);
    pub fn replace_sys_line_fighter_script(agent: smash::lib::LuaConst, func: SysLineControlFunc);
    pub fn replace_sys_line_weapon_script(agent: smash::lib::LuaConst, func: SysLineWeaponControlFunc);
    pub fn add_sys_line_fighter_callback(func: SysLineCallbackFunc);
    pub fn add_sys_line_weapon_callback(func: SysLineWeaponCallbackFunc);
    pub fn add_fighter_reset_callback(func: SysLineCallbackFunc);
    pub fn add_global_reset_callback(func: SysLineWeaponCallbackFunc);
    pub fn replace_symbol(module: String, symbol: String, replace: *const extern "C" fn());
    pub fn replace_common_status(status: smash::lib::LuaConst, condition: smash::lib::LuaConst, replace: *const extern "C" fn());
}