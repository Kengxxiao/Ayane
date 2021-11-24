use std::{fs::File, io::Read};

use once_cell::sync::OnceCell;

use crate::clan_battle;

pub static INI_SETTING: OnceCell<serde_json::Value> = OnceCell::new();
pub static UNIT_STATUS_TABLE: OnceCell<serde_json::Value> = OnceCell::new();
pub static RESOURCE_INFO: OnceCell<serde_json::Value> = OnceCell::new();
pub static CLANBATTLE_BOSS_HP : OnceCell<Vec<i32>> = OnceCell::new();

#[derive(Debug)]
pub struct StoryDetail {
    pub story_id: i64,
}
pub static STORY_DATA: OnceCell<Vec<StoryDetail>> = OnceCell::new();
#[derive(Debug)]
pub struct ClanBattleSchedule {
    pub clan_battle_id: i64
}
pub static CLAN_BATTLE_SCHEDULE: OnceCell<Vec<ClanBattleSchedule>> = OnceCell::new();
#[derive(Debug)]
pub struct ClanBattle2MapData {
    pub clan_battle_id: i64,
    pub lap_num_from: i64,
    pub lap_num_to: i64,
    pub boss_id_1: i64,
    pub boss_id_2: i64,
    pub boss_id_3: i64,
    pub boss_id_4: i64,
    pub boss_id_5: i64,
    pub wave_group_id_1: i64,
    pub wave_group_id_2: i64,
    pub wave_group_id_3: i64,
    pub wave_group_id_4: i64,
    pub wave_group_id_5: i64,
}
pub static CLAN_BATTLE_2_MAP_DATA: OnceCell<Vec<ClanBattle2MapData>> = OnceCell::new();
#[derive(Debug)]
pub struct ClanBattle2BossData {
    pub boss_id: i64,
    pub clan_battle_id: i64,
    pub order_num: i64
}
pub static CLAN_BATTLE_2_BOSS_DATA: OnceCell<Vec<ClanBattle2BossData>> = OnceCell::new();
#[derive(Debug)]
pub struct QuestData {
    pub quest_id: i64,
    pub stamina: i64,
    pub limit_time: i64,
    pub wave_group_id_1: i64,
    pub wave_group_id_2: i64,
    pub wave_group_id_3: i64,
}
pub static QUEST_DATA: OnceCell<Vec<QuestData>> = OnceCell::new();
#[derive(Debug)]
pub struct WaveGroupData {
    pub wave_group_id: i64,
    pub enemy_id_1: i64,
    pub enemy_id_2: i64,
    pub enemy_id_3: i64,
    pub enemy_id_4: i64,
    pub enemy_id_5: i64
}
pub static WAVE_GROUP_DATA: OnceCell<Vec<WaveGroupData>> = OnceCell::new();
#[derive(Debug)]
pub struct UnitSkillData {
    pub unit_id: i64,
    pub union_burst: i64,
    pub main_skill_1: i64,
    pub main_skill_2: i64,
    pub main_skill_3: i64,
    pub main_skill_4: i64,
    pub main_skill_5: i64,
    pub main_skill_6: i64,
    pub main_skill_7: i64,
    pub main_skill_8: i64,
    pub main_skill_9: i64,
    pub main_skill_10: i64,
    pub ex_skill_1: i64,
    pub ex_skill_evolution_1: i64,
    pub ex_skill_2: i64,
    pub ex_skill_evolution_2: i64,
    pub ex_skill_3: i64,
    pub ex_skill_evolution_3: i64,
    pub ex_skill_4: i64,
    pub ex_skill_evolution_4: i64,
    pub ex_skill_5: i64,
    pub ex_skill_evolution_5: i64,
    pub sp_skill_1: i64,
    pub sp_skill_2: i64,
    pub sp_skill_3: i64,
    pub sp_skill_4: i64,
    pub sp_skill_5: i64,
    pub union_burst_evolution: i64,
    pub main_skill_evolution_1: i64,
    pub main_skill_evolution_2: i64
}
pub static UNIT_SKILL_DATA: OnceCell<Vec<UnitSkillData>> = OnceCell::new();

#[derive(Debug)]
pub struct CharaStoryStatus {
    pub chara_id: Vec<i32>,
    pub status_add: Vec<(String, i32)>
}
pub static CHARA_STORY_STATUS: OnceCell<Vec<CharaStoryStatus>> = OnceCell::new();

