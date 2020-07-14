pub mod consts;

use crate::common::consts::*;
use smash::app::{self, lua_bind::*};
use smash::lib::lua_const::*;

pub static mut MENU_STRUCT: consts::TrainingModpackMenu = consts::TrainingModpackMenu {
    hitbox_vis: HitboxVisualization::On,
    di_state: Direction::None,
    left_stick: Direction::None,
    mash_attack_state: Attack::Nair,
    follow_up: Action::Nothing,
    ledge_state: LedgeOption::Random,
    tech_state: TechOption::Random,
    mash_state: Mash::None,
    shield_state: Shield::None,
    defensive_state: Defensive::Random,
    oos_offset: 0,
    reaction_time: 0,
    mash_in_neutral: OnOff::Off,
    fast_fall: OnOff::Off,
    fast_fall_delay: 0,
    falling_aerials: OnOff::Off,
    full_hop: OnOff::Off,
};

pub static mut MENU: &'static mut consts::TrainingModpackMenu = unsafe { &mut MENU_STRUCT };

pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut STAGE_MANAGER_ADDR: usize = 0;

pub fn is_training_mode() -> bool {
true
}
pub fn get_category(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
    return (module_accessor.info >> 28) as u8 as i32;
}

pub unsafe fn is_fighter(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER
}

pub unsafe fn is_operation_cpu(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    if !is_fighter(module_accessor) {
        return false;
    }

    let entry_id_int =
        WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as i32;
    let entry_id = app::FighterEntryID(entry_id_int);
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    let fighter_information =
        FighterManager::get_fighter_information(mgr, entry_id) as *mut app::FighterInformation;

    FighterInformation::is_operation_cpu(fighter_information)
}

pub unsafe fn is_grounded(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = StatusModule::situation_kind(module_accessor) as i32;
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn is_airborne(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = StatusModule::situation_kind(module_accessor) as i32;
    situation_kind == SITUATION_KIND_AIR
}

pub unsafe fn is_idle(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);
    status_kind == FIGHTER_STATUS_KIND_WAIT
}

pub unsafe fn is_in_hitstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);
    (*FIGHTER_STATUS_KIND_DAMAGE..=*FIGHTER_STATUS_KIND_DAMAGE_FALL).contains(&status_kind)
}

pub fn is_shielding(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    unsafe {
        let status_kind = StatusModule::status_kind(module_accessor) as i32;
        (*FIGHTER_STATUS_KIND_GUARD_ON..=*FIGHTER_STATUS_KIND_GUARD_DAMAGE).contains(&status_kind)
    }
}

pub unsafe fn is_in_shieldstun(module_accessor: &mut app::BattleObjectModuleAccessor) -> bool {
    let status_kind = StatusModule::status_kind(module_accessor);
    let prev_status = StatusModule::prev_status_kind(module_accessor, 0);
    // If we are taking shield damage or we are droping shield from taking shield damage we are in hitstun
    status_kind == FIGHTER_STATUS_KIND_GUARD_DAMAGE
        || (prev_status == FIGHTER_STATUS_KIND_GUARD_DAMAGE
            && status_kind == FIGHTER_STATUS_KIND_GUARD_OFF)
}
