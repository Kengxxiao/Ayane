use std::sync::Mutex;

use serde_json::json;

use crate::global_data;
use crate::global_data::{ClanBattle2BossData, WaveGroupData, ClanBattle2MapData, EnemyParameter, UnitSkillData};

pub struct ClanBattleConfig {
    pub clanbattle_id: i64,
    pub lap_num: i64
}

lazy_static! {
    pub static ref CLANBATTLE_CONFIG : Mutex<ClanBattleConfig> = Mutex::new(ClanBattleConfig {
        clanbattle_id: 1001,
        lap_num: 35
    });
}

pub fn check_clan_battle_id_safe(clan_battle_id: i64) -> bool {
    let cb = global_data::CLAN_BATTLE_SCHEDULE.get().unwrap().iter().find(|x| x.clan_battle_id == clan_battle_id);
    match cb {
        Some(_) => true,
        None => false
    }
}

pub fn check_clan_battle_id_exists(clan_battle_id: i64) -> bool {
    match global_data::CLAN_BATTLE_2_MAP_DATA.get().unwrap().iter().find(|x| x.clan_battle_id == clan_battle_id) {
        Some(_) => true,
        None => false
    }
}

pub fn get_clan_battle_id_list() -> Vec<i64> {
    let mut cb : Vec<i64> = global_data::CLAN_BATTLE_2_MAP_DATA.get().unwrap().iter().map(|x| x.clan_battle_id).collect();
    cb.dedup();
    cb
}

pub fn get_clan_battle_boss_info(lap_num: i64, clan_battle_id: i64) -> Vec<serde_json::Value> {
    let now_cb : Vec<&ClanBattle2MapData> = global_data::CLAN_BATTLE_2_MAP_DATA.get().unwrap().iter().filter(|x| x.clan_battle_id == clan_battle_id).collect();
    let mut boss_info = vec![];
    for boss in now_cb {
        if (boss.lap_num_from <= lap_num && lap_num <= boss.lap_num_to) || boss.lap_num_to == -1 {
            let boss_id = vec![boss.boss_id_1, boss.boss_id_2, boss.boss_id_3, boss.boss_id_4, boss.boss_id_5];
            let wave_group_id = vec![boss.wave_group_id_1, boss.wave_group_id_2, boss.wave_group_id_3, boss.wave_group_id_4, boss.wave_group_id_5];
            let boss_data_cell = global_data::CLAN_BATTLE_2_BOSS_DATA.get().unwrap();
            let wave_group_cell = global_data::WAVE_GROUP_DATA.get().unwrap();
            let enemy_parameter_cell = global_data::ENEMY_PARAMETER.get().unwrap();
            for idx in 0..boss_id.len() {
                let boss_data : &ClanBattle2BossData = boss_data_cell.iter().find(|x| x.boss_id == boss_id[idx]).unwrap();
                let wave_group : &WaveGroupData = wave_group_cell.iter().find(|x| x.wave_group_id == wave_group_id[idx]).unwrap();
                let enemy_param : &EnemyParameter = enemy_parameter_cell.iter().find(|x| x.enemy_id == wave_group.enemy_id_1).unwrap();
                boss_info.push(json!({
                    "order_num": boss_data.order_num,
                    "enemy_id": wave_group.enemy_id_1,
                    "max_hp": enemy_param.hp,
                    "current_hp": enemy_param.hp
                }));
            }
            break;
        }
    }
    boss_info
}

pub fn get_clan_battle_enemy_data(enemy_id: i64) -> serde_json::Value {
    let enemy_data : &EnemyParameter = global_data::ENEMY_PARAMETER.get().unwrap().iter().find(|x| x.enemy_id == enemy_id).unwrap();
    let unit_skill_data : &UnitSkillData = global_data::UNIT_SKILL_DATA.get().unwrap().iter().find(|x| x.unit_id == enemy_data.unit_id).unwrap();

    let mut union_burst = vec![];
    let mut main_skill = vec![];
    
    let ms_list = vec![unit_skill_data.main_skill_1, enemy_data.main_skill_lv_1,
    unit_skill_data.main_skill_2, enemy_data.main_skill_lv_2,
    unit_skill_data.main_skill_3, enemy_data.main_skill_lv_3,
    unit_skill_data.main_skill_4, enemy_data.main_skill_lv_4,
    unit_skill_data.main_skill_5, enemy_data.main_skill_lv_5,
    unit_skill_data.main_skill_6, enemy_data.main_skill_lv_6,
    unit_skill_data.main_skill_7, enemy_data.main_skill_lv_7,
    unit_skill_data.main_skill_8, enemy_data.main_skill_lv_8,
    unit_skill_data.main_skill_9, enemy_data.main_skill_lv_9,
    unit_skill_data.main_skill_10, enemy_data.main_skill_lv_10];

    union_burst.push(json!({
        "skill_id": unit_skill_data.union_burst,
        "skill_level": enemy_data.union_burst_level
    }));
    for idx in 0..10 {
        let midx = 2 * idx;
        if ms_list[midx] == 0 {continue;}
        main_skill.push(json!({
            "skill_id": ms_list[midx],
            "skill_level": ms_list[midx + 1]
        }));
    }

    json!({
        "id": enemy_id,
        "get_time": 0,
        "unit_rarity": 1,
        "unit_level": enemy_data.level,
        "unit_exp": 0,
        "promotion_level": 1,
        "union_burst": union_burst,
        "main_skill": main_skill,
        "ex_skill": [],
        "equip_slot": [],
        "free_skill": [],
        "unique_equip_slot": []
    })
}