#[derive(Debug)]
pub struct UnitData {
    pub unit_id: i64,
    pub rarity: i64,
    pub comment: String,
    pub search_area_width: i64
}
pub static UNIT_DATA: OnceCell<Vec<UnitData>> = OnceCell::new();

#[derive(Debug)]
pub struct UnitPromotion {
    pub unit_id: i64,
    pub promotion_level: i64,
    pub equip_slot_1: i64,
    pub equip_slot_2: i64,
    pub equip_slot_3: i64,
    pub equip_slot_4: i64,
    pub equip_slot_5: i64,
    pub equip_slot_6: i64
}
pub static UNIT_PROMOTION: OnceCell<Vec<UnitPromotion>> = OnceCell::new();

#[derive(Debug)]
pub struct UnitRarity {
    pub unit_id: i64,
    pub rarity: i64,
    pub growth: Vec<(String, f64, f64)>
}
pub static UNIT_RARITY: OnceCell<Vec<UnitRarity>> = OnceCell::new();

#[derive(Debug)]
pub struct UnitPromotionStatus {
    pub unit_id: i64,
    pub promotion_level: i64,
    pub base: Vec<(String, f64)>
}
pub static UNIT_PROMOTION_STATUS: OnceCell<Vec<UnitPromotionStatus>> = OnceCell::new();

#[derive(Debug)]
pub struct EquipmentData {
    pub equipment_id: i64,
    pub promotion_level: i64,
    pub euqipment_enhance_point: i64,
    pub base: Vec<(String, f64)>
}
pub static EQUIPMENT_DATA: OnceCell<Vec<EquipmentData>> = OnceCell::new();

#[derive(Debug)]
pub struct EquipmentEnhanceRate {
    pub equipment_id: i64,
    pub promotion_level: i64,
    pub base: Vec<(String, f64)>
}
pub static EQUIPMENT_ENHANCE_RATE: OnceCell<Vec<EquipmentEnhanceRate>> = OnceCell::new();

#[derive(Debug)]
pub struct UniqueEquipmentData {
    pub equipment_id: i64,
    pub promotion_level: i64,
    pub euqipment_enhance_point: i64,
    pub base: Vec<(String, f64)>
}
pub static UNIQUE_EQUIPMENT_DATA: OnceCell<Vec<UniqueEquipmentData>> = OnceCell::new();

#[derive(Debug)]
pub struct UniqueEquipmentEnhanceRate {
    pub equipment_id: i64,
    pub promotion_level: i64,
    pub base: Vec<(String, f64)>
}
pub static UNIQUE_EQUIPMENT_ENHANCE_RATE: OnceCell<Vec<UniqueEquipmentEnhanceRate>> = OnceCell::new();

#[derive(Debug)]
pub struct UnitUniqueEquip {
    pub unit_id: i64,
    pub equip_slot: i64,
    pub equip_id: i64
}
pub static UNIT_UNIQUE_EQUIP : OnceCell<Vec<UnitUniqueEquip>> = OnceCell::new();

#[derive(Debug)]
pub struct SkillAction {
    pub action_id: i64,
    pub action_type: i64,
    pub action_detail_1: i64,
    pub action_value_2: f64,
    pub action_value_3: f64
}
pub static SKILL_ACTION : OnceCell<Vec<SkillAction>> = OnceCell::new();

#[derive(Debug)]
pub struct SkillData {
    pub skill_id: i64,
    pub actions: Vec<i64>
}
pub static SKILL_DATA: OnceCell<Vec<SkillData>> = OnceCell::new();

#[derive(Debug)]
pub struct SkillCost {
    pub target_level: i64,
    pub cost: i64
}
pub static SKILL_COST: OnceCell<Vec<SkillCost>> = OnceCell::new();

#[derive(Debug)]
pub struct ExperienceUnit {
    pub unit_level: i64,
    pub total_exp: i64
}
pub static EXPERIENCE_UNIT: OnceCell<Vec<ExperienceUnit>> = OnceCell::new();

#[derive(Debug)]
pub struct EquipmentEnhanceData {
    pub promotion_level: i64,
    pub equipment_enhance_level: i64,
    pub needed_point: i64,
    pub total_point: i64
}
pub static EQUIPMENT_ENHANCE_DATA: OnceCell<Vec<EquipmentEnhanceData>> = OnceCell::new();

