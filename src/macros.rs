#![allow(non_snake_case)]
use smash::lib::L2CValue;
use smash::lua2cpp::L2CFighterCommon;
use smash::phx::Hash40;

use smash::app::sv_animcmd;
use smash::lib::lua_const::*;

#[macro_export]
macro_rules! lua_args {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $(
            $fighter.push_lua_stack(&mut $arg.into());
        )*
    };
}

#[inline]
pub unsafe fn ATTACK(fighter: &mut L2CFighterCommon, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: u64, fkb: u64, bkb: u64, size: f32, x: f32, y: f32, z: f32,
                    x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, hitlag: f32, sdi: f32, clang: i32, facing: i32, set_weight: bool, shield_damage: u64, trip: f32, rehit: u64, reflectable: bool,
                    absorbable: bool, flinchless: bool, disable_hitlag: bool, direct: bool, ground_air: i32, hitbits: i32, collision_part: i32, friendly_fire: bool, effect: Hash40, sfx_level: i32, collision_sound: i32, _type: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, part, bone, damage, angle, kbg, fkb, bkb, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, hitlag, sdi, clang, facing, set_weight, shield_damage, trip, rehit, reflectable, absorbable, flinchless, disable_hitlag, direct, ground_air, hitbits, collision_part, friendly_fire, effect, sfx_level, collision_sound, _type);
    sv_animcmd::ATTACK(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn ATTACK_ABS(fighter: &mut L2CFighterCommon, kind: i32, id: u64, damage: f32, angle: u64, kbg: u64, fkb: u64, bkb: u64, hitlag: f32,
                        unk: f32, facing: i32, unk2: f32, unk3: bool, effect: Hash40, sfx_level: i32, sfx_type: i32, _type: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind, id, damage, angle, kbg, fkb, bkb, hitlag, unk, facing, unk2, unk3, effect, sfx_level, sfx_type, _type);
    sv_animcmd::ATTACK_ABS(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn ATK_HIT_ABS(fighter: &mut L2CFighterCommon, kind: i32, unk: Hash40, target: u64, target_group: u64, target_no: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind, unk, target, target_group, target_no);
    sv_animcmd::ATK_HIT_ABS(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn is_excute(fighter: &mut L2CFighterCommon) -> bool {
    fighter.clear_lua_stack();
    sv_animcmd::is_excute(fighter.lua_state_agent);
    let ret = fighter.pop_lua_stack(1).get_bool();
    ret
}

#[inline]
pub unsafe fn CATCH(fighter: &mut L2CFighterCommon, id: i32, bone: Hash40, size: f32, x: f32, y: f32, z: f32, x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, status: i32, situation: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, bone, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, status, situation);
    sv_animcmd::CATCH(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn FT_MOTION_RATE(fighter: &mut L2CFighterCommon, rate: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, rate);
    sv_animcmd::FT_MOTION_RATE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn IS_GENERATABLE_ARTICLE(fighter: &mut L2CFighterCommon, article: i32) -> bool {
    fighter.clear_lua_stack();
    lua_args!(fighter, article);
    let ret = sv_animcmd::IS_GENERATABLE_ARTICLE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    ret
}

#[inline]
pub unsafe fn HIT_NO(fighter: &mut L2CFighterCommon, num: u64, status: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, num, status);
    sv_animcmd::HIT_NO(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[macro_export]
macro_rules! grab {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::grab($fighter.lua_state_agent);
        let ret = $fighter.pop_lua_stack(1).get_bool();
        ret
    };
}

#[macro_export]
macro_rules! damage {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::damage($fighter.lua_state_agent);
        let ret = $fighter.pop_lua_stack(1).get_int();
        ret
    }
}

#[macro_export]
macro_rules! notify_event_msc_cmd {
    ($fighter: ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_battle_object::notify_event_msc_cmd($fighter.lua_state_agent);
        let ret = $fighter.pop_lua_stack(1).get_int();
        ret
    }
}

#[inline]
pub unsafe fn game_CaptureCutCommon(fighter: &mut L2CFighterCommon) {
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    sv_animcmd::ATTACK_ABS(fighter.lua_state_agent);
}