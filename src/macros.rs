#![allow(non_snake_case)]
use smash::lib::L2CValue;
use smash::lua2cpp::L2CAgentBase;
use smash::phx::Hash40;

use smash::app::{sv_animcmd, lua_bind};
use smash::lib::lua_const::*;

pub trait ToF32 {
    fn to_f32(self) -> f32;
}

impl ToF32 for i32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u32 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for i64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for u64 {
    fn to_f32(self) -> f32 {
        self as f32
    }
}

impl ToF32 for f32 {
    fn to_f32(self) -> f32 { self }
}

impl ToF32 for f64 {
    fn to_f32(self) -> f32 { self as f32 }
}

#[macro_export]
macro_rules! lua_args {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $(
            $fighter.push_lua_stack(&mut $arg.into());
        )*
    };
}

#[inline]
pub unsafe fn ATTACK<A: ToF32, B: ToF32, C: ToF32, D: ToF32>(fighter: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: D, fkb: i32, bkb: i32, size: C, x: f32, y: f32, z: f32,
                    x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, hitlag: f32, sdi: f32, clang: i32, facing: i32, set_weight: bool, shield_damage: A, trip: f32, rehit: B, reflectable: bool,
                    absorbable: bool, flinchless: bool, disable_hitlag: bool, direct: bool, ground_air: i32, hitbits: i32, collision_part: i32, friendly_fire: bool, effect: Hash40, sfx_level: i32, collision_sound: i32, _type: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, part, bone, damage, angle, kbg.to_f32(), fkb, bkb, size.to_f32(), x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, hitlag, sdi, clang, facing, set_weight);
    let dmg = shield_damage.to_f32();
    if (dmg.is_nan()) { fighter.push_lua_stack(&mut Hash40::new("no").into()); } else { fighter.push_lua_stack(&mut dmg.into()); }
    lua_args!(fighter, trip, rehit.to_f32(), reflectable, absorbable, flinchless, disable_hitlag, direct, ground_air, hitbits, collision_part, friendly_fire, effect, sfx_level, collision_sound, _type);
    sv_animcmd::ATTACK(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn ATTACK_IGNORE_THROW<A: ToF32, B: ToF32, C: ToF32>(fighter: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, damage: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32, size: C, x: f32, y: f32, z: f32,
                    x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, hitlag: f32, sdi: f32, clang: i32, facing: i32, set_weight: bool, shield_damage: A, trip: f32, rehit: B, reflectable: bool,
                    absorbable: bool, flinchless: bool, disable_hitlag: bool, direct: bool, ground_air: i32, hitbits: i32, collision_part: i32, friendly_fire: bool, effect: Hash40, sfx_level: i32, collision_sound: i32, _type: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, part, bone, damage, angle, kbg, fkb, bkb, size.to_f32(), x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, hitlag, sdi, clang, facing, set_weight);
    let dmg = shield_damage.to_f32();
    if (dmg.is_nan()) { fighter.push_lua_stack(&mut Hash40::new("no").into()); } else { fighter.push_lua_stack(&mut dmg.into()); }
    lua_args!(fighter, trip, rehit.to_f32(), reflectable, absorbable, flinchless, disable_hitlag, direct, ground_air, hitbits, collision_part, friendly_fire, effect, sfx_level, collision_sound, _type);
    sv_animcmd::ATTACK_IGNORE_THROW(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn ATK_POWER<F: ToF32>(fighter: &mut L2CAgentBase, id: u64, power: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, power.to_f32());
    sv_animcmd::ATK_POWER(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATTACK_ABS(fighter: &mut L2CAgentBase, kind: i32, id: u64, damage: f32, angle: u64, kbg: i32, fkb: i32, bkb: i32, hitlag: f32,
                        unk: f32, facing: i32, unk2: f32, unk3: bool, effect: Hash40, sfx_level: i32, sfx_type: i32, _type: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind, id, damage, angle, kbg, fkb, bkb, hitlag, unk, facing, unk2, unk3, effect, sfx_level, sfx_type, _type);
    sv_animcmd::ATTACK_ABS(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn ATK_HIT_ABS(fighter: &mut L2CAgentBase, kind: i32, unk: Hash40, target: u64, target_group: u64, target_no: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind, unk, target, target_group, target_no);
    sv_animcmd::ATK_HIT_ABS(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn is_excute(fighter: &mut L2CAgentBase) -> bool {
    fighter.clear_lua_stack();
    sv_animcmd::is_excute(fighter.lua_state_agent);
    let ret = fighter.pop_lua_stack(1).get_bool();
    ret
}

#[inline]
pub unsafe fn CATCH(fighter: &mut L2CAgentBase, id: i32, bone: Hash40, size: f32, x: f32, y: f32, z: f32, x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, status: i32, situation: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, bone, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, status, situation);
    sv_animcmd::CATCH(fighter.lua_state_agent);
}

#[inline]
pub unsafe fn FT_DESIRED_RATE(fighter: &mut L2CAgentBase, motion_frames: f32, game_frames: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, (game_frames / motion_frames));
    sv_animcmd::FT_MOTION_RATE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_DESIRED_RATE2(fighter: &mut L2CAgentBase, motion_start_frame: f32, motion_end_frame: f32, game_frames: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, (game_frames / (motion_end_frame / motion_start_frame)));
    sv_animcmd::FT_MOTION_RATE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_MOTION_RATE<F: ToF32>(fighter: &mut L2CAgentBase, rate: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, rate.to_f32());
    sv_animcmd::FT_MOTION_RATE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SHOOTING_ATTACK_GROUND_CHECK_NEW<A: ToF32, B: ToF32, C: ToF32>(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5<A: ToF32, B: ToF32, C: ToF32>(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: Hash40, unk5: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4, unk5);
    sv_animcmd::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_START_CUTIN(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::FT_START_CUTIN(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_LEAVE_NEAR_OTTOTTO<A: ToF32, B: ToF32>(fighter: &mut L2CAgentBase, unk1: A, unk2: B) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32());
    sv_animcmd::FT_LEAVE_NEAR_OTTOTTO(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_START_ADJUST_MOTION_FRAME_arg1(fighter: &mut L2CAgentBase, arg: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, arg);
    sv_animcmd::FT_START_ADJUST_MOTION_FRAME_arg1(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT<F: ToF32>(fighter: &mut L2CAgentBase, offset: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, offset.to_f32());
    sv_animcmd::LAST_EFFECT_SET_OFFSET_TO_CAMERA_FLAT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CHECK_VALID_FINAL_START_CAMERA<A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32>(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32());
    sv_animcmd::CHECK_VALID_FINAL_START_CAMERA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn REQ_FINAL_START_CAMERA(fighter: &mut L2CAgentBase, camera: Hash40, unk: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, camera, unk);
    sv_animcmd::REQ_FINAL_START_CAMERA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn REQ_FINAL_START_CAMERA_arg3(fighter: &mut L2CAgentBase, camera: Hash40, unk: bool, unk2: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, camera, unk, unk2);
    sv_animcmd::REQ_FINAL_START_CAMERA_arg3(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn IS_GENERATABLE_ARTICLE(fighter: &mut L2CAgentBase, article: i32) -> bool {
    fighter.clear_lua_stack();
    lua_args!(fighter, article);
    let ret = sv_animcmd::IS_GENERATABLE_ARTICLE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
    ret
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_arg5(fighter: &mut L2CAgentBase, zoom_amount: f32, arg2: f32, arg3: f32, y_rot: f32, x_rot: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, zoom_amount, arg2, arg3, y_rot, x_rot);
    sv_animcmd::CAM_ZOOM_IN_arg5(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_arg6(fighter: &mut L2CAgentBase, arg1: f32, arg2: f32, arg3: f32, arg4: f32, arg5: f32, arg6: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, arg1, arg2, arg3, arg4, arg5, arg6);
    sv_animcmd::CAM_ZOOM_IN_arg6(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_NO(fighter: &mut L2CAgentBase, num: u64, status: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, num, status);
    sv_animcmd::HIT_NO(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_NODE(fighter: &mut L2CAgentBase, bone: Hash40, status: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, bone, status);
    sv_animcmd::HIT_NODE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn HIT_RESET_ALL(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::HIT_RESET_ALL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg3 <A: ToF32> (fighter: &mut L2CAgentBase, unk: u64, unk2: A, unk3: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk, unk2.to_f32(), unk3);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_LERP_RATIO <A: ToF32> (fighter: &mut L2CAgentBase, ratio: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, ratio.to_f32());
    sv_animcmd::ATK_LERP_RATIO(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn QUAKE(fighter: &mut L2CAgentBase, kind: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind);
    sv_animcmd::QUAKE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_SPEED_EX<A: ToF32, B: ToF32>(fighter: &mut L2CAgentBase, speed_x: A, speed_y: B, kinetic_kind: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, speed_x.to_f32(), speed_y.to_f32(), kinetic_kind);
    sv_animcmd::SET_SPEED_EX(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SLOW_OPPONENT(fighter: &mut L2CAgentBase, slow_mul: f32, slow_time: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, slow_mul, slow_time);
    sv_animcmd::SLOW_OPPONENT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_SET_FINAL_FEAR_FACE(fighter: &mut L2CAgentBase, unk: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::FT_SET_FINAL_FEAR_FACE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_OUT(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::CAM_ZOOM_OUT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_OUT_FINAL(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::CAM_ZOOM_OUT_FINAL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CAM_ZOOM_IN_FINAL_arg13(fighter: &mut L2CAgentBase, x: f32, y: f32, z: f32, unk1: i32, unk2: i32, unk3: i32, unk4: i32, unk5: i32, unk6: bool, object_id: u32, unk7: i32, unk8: i32, unk9: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, x, y, z, unk1, unk2, unk3, unk4, unk5, unk6, object_id, unk7 ,unk8, unk9);
    sv_animcmd::CAM_ZOOM_OUT_FINAL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL<A: ToF32>(fighter: &mut L2CAgentBase, id: u64, val: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, val.to_f32());
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL2<A: ToF32>(fighter: &mut L2CAgentBase, id: u64, val: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, val.to_f32());
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL2(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter: &mut L2CAgentBase, unk: u64, unk2: u64, unk3: u64, unk4: u64, unk5: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk, unk2, unk3, unk4, unk5);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg5(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter: &mut L2CAgentBase, unk: u64, unk2: u64, unk3: u64, unk4: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk, unk2, unk3, unk4);
    sv_animcmd::ATK_SET_SHIELD_SETOFF_MUL_arg4(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn WHOLE_HIT(fighter: &mut L2CAgentBase, hit_status: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, hit_status);
    sv_animcmd::WHOLE_HIT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FLASH<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32
    >(fighter: &mut L2CAgentBase, unk: A, unk2: B, unk3: C, unk4: D) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::FLASH(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FLASH_FRM<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32
    >(fighter: &mut L2CAgentBase, frame: A, unk: B, unk2: C, unk3: D, unk4: E) {
    fighter.clear_lua_stack();
    lua_args!(fighter, frame.to_f32(), unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::FLASH(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_WORK<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, effect_const: i64, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    let effect = lua_bind::WorkModule::get_int64(fighter.module_accessor, effect_const);
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW_WORK(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

pub unsafe fn EFFECT_FOLLOW_arg11<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool, unk9: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8, unk9);
    sv_animcmd::EFFECT_FOLLOW_arg11(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_POS<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FLW_POS(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_UNSYNC_VIS<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8);
    sv_animcmd::EFFECT_FLW_UNSYNC_VIS(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LANDING_EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::LANDING_EFFECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LANDING_EFFECT_FLIP<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(fighter: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, axis: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, axis);
    sv_animcmd::LANDING_EFFECT_FLIP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_ALPHA<F: ToF32>(fighter: &mut L2CAgentBase, alpha: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, alpha.to_f32());
    sv_animcmd::LAST_EFFECT_SET_ALPHA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_SCALE_W<A: ToF32, B: ToF32, C: ToF32>(fighter: &mut L2CAgentBase, x: A, y: B, z: C) {
    fighter.clear_lua_stack();
    lua_args!(fighter, x.to_f32(), y.to_f32(), z.to_f32());
    sv_animcmd::LAST_EFFECT_SET_SCALE_W(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FOOT_EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::FOOT_EFFECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14);
    sv_animcmd::EFFECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]	
pub unsafe fn EFFECT_FLIP<	
    A: ToF32,	
    B: ToF32,	
    C: ToF32,	
    D: ToF32,	
    E: ToF32,	
    F: ToF32,	
    G: ToF32,	
    H: ToF32,	
    I: ToF32,	
    J: ToF32,	
    K: ToF32,	
    L: ToF32,	
    M: ToF32	
    >(fighter: &mut L2CAgentBase, unk1: Hash40, unk2: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: H, unk11: I, unk12: J, unk13: K, unk14: L, unk15: M, unk16: bool, axis: i32) {	
    fighter.clear_lua_stack();	
    lua_args!(fighter, unk1, unk2, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14.to_f32(), unk15.to_f32(), unk16, axis);	
    sv_animcmd::EFFECT_FLIP(fighter.lua_state_agent);	
    fighter.clear_lua_stack();	
}

#[inline]
pub unsafe fn EFFECT_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32,
    N: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, alpha: N) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, alpha.to_f32());
    sv_animcmd::EFFECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLIP_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    K: ToF32,
    L: ToF32,
    M: ToF32,
    N: ToF32
    >(fighter: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: H, unk9: I,
    unk10: J, unk11: K, unk12: L, unk13: M, unk14: bool, axis: i32, alpha: N) {
    fighter.clear_lua_stack();
    lua_args!(fighter, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), unk11.to_f32(), unk12.to_f32(), unk13.to_f32(), unk14, axis, alpha.to_f32());
    sv_animcmd::EFFECT_FLIP_ALPHA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk8: bool, alpha: H) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk8, alpha.to_f32());
    sv_animcmd::EFFECT_FOLLOW_ALPHA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_FLIP<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32
    >(fighter: &mut L2CAgentBase, right_effect: Hash40, left_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: bool, axis: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, right_effect, left_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(), unk10, axis);
    sv_animcmd::EFFECT_FOLLOW_FLIP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_FLIP_ALPHA<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32
    >(fighter: &mut L2CAgentBase, left_effect: Hash40, right_effect: Hash40, bone: Hash40, x_pos: A, y_pos: B, z_pos: C, x_rot: D, y_rot: E, z_rot: F, size: G, unk10: bool, 
    axis: i32, alpha: H) {
    fighter.clear_lua_stack();
    lua_args!(fighter, left_effect, right_effect, bone, x_pos.to_f32(), y_pos.to_f32(), z_pos.to_f32(), x_rot.to_f32(), y_rot.to_f32(), z_rot.to_f32(), size.to_f32(),
    unk10, axis, alpha.to_f32());
    sv_animcmd::EFFECT_FOLLOW_FLIP_ALPHA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn ENABLE_AREA(fighter: &mut L2CAgentBase, kind: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind);
    sv_animcmd::ENABLE_AREA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn UNABLE_AREA(fighter: &mut L2CAgentBase, kind: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind);
    sv_animcmd::UNABLE_AREA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_SEARCH_SIZE_EXIST<A: ToF32>(fighter: &mut L2CAgentBase, id: u64, size: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, size.to_f32());
    sv_animcmd::SET_SEARCH_SIZE_EXIST(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_PARTICLE_SET_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32
>(fighter: &mut L2CAgentBase, unk: A, unk2: B, unk3: C) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::LAST_PARTICLE_SET_COLOR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32
>(fighter: &mut L2CAgentBase, unk: A, unk2: B, unk3: C) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32(), unk2.to_f32(), unk3.to_f32());
    sv_animcmd::LAST_EFFECT_SET_COLOR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32
>(fighter: &mut L2CAgentBase, unk: A, unk2: B, unk3: C, unk4: D) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::LAST_EFFECT_SET_COLOR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR_FRAME<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32
>(fighter: &mut L2CAgentBase, frame: A, unk: B, unk2: C, unk3: D, unk4: E) {
    fighter.clear_lua_stack();
    lua_args!(fighter, frame.to_f32(), unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32());
    sv_animcmd::LAST_EFFECT_SET_COLOR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn BURN_COLOR_NORMAL(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::BURN_COLOR_NORMAL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn LAST_EFFECT_SET_RATE<F: ToF32>(fighter: &mut L2CAgentBase, rate: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, rate.to_f32());
    sv_animcmd::LAST_EFFECT_SET_RATE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_OFF_KIND(fighter: &mut L2CAgentBase, effect: Hash40, unk: bool, unk2: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, unk, unk2);
    sv_animcmd::EFFECT_OFF_KIND(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_OFF_KIND_WORK(fighter: &mut L2CAgentBase, effect_const: i32, unk: bool, unk2: bool) {
    let effect = lua_bind::WorkModule::get_int64(fighter.module_accessor, effect_const);
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, unk, unk2);
    sv_animcmd::EFFECT_OFF_KIND_WORK(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn COL_NORMAL(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::COL_NORMAL(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FILL_SCREEN_MODEL_COLOR<
    A: ToF32,
    B: ToF32,
    C: ToF32,
    D: ToF32,
    E: ToF32,
    F: ToF32,
    G: ToF32,
    H: ToF32,
    I: ToF32,
    J: ToF32,
    >(fighter: &mut L2CAgentBase, unk1: i32, unk2: A, unk3: B, unk4: C, unk5: D, unk6: E, unk7: F, unk8: G, unk9: H, unk10: I, effect_screen_layer: i32, unk11: J) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32(), effect_screen_layer, unk11.to_f32());
    sv_animcmd::FILL_SCREEN_MODEL_COLOR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CANCEL_FILL_SCREEN<
    A: ToF32,
>(fighter: &mut L2CAgentBase, unk1: i32, unk2: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1, unk2.to_f32());
    sv_animcmd::CANCEL_FILL_SCREEN(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SA_SET(fighter: &mut L2CAgentBase, unk: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk);
    sv_animcmd::SA_SET(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CHECK_FINISH_CAMERA<A: ToF32, B: ToF32>(fighter: &mut L2CAgentBase, unk: A, unk2: B) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32(), unk2.to_f32());
    sv_animcmd::CHECK_FINISH_CAMERA(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_SE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_STATUS(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_STATUS(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_LANDING_SE(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_LANDING_SE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE_NO_3D(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_SE_NO_3D(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SE_REMAIN(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_SE_REMAIN(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_DOWN_SE(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::PLAY_DOWN_SE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_FLY_VOICE(fighter: &mut L2CAgentBase, se1: Hash40, se2: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se1, se2);
    sv_animcmd::PLAY_FLY_VOICE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn STOP_SE(fighter: &mut L2CAgentBase, se: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se);
    sv_animcmd::STOP_SE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_STEP(fighter: &mut L2CAgentBase, step: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, step);
    sv_animcmd::PLAY_STEP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn PLAY_SEQUENCE(fighter: &mut L2CAgentBase, sequence: Hash40) {
    fighter.clear_lua_stack();
    lua_args!(fighter, sequence);
    sv_animcmd::PLAY_SEQUENCE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SET_PLAY_INHIVIT<A: ToF32>(fighter: &mut L2CAgentBase, se: Hash40, unk: A) {
    fighter.clear_lua_stack();
    lua_args!(fighter, se, unk.to_f32());
    sv_animcmd::SET_PLAY_INHIVIT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE_OFF<F: ToF32>(fighter: &mut L2CAgentBase, unk: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk.to_f32());
    sv_animcmd::AFTER_IMAGE_OFF(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE4_ON_arg29(fighter: &mut L2CAgentBase, trail1: Hash40, trail2: Hash40, trail_length: u64, trail_bone1: Hash40, trail_x1: f32, trail_y1: f32,
        trail_z1: f32, trail_bone2: Hash40, trail_x2: f32, trail_y2: f32, trail_z2: f32, unk10: bool, flare: Hash40, flare_bone: Hash40, flare_x: f32, flare_y: f32,
        flare_z: f32, flare_x_rot: f32, flare_y_rot: f32, flare_z_rot: f32, flare_size: f32, unk13: u64, axis: i32, unk15: u64, trail_blend: i32,
        blend: u64, cull: i32, unk16: f32, unk17: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, trail1, trail2, trail_length, trail_bone1, trail_x1, trail_y1, trail_z1, trail_bone2, trail_x2, trail_y2, trail_z2, unk10, flare, flare_bone, flare_x, flare_y, flare_z, flare_x_rot);
    lua_args!(fighter, flare_y_rot, flare_z_rot, flare_size, unk13, axis, unk15, trail_blend, blend, cull, unk16, unk17);
    sv_animcmd::AFTER_IMAGE4_ON_arg29(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AFTER_IMAGE4_ON_WORK_arg29(fighter: &mut L2CAgentBase, trail1: i32, trail2: i32, trail_length: u64, trail_bone1: Hash40, trail_x1: f32, trail_y1: f32,
        trail_z1: f32, trail_bone2: Hash40, trail_x2: f32, trail_y2: f32, trail_z2: f32, unk10: bool, flare: i32, flare_bone: Hash40, flare_x: f32, flare_y: f32,
        flare_z: f32, flare_x_rot: f32, flare_y_rot: f32, flare_z_rot: f32, flare_size: f32, unk13: u64, axis: i32, unk15: u64, trail_blend: i32,
        blend: u64, cull: i32, unk16: f32, unk17: f32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, trail1, trail2, trail_length, trail_bone1, trail_x1, trail_y1, trail_z1, trail_bone2, trail_x2, trail_y2, trail_z2, unk10, flare, flare_bone, flare_x, flare_y, flare_z, flare_x_rot);
    lua_args!(fighter, flare_y_rot, flare_z_rot, flare_size, unk13, axis, unk15, trail_blend, blend, cull, unk16, unk17);
    sv_animcmd::AFTER_IMAGE4_ON_WORK_arg29(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_NO_STOP<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32 , G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, unk.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8);
    sv_animcmd::EFFECT_FOLLOW_NO_STOP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FOLLOW_NO_STOP_FLIP<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32
    >(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: Hash40, unk2: A, unk3: B, unk4: C, unk5: D, unk6: E, unk7: F, unk8: G, unk9: bool, axis: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, unk, unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9, axis);
    sv_animcmd::EFFECT_FOLLOW_NO_STOP_FLIP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_FLW_POS_NO_STOP(fighter: &mut L2CAgentBase, effect: Hash40, bone: Hash40, unk: u64, unk2: u64, unk3: u64, unk4: u64, unk5: u64, unk6: u64, unk7: u64, unk8: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, bone, unk, unk2, unk3, unk4, unk5, unk6, unk7, unk8);
    sv_animcmd::EFFECT_FLW_POS_NO_STOP(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn COL_PRI(fighter: &mut L2CAgentBase, pri: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, pri);
    sv_animcmd::COL_PRI(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_RAD<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32
    >(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32());
    sv_animcmd::AREA_WIND_2ND_RAD(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_RAD_arg9<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32, I: ToF32
    >(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H, unk9: I) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32());
    sv_animcmd::AREA_WIND_2ND_RAD_arg9(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn AREA_WIND_2ND_arg10<
    A: ToF32, B: ToF32, C: ToF32, D: ToF32, E: ToF32, F: ToF32, G: ToF32, H: ToF32, I: ToF32, J: ToF32
    >(fighter: &mut L2CAgentBase, unk1: A, unk2: B, unk3: C, unk4: D, unk5: E, unk6: F, unk7: G, unk8: H, unk9: I, unk10: J) {
    fighter.clear_lua_stack();
    lua_args!(fighter, unk1.to_f32(), unk2.to_f32(), unk3.to_f32(), unk4.to_f32(), unk5.to_f32(), unk6.to_f32(), unk7.to_f32(), unk8.to_f32(), unk9.to_f32(), unk10.to_f32());
    sv_animcmd::AREA_WIND_2ND_arg10(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn FT_ADD_DAMAGE<F: ToF32>(fighter: &mut L2CAgentBase, damage: F) {
    fighter.clear_lua_stack();
    lua_args!(fighter, damage.to_f32());
    sv_animcmd::FT_ADD_DAMAGE(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn CORRECT(fighter: &mut L2CAgentBase, kind: i32) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind);
    sv_animcmd::CORRECT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn RUMBLE_HIT(fighter: &mut L2CAgentBase, kind: Hash40, unk: u64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, kind, unk);
    sv_animcmd::RUMBLE_HIT(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn EFFECT_DETACH_KIND(fighter: &mut L2CAgentBase, effect: Hash40, unk: i64) {
    fighter.clear_lua_stack();
    lua_args!(fighter, effect, unk);
    sv_animcmd::EFFECT_DETACH_KIND(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn REVERSE_LR(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    sv_animcmd::REVERSE_LR(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn SEARCH(fighter: &mut L2CAgentBase, id: u64, part: u64, bone: Hash40, size: f32, x: f32, y: f32, z: f32, x2: Option<f32>, y2: Option<f32>, z2: Option<f32>, 
    collision: i32, hit_status: i32, unk: u64, ground_air: i32, collision_category: i32, collision_parts: i32, unk2: bool) {
    fighter.clear_lua_stack();
    lua_args!(fighter, id, part, bone, size, x, y, z);
    if let Some(x2) = x2 { lua_args!(fighter, x2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(y2) = y2 { lua_args!(fighter, y2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    if let Some(z2) = z2 { lua_args!(fighter, z2); } else { fighter.push_lua_stack(&mut L2CValue::new()); }
    lua_args!(fighter, collision, hit_status, unk, ground_air, collision_category, collision_parts, unk2);
    sv_animcmd::SEARCH(fighter.lua_state_agent);
    fighter.clear_lua_stack();
}

#[inline]
pub unsafe fn game_CaptureCutCommon(fighter: &mut L2CAgentBase) {
    fighter.clear_lua_stack();
    lua_args!(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
    sv_animcmd::ATTACK_ABS(fighter.lua_state_agent);
}

#[macro_export]
macro_rules! grab {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::grab($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! item {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::item($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! shield {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::shield($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! search {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::search($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! slope {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::slope($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! damage {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::damage($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! physics {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::physics($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! camera {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::camera($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! capture {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::capture($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! sv_kinetic_energy {
    ($cmd_name:ident, $fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_kinetic_energy::$cmd_name($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! attack {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::attack($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}


#[macro_export]
macro_rules! effect {
    ($fighter:ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_module_access::effect($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}

#[macro_export]
macro_rules! notify_event_msc_cmd {
    ($fighter: ident, $($arg:expr),* $(,)?) => {
        $fighter.clear_lua_stack();
        lua_args!($fighter, $($arg),*);
        smash::app::sv_battle_object::notify_event_msc_cmd($fighter.lua_state_agent);
        $fighter.pop_lua_stack(1)
    }
}