#[derive(Debug)]
pub struct UniqueEquipmentEnhanceData {
    pub enhance_level: i64,
    pub total_point: i64,
    pub needed_mana: i64,
    pub rank: i64
}
pub static UNIQUE_EQUIPMENT_ENHANCE_DATA: OnceCell<Vec<UniqueEquipmentEnhanceData>> = OnceCell::new();

#[derive(Debug)]
pub struct UniqueEquipmentRankup {
    pub equip_id: i64,
    pub unique_equip_rank: i64,
    pub crafted_cost: i64,
    pub unit_level: i64
}
pub static UNIQUE_EQUIPMENT_RANKUP: OnceCell<Vec<UniqueEquipmentRankup>> = OnceCell::new();

#[derive(Debug)]
pub struct ItemData {
    pub item_id: i64,
    pub item_type: i64
}
pub static ITEM_DATA: OnceCell<Vec<ItemData>> = OnceCell::new();

#[derive(Debug)]
pub struct EnemyParameter {
    pub unit_id: i64,
    pub level: i64,
    pub union_burst_level: i64,
    pub main_skill_lv_1: i64,
    pub main_skill_lv_2: i64,
    pub main_skill_lv_3: i64,
    pub main_skill_lv_4: i64,
    pub main_skill_lv_5: i64,
    pub main_skill_lv_6: i64,
    pub main_skill_lv_7: i64,
    pub main_skill_lv_8: i64,
    pub main_skill_lv_9: i64,
    pub main_skill_lv_10: i64,
    pub enemy_id: i64,
    pub hp: i64,
}
pub static ENEMY_PARAMETER: OnceCell<Vec<EnemyParameter>> = OnceCell::new();

pub fn init_priconne_server_db() {
    let conn = sqlite::open("master.mdb").unwrap();
    let mut stmt = conn.prepare("select story_id from story_detail").unwrap();
    let mut story_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        story_data.push(StoryDetail {
            story_id: stmt.read::<i64>(0).unwrap()
        });
    }
    STORY_DATA.set(story_data).unwrap();
    let mut stmt = conn.prepare("select clan_battle_id from clan_battle_schedule").unwrap();
    let mut clan_battle_schedule = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        clan_battle_schedule.push(ClanBattleSchedule {
            clan_battle_id: stmt.read::<i64>(0).unwrap()
        });
    }
    CLAN_BATTLE_SCHEDULE.set(clan_battle_schedule).unwrap();
    let mut stmt = conn.prepare("select clan_battle_id, lap_num_from, lap_num_to, boss_id_1, boss_id_2, boss_id_3, boss_id_4, boss_id_5, wave_group_id_1, wave_group_id_2, wave_group_id_3, wave_group_id_4, wave_group_id_5 from clan_battle_2_map_data").unwrap();
    let mut clan_battle_2_map_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        clan_battle_2_map_data.push(ClanBattle2MapData {
            clan_battle_id: stmt.read::<i64>(0).unwrap(),
            lap_num_from: stmt.read::<i64>(1).unwrap(),
            lap_num_to: stmt.read::<i64>(2).unwrap(),
            boss_id_1: stmt.read::<i64>(3).unwrap(),
            boss_id_2: stmt.read::<i64>(4).unwrap(),
            boss_id_3: stmt.read::<i64>(5).unwrap(),
            boss_id_4: stmt.read::<i64>(6).unwrap(),
            boss_id_5: stmt.read::<i64>(7).unwrap(),
            wave_group_id_1: stmt.read::<i64>(8).unwrap(),
            wave_group_id_2: stmt.read::<i64>(9).unwrap(),
            wave_group_id_3: stmt.read::<i64>(10).unwrap(),
            wave_group_id_4: stmt.read::<i64>(11).unwrap(),
            wave_group_id_5: stmt.read::<i64>(12).unwrap(),
        });
    }
    CLAN_BATTLE_2_MAP_DATA.set(clan_battle_2_map_data).unwrap();
    let mut stmt = conn.prepare("select boss_id, clan_battle_id, order_num from clan_battle_2_boss_data").unwrap();
    let mut clan_battle_2_boss_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        clan_battle_2_boss_data.push(ClanBattle2BossData {
            boss_id: stmt.read::<i64>(0).unwrap(),
            clan_battle_id: stmt.read::<i64>(1).unwrap(),
            order_num: stmt.read::<i64>(2).unwrap()
        })
    }
    CLAN_BATTLE_2_BOSS_DATA.set(clan_battle_2_boss_data).unwrap();
    let mut stmt = conn.prepare("select quest_id, stamina, limit_time, wave_group_id_1, wave_group_id_2, wave_group_id_3 from quest_data").unwrap();
    let mut quest_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        quest_data.push(QuestData {
            quest_id: stmt.read::<i64>(0).unwrap(),
            stamina: stmt.read::<i64>(1).unwrap(),
            limit_time: stmt.read::<i64>(2).unwrap(),
            wave_group_id_1: stmt.read::<i64>(3).unwrap(),
            wave_group_id_2: stmt.read::<i64>(4).unwrap(),
            wave_group_id_3: stmt.read::<i64>(5).unwrap()
        })
    }
    QUEST_DATA.set(quest_data).unwrap();
    let mut stmt = conn.prepare("select wave_group_id, enemy_id_1, enemy_id_2, enemy_id_3, enemy_id_4, enemy_id_5 from wave_group_data").unwrap();
    let mut wave_group_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        wave_group_data.push(WaveGroupData {
            wave_group_id: stmt.read::<i64>(0).unwrap(),
            enemy_id_1: stmt.read::<i64>(1).unwrap(),
            enemy_id_2: stmt.read::<i64>(2).unwrap(),
            enemy_id_3: stmt.read::<i64>(3).unwrap(),
            enemy_id_4: stmt.read::<i64>(4).unwrap(),
            enemy_id_5: stmt.read::<i64>(5).unwrap()
        })
    }
    WAVE_GROUP_DATA.set(wave_group_data).unwrap();
    let mut stmt = conn.prepare("select unit_id, union_burst, main_skill_1, main_skill_2, main_skill_3, main_skill_4, main_skill_5, main_skill_6, main_skill_7, main_skill_8, main_skill_9, main_skill_10,
    ex_skill_1, ex_skill_evolution_1, ex_skill_2, ex_skill_evolution_2, ex_skill_3, ex_skill_evolution_3, ex_skill_4, ex_skill_evolution_4, ex_skill_5, ex_skill_evolution_5, sp_skill_1, sp_skill_2, sp_skill_3, sp_skill_4, sp_skill_5,
    union_burst_evolution, main_skill_evolution_1, main_skill_evolution_2 from unit_skill_data").unwrap();
    let mut unit_skill_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unit_skill_data.push(UnitSkillData {
            unit_id: stmt.read::<i64>(0).unwrap(),
            union_burst: stmt.read::<i64>(1).unwrap(),
            main_skill_1: stmt.read::<i64>(2).unwrap(),
            main_skill_2: stmt.read::<i64>(3).unwrap(),
            main_skill_3: stmt.read::<i64>(4).unwrap(),
            main_skill_4: stmt.read::<i64>(5).unwrap(),
            main_skill_5: stmt.read::<i64>(6).unwrap(),
            main_skill_6: stmt.read::<i64>(7).unwrap(),
            main_skill_7: stmt.read::<i64>(8).unwrap(),
            main_skill_8: stmt.read::<i64>(9).unwrap(),
            main_skill_9: stmt.read::<i64>(10).unwrap(),
            main_skill_10: stmt.read::<i64>(11).unwrap(),
            ex_skill_1: stmt.read::<i64>(12).unwrap(),
            ex_skill_evolution_1: stmt.read::<i64>(13).unwrap(),
            ex_skill_2: stmt.read::<i64>(14).unwrap(),
            ex_skill_evolution_2: stmt.read::<i64>(15).unwrap(),
            ex_skill_3: stmt.read::<i64>(16).unwrap(),
            ex_skill_evolution_3: stmt.read::<i64>(17).unwrap(),
            ex_skill_4: stmt.read::<i64>(18).unwrap(),
            ex_skill_evolution_4: stmt.read::<i64>(19).unwrap(),
            ex_skill_5: stmt.read::<i64>(20).unwrap(),
            ex_skill_evolution_5: stmt.read::<i64>(21).unwrap(),
            sp_skill_1: stmt.read::<i64>(22).unwrap(),
            sp_skill_2: stmt.read::<i64>(23).unwrap(),
            sp_skill_3: stmt.read::<i64>(24).unwrap(),
            sp_skill_4: stmt.read::<i64>(25).unwrap(),
            sp_skill_5: stmt.read::<i64>(26).unwrap(),
            union_burst_evolution: stmt.read::<i64>(27).unwrap(),
            main_skill_evolution_1: stmt.read::<i64>(28).unwrap(),
            main_skill_evolution_2: stmt.read::<i64>(29).unwrap(),
        })
    }
    UNIT_SKILL_DATA.set(unit_skill_data).unwrap();
    let mut stmt = conn.prepare("select unit_id, rarity, comment, search_area_width from unit_data").unwrap();
    let mut unit_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unit_data.push(UnitData {
            unit_id: stmt.read::<i64>(0).unwrap(),
            rarity: stmt.read::<i64>(1).unwrap(),
            comment: stmt.read::<String>(2).unwrap(),
            search_area_width: stmt.read::<i64>(3).unwrap()
        })
    }
    UNIT_DATA.set(unit_data).unwrap();
    let mut stmt = conn.prepare("select unit_id, promotion_level, equip_slot_1, equip_slot_2, equip_slot_3, equip_slot_4, equip_slot_5, equip_slot_6 from unit_promotion").unwrap();
    let mut unit_promotion = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unit_promotion.push(UnitPromotion {
            unit_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            equip_slot_1: stmt.read::<i64>(2).unwrap(),
            equip_slot_2: stmt.read::<i64>(3).unwrap(),
            equip_slot_3: stmt.read::<i64>(4).unwrap(),
            equip_slot_4: stmt.read::<i64>(5).unwrap(),
            equip_slot_5: stmt.read::<i64>(6).unwrap(),
            equip_slot_6: stmt.read::<i64>(7).unwrap()
        })
    }
    UNIT_PROMOTION.set(unit_promotion).unwrap();

    let mut status_table_file = File::open("STATUS_TABLE.json").unwrap();
    let mut status_table_data = String::new();
    status_table_file.read_to_string(&mut status_table_data).unwrap();
    let status_table : serde_json::Value = serde_json::from_str(&status_table_data).unwrap();

    let mut unit_status_table_file = File::open("UNIT_STATUS_TABLE.json").unwrap();
    let mut unit_status_table_data = String::new();
    unit_status_table_file.read_to_string(&mut unit_status_table_data).unwrap();
    let unit_status_table : serde_json::Value = serde_json::from_str(&unit_status_table_data).unwrap();

    let mut stmt = conn.prepare("select status_type_1, status_type_2, status_type_3, status_type_4, status_type_5, status_rate_1, status_rate_2, status_rate_3, status_rate_4, status_rate_5, chara_id_1, chara_id_2, chara_id_3, chara_id_4, chara_id_5, chara_id_6 from chara_story_status").unwrap();
    let mut chara_story_status = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut char_ids = vec![];
        let mut status_t = vec![];
        for c_i in 10..16 {
            let charid = stmt.read::<i64>(c_i).unwrap();
            if charid != 0 {
                char_ids.push(charid as i32);
            }
        }
        for type_i in  0..5 {
            let status_type = stmt.read::<i64>(type_i).unwrap();
            let status_rate = stmt.read::<i64>(type_i + 5).unwrap();
            if status_type == 0 {
                break;
            }
            let cname = status_table[status_type.to_string()].as_str().unwrap();
            status_t.push((cname.to_string(), status_rate as i32));
        }
        chara_story_status.push(CharaStoryStatus {
            chara_id: char_ids,
            status_add: status_t
        });
    }
    CHARA_STORY_STATUS.set(chara_story_status).unwrap();

    let mut stmt = conn.prepare("select unit_id, rarity, hp, hp_growth, atk, atk_growth, def, def_growth, magic_str, magic_str_growth, magic_def, magic_def_growth, physical_critical, physical_critical_growth, magic_critical, magic_critical_growth, wave_hp_recovery, wave_hp_recovery_growth, wave_energy_recovery, wave_energy_recovery_growth, hp_recovery_rate, hp_recovery_rate_growth, physical_penetrate, physical_penetrate_growth, magic_penetrate, magic_penetrate_growth, life_steal, life_steal_growth, dodge, dodge_growth, energy_reduce_rate, energy_reduce_rate_growth, energy_recovery_rate, energy_recovery_rate_growth, accuracy, accuracy_growth from unit_rarity").unwrap();
    let mut unit_rarity = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut st_base = vec![];
        let char_id = stmt.read::<i64>(0).unwrap();
        let rarity = stmt.read::<i64>(1).unwrap();
        for s_i in 1..=17 {
            let s = unit_status_table[s_i.to_string()].as_str().unwrap().to_string();
            st_base.push((s, stmt.read::<f64>(s_i * 2).unwrap(), stmt.read::<f64>(s_i * 2 + 1).unwrap()));
        }
        unit_rarity.push(UnitRarity {
            unit_id: char_id,
            rarity,
            growth: st_base
        });
    }
    UNIT_RARITY.set(unit_rarity).unwrap();
    
    let mut stmt = conn.prepare("select unit_id, promotion_level, hp, atk, def, magic_str, magic_def, physical_critical, magic_critical, wave_hp_recovery, wave_energy_recovery, hp_recovery_rate, physical_penetrate, magic_penetrate, life_steal, dodge, energy_reduce_rate, energy_recovery_rate, accuracy from unit_promotion_status").unwrap();
    let mut unit_promotion_status = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut pro_base = vec![];
        for s_i in 1..=17 {
            pro_base.push((unit_status_table[s_i.to_string()].as_str().unwrap().to_string(), stmt.read::<f64>(s_i + 1).unwrap()));
        }
        unit_promotion_status.push(UnitPromotionStatus {
            unit_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            base: pro_base
        });
    }
    UNIT_PROMOTION_STATUS.set(unit_promotion_status).unwrap();

    let mut stmt = conn.prepare("select equipment_id, promotion_level, equipment_enhance_point, hp, atk, def, magic_str, magic_def, physical_critical, magic_critical, wave_hp_recovery, wave_energy_recovery, hp_recovery_rate, physical_penetrate, magic_penetrate, life_steal, dodge, energy_reduce_rate, energy_recovery_rate, accuracy from equipment_data").unwrap();
    let mut equipment_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut eq_base = vec![];
        for s_i in 1..=17 {
            eq_base.push((unit_status_table[s_i.to_string()].as_str().unwrap().to_string(), stmt.read::<f64>(s_i + 2).unwrap()));
        }
        equipment_data.push(EquipmentData {
            equipment_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            euqipment_enhance_point: stmt.read::<i64>(2).unwrap(),
            base: eq_base
        })
    }
    EQUIPMENT_DATA.set(equipment_data).unwrap();

    let mut stmt = conn.prepare("select equipment_id, promotion_level, hp, atk, def, magic_str, magic_def, physical_critical, magic_critical, wave_hp_recovery, wave_energy_recovery, hp_recovery_rate, physical_penetrate, magic_penetrate, life_steal, dodge, energy_reduce_rate, energy_recovery_rate, accuracy from equipment_enhance_rate").unwrap();
    let mut equipment_enhance_rate = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut eq_base = vec![];
        for s_i in 1..=17 {
            eq_base.push((unit_status_table[s_i.to_string()].as_str().unwrap().to_string(), stmt.read::<f64>(s_i + 1).unwrap()));
        }
        equipment_enhance_rate.push(EquipmentEnhanceRate {
            equipment_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            base: eq_base
        })
    }
    EQUIPMENT_ENHANCE_RATE.set(equipment_enhance_rate).unwrap();

    let mut stmt = conn.prepare("select equipment_id, promotion_level, equipment_enhance_point, hp, atk, def, magic_str, magic_def, physical_critical, magic_critical, wave_hp_recovery, wave_energy_recovery, hp_recovery_rate, physical_penetrate, magic_penetrate, life_steal, dodge, energy_reduce_rate, energy_recovery_rate, accuracy from unique_equipment_data").unwrap();
    let mut equipment_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut eq_base = vec![];
        for s_i in 1..=17 {
            eq_base.push((unit_status_table[s_i.to_string()].as_str().unwrap().to_string(), stmt.read::<f64>(s_i + 2).unwrap()));
        }
        equipment_data.push(UniqueEquipmentData {
            equipment_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            euqipment_enhance_point: stmt.read::<i64>(2).unwrap(),
            base: eq_base
        })
    }
    UNIQUE_EQUIPMENT_DATA.set(equipment_data).unwrap();

    let mut stmt = conn.prepare("select equipment_id, promotion_level, hp, atk, def, magic_str, magic_def, physical_critical, magic_critical, wave_hp_recovery, wave_energy_recovery, hp_recovery_rate, physical_penetrate, magic_penetrate, life_steal, dodge, energy_reduce_rate, energy_recovery_rate, accuracy from unique_equipment_enhance_rate").unwrap();
    let mut equipment_enhance_rate = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut eq_base = vec![];
        for s_i in 1..=17 {
            eq_base.push((unit_status_table[s_i.to_string()].as_str().unwrap().to_string(), stmt.read::<f64>(s_i + 1).unwrap()));
        }
        equipment_enhance_rate.push(UniqueEquipmentEnhanceRate {
            equipment_id: stmt.read::<i64>(0).unwrap(),
            promotion_level: stmt.read::<i64>(1).unwrap(),
            base: eq_base
        })
    }
    UNIQUE_EQUIPMENT_ENHANCE_RATE.set(equipment_enhance_rate).unwrap();

    let mut stmt = conn.prepare("select unit_id, equip_slot, equip_id from unit_unique_equip").unwrap();
    let mut unit_unique_equip = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unit_unique_equip.push(UnitUniqueEquip {
            unit_id: stmt.read::<i64>(0).unwrap(),
            equip_slot: stmt.read::<i64>(1).unwrap(),
            equip_id: stmt.read::<i64>(2).unwrap()
        })
    }
    UNIT_UNIQUE_EQUIP.set(unit_unique_equip).unwrap();

    let mut stmt = conn.prepare("select action_id, action_type, action_detail_1, action_value_2, action_value_3 from skill_action").unwrap();
    let mut skill_action = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        skill_action.push(SkillAction {
            action_id: stmt.read::<i64>(0).unwrap(),
            action_type: stmt.read::<i64>(1).unwrap(),
            action_detail_1: stmt.read::<i64>(2).unwrap(),
            action_value_2: stmt.read::<f64>(3).unwrap(),
            action_value_3: stmt.read::<f64>(4).unwrap()
        })
    }
    SKILL_ACTION.set(skill_action).unwrap();

    let mut stmt = conn.prepare("select skill_id, action_1, action_2, action_3, action_4, action_5, action_6, action_7 from skill_data").unwrap();
    let mut skill_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        let mut ids = vec![];
        for s in 1..=7 {
            ids.push(stmt.read::<i64>(s).unwrap());
        }
        skill_data.push(SkillData {
            skill_id: stmt.read::<i64>(0).unwrap(),
            actions: ids
        })
    }
    SKILL_DATA.set(skill_data).unwrap();

    let mut stmt = conn.prepare("select target_level, cost from skill_cost").unwrap();
    let mut skill_cost = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        skill_cost.push(SkillCost {
            target_level: stmt.read::<i64>(0).unwrap(),
            cost: stmt.read::<i64>(1).unwrap()
        })
    }
    SKILL_COST.set(skill_cost).unwrap();

    let mut stmt = conn.prepare("select unit_level, total_exp from experience_unit").unwrap();
    let mut experience_unit = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        experience_unit.push(ExperienceUnit {
            unit_level: stmt.read::<i64>(0).unwrap(),
            total_exp: stmt.read::<i64>(1).unwrap()
        })
    }
    experience_unit.reverse();
    //println!("{:?}", experience_unit);
    EXPERIENCE_UNIT.set(experience_unit).unwrap();

    let mut stmt = conn.prepare("select promotion_level, equipment_enhance_level, needed_point, total_point from equipment_enhance_data").unwrap();
    let mut equipment_enhance_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        equipment_enhance_data.push(EquipmentEnhanceData {
            promotion_level: stmt.read::<i64>(0).unwrap(),
            equipment_enhance_level: stmt.read::<i64>(1).unwrap(),
            needed_point: stmt.read::<i64>(2).unwrap(),
            total_point: stmt.read::<i64>(3).unwrap()
        })
    }
    equipment_enhance_data.reverse();
    EQUIPMENT_ENHANCE_DATA.set(equipment_enhance_data).unwrap();

    let mut stmt = conn.prepare("select enhance_level, total_point, needed_mana, rank from unique_equipment_enhance_data").unwrap();
    let mut unique_equipment_enhance_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unique_equipment_enhance_data.push(UniqueEquipmentEnhanceData {
            enhance_level: stmt.read::<i64>(0).unwrap(),
            total_point: stmt.read::<i64>(1).unwrap(),
            needed_mana: stmt.read::<i64>(2).unwrap(),
            rank: stmt.read::<i64>(3).unwrap()
        })
    }
    UNIQUE_EQUIPMENT_ENHANCE_DATA.set(unique_equipment_enhance_data).unwrap();

    let mut stmt = conn.prepare("select equip_id, unique_equip_rank, crafted_cost, unit_level from unique_equipment_rankup").unwrap();
    let mut unique_equipment_rankup = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        unique_equipment_rankup.push(UniqueEquipmentRankup {
            equip_id: stmt.read::<i64>(0).unwrap(),
            unique_equip_rank: stmt.read::<i64>(1).unwrap(),
            crafted_cost: stmt.read::<i64>(2).unwrap(),
            unit_level: stmt.read::<i64>(3).unwrap()
        })
    }
    UNIQUE_EQUIPMENT_RANKUP.set(unique_equipment_rankup).unwrap();

    let mut stmt = conn.prepare("select item_id, item_type from item_data").unwrap();
    let mut item_data = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        item_data.push(ItemData {
            item_id: stmt.read::<i64>(0).unwrap(),
            item_type: stmt.read::<i64>(1).unwrap()
        })
    }
    ITEM_DATA.set(item_data).unwrap();

    let mut stmt = conn.prepare("select unit_id, level, union_burst_level,
   main_skill_lv_1, main_skill_lv_2, main_skill_lv_3, main_skill_lv_4, main_skill_lv_5, main_skill_lv_6, main_skill_lv_7, main_skill_lv_8, main_skill_lv_9, main_skill_lv_10, enemy_id, hp from enemy_parameter").unwrap();
    let mut enemy_parameter = vec![];
    while let sqlite::State::Row = stmt.next().unwrap() {
        enemy_parameter.push(EnemyParameter {
            unit_id: stmt.read::<i64>(0).unwrap(),
            level: stmt.read::<i64>(1).unwrap(),
            union_burst_level: stmt.read::<i64>(2).unwrap(),
            main_skill_lv_1: stmt.read::<i64>(3).unwrap(),
            main_skill_lv_2: stmt.read::<i64>(4).unwrap(),
            main_skill_lv_3: stmt.read::<i64>(5).unwrap(),
            main_skill_lv_4: stmt.read::<i64>(6).unwrap(),
            main_skill_lv_5: stmt.read::<i64>(7).unwrap(),
            main_skill_lv_6: stmt.read::<i64>(8).unwrap(),
            main_skill_lv_7: stmt.read::<i64>(9).unwrap(),
            main_skill_lv_8: stmt.read::<i64>(10).unwrap(),
            main_skill_lv_9: stmt.read::<i64>(11).unwrap(),
            main_skill_lv_10: stmt.read::<i64>(12).unwrap(),
            enemy_id: stmt.read::<i64>(13).unwrap(),
            hp: stmt.read::<i64>(14).unwrap()
        })
    }
    ENEMY_PARAMETER.set(enemy_parameter).unwrap();

    UNIT_STATUS_TABLE.set(unit_status_table).unwrap();

    let mut ini_file = File::open("INI_SETTING.json").unwrap();
    let mut ini_data = String::new();
    ini_file.read_to_string(&mut ini_data).unwrap();
    INI_SETTING.set(serde_json::from_str(&ini_data).unwrap()).unwrap();

    let mut res_info_file = File::open("RESOURCE_INFO.json").unwrap();
    let mut res_info_data = String::new();
    res_info_file.read_to_string(&mut res_info_data).unwrap();
    RESOURCE_INFO.set(serde_json::from_str(&res_info_data).unwrap()).unwrap();

    let mut cb_config_file = File::open("CB_CONFIG.txt").unwrap();
    let mut cb_config_data = String::new();
    cb_config_file.read_to_string(&mut cb_config_data).unwrap();
    clan_battle::CLANBATTLE_CONFIG.lock().unwrap().clanbattle_id = cb_config_data.parse::<i64>().unwrap();

    CLANBATTLE_BOSS_HP.set(vec![6000000, 8000000, 10000000, 12000000, 15000000]).unwrap();
}