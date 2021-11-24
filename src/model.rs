use crate::schema::user_session;
use crate::schema::player_data;
use crate::schema::player_unit_data;
use crate::schema::player_deck_list;
use crate::schema::player_story;
use crate::schema::player_present;
use crate::schema::player_item;
use crate::schema::player_arena_data;
use crate::schema::player_arena_log;
use crate::schema::player_equip;
use crate::schema::clan_battle_log;

use serde::Deserialize;

#[derive(Queryable, Debug, AsChangeset, Deserialize)]
#[table_name = "user_session"]
pub struct BaseUserSession {
    pub id: i32,
    pub viewer_id: i64,
    pub request_id: String,
    pub next_sid: String,
    pub short_udid: i64
}

#[derive(Insertable)]
#[table_name = "user_session"]
pub struct NewUserSession {
    pub viewer_id: i64,
    pub request_id: String,
    pub next_sid: String,
    pub short_udid: i64
}

#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "player_data"]
pub struct BasePlayerData {
    pub id: i32,
    pub viewer_id: i64,
    pub now_name: String,
    pub user_comment: String,
    pub now_team_level: i32,
    pub favorite_unit_id: i32,
    pub free_jewel: i32,
    pub paid_jewel: i32,
    pub gold_id_free: i32,
    pub gold_id_pay: i32,
    pub emblem_id: i32
}

#[derive(Insertable)]
#[table_name = "player_data"]
pub struct NewPlayerData {
    pub gold_id_free: i32,
    pub viewer_id: i64
}

// player_unit_data (id) {
//     id -> Integer,
//     viewer_id -> Bigint,
//     unit_id -> Integer,
//     promotion_level -> Integer,
//     ub_level -> Integer,
//     ms_level_1 -> Integer,
//     ms_level_2 -> Integer,
//     ex_level -> Integer,
//     equip_slot_1 -> Bool,
//     equip_slot_2 -> Bool,
//     equip_slot_3 -> Bool,
//     equip_slot_4 -> Bool,
//     equip_slot_5 -> Bool,
//     equip_slot_6 -> Bool,
//     ue_level -> Integer,
//     ue_rank -> Integer,
// }
#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "player_unit_data"]
pub struct BasePlayerUnitData {
    pub id: i32,
    pub viewer_id: i64,
    pub unit_id: i32,
    pub rarity: i32,
    pub unit_level: i32,
    pub unit_exp: i32,
    pub promotion_level: i32,
    pub ub_level: i32,
    pub ms_level_1: i32,
    pub ms_level_2: i32,
    pub ex_level: i32,

    pub e_lv_1: i32,
    pub e_pt_1: i32,
    pub e_lv_2: i32,
    pub e_pt_2: i32,
    pub e_lv_3: i32,
    pub e_pt_3: i32,
    pub e_lv_4: i32,
    pub e_pt_4: i32,
    pub e_lv_5: i32,
    pub e_pt_5: i32,
    pub e_lv_6: i32,
    pub e_pt_6: i32,

    pub ue_level: i32,
    pub ue_rank: i32,
    pub ue_pt: i32,

    pub icon_skin_id: i32,
    pub sd_skin_id: i32,
    pub still_skin_id: i32,
    pub motion_id: i32,

    pub favorite_flag: i32
}

#[derive(Insertable)]
#[table_name = "player_unit_data"]
pub struct NewPlayerUnitData {
    pub viewer_id: i64,
    pub unit_id: i32,
    pub rarity: i32,
    pub ub_level: i32
}

#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "player_deck_list"]
pub struct BasePlayerDeckData {
    pub id: i32,
    pub viewer_id: i64,
    pub deck_number: i32,

    pub unit_id_1: i32,
    pub unit_id_2: i32,
    pub unit_id_3: i32,
    pub unit_id_4: i32,
    pub unit_id_5: i32
}

#[derive(Insertable)]
#[table_name = "player_deck_list"]
pub struct NewPlayerDeckData {
    pub viewer_id: i64,
    pub deck_number: i32,
}

#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "player_story"]
pub struct BasePlayerStory {
    pub id: i32,
    pub viewer_id: i64,
    
    pub story_id: i32,
    pub unlocked: bool,
    pub seen: bool
}

#[derive(Insertable)]
#[table_name = "player_story"]
pub struct NewPlayerStory {
    pub viewer_id: i64,
    pub story_id: i32,
    pub seen: bool
}

#[derive(Queryable, Debug, AsChangeset)]
#[table_name = "player_present"]
pub struct BasePlayerPresent {
    pub id: i32,
    pub viewer_id: i64,

    pub receive_status: bool,

    pub reward_type: i32,
    pub reward_id: i32,
    pub reward_count: i32,
    pub reward_rarity: i32,

    pub message_id: i32,
    pub create_time: i64
}

#[derive(Insertable)]
#[table_name = "player_present"]
pub struct NewPlayerPresent {
    pub viewer_id: i64,
    pub reward_type: i32,
    pub reward_id: i32,
    pub reward_count: i32,
    pub create_time: i64
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "player_item"]
pub struct BasePlayerItem {
    pub id: i32,
    pub viewer_id: i64,
    pub item_id: i32,
    pub stock: i32
}

#[derive(Insertable)]
#[table_name = "player_item"]
pub struct NewPlayerItem {
    pub viewer_id: i64,
    pub item_id: i32,
    pub stock: i32
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "player_equip"]
pub struct BasePlayerEquip {
    pub id: i32,
    pub viewer_id: i64,
    pub equip_id: i32,
    pub stock: i32
}

#[derive(Insertable)]
#[table_name = "player_equip"]
pub struct NewPlayerEquip {
    pub viewer_id: i64,
    pub equip_id: i32,
    pub stock: i32
}

#[derive(Queryable, AsChangeset, Debug)]
#[table_name = "player_arena_data"]
pub struct BasePlayerArenaData {
    pub id: i32,
    pub viewer_id: i64,
    pub arena_rank: i32,
    pub battle_num: i32
}

#[derive(Insertable)]
#[table_name = "player_arena_data"]
pub struct NewPlayerArenaData {
    pub viewer_id: i64,
    pub arena_rank: i32
}

#[derive(Queryable, Debug, AsChangeset, QueryableByName)]
#[table_name = "player_arena_log"]
pub struct BasePlayerArenaLog {
    pub id: i32,

    pub battle_token: String,
    pub battle_1_viewer_id: i64,
    pub battle_2_viewer_id: i64,
    pub user_arena_deck: String,
    pub vs_user_arena_deck: String,

    pub versus: bool,

    pub unit_id_1: i32,
    pub damage_1: i32,
    pub unit_id_2: i32,
    pub damage_2: i32,
    pub unit_id_3: i32,
    pub damage_3: i32,
    pub unit_id_4: i32,
    pub damage_4: i32,
    pub unit_id_5: i32,
    pub damage_5: i32,

    pub unit_id_1_e: i32,
    pub damage_1_e: i32,
    pub unit_id_2_e: i32,
    pub damage_2_e: i32,
    pub unit_id_3_e: i32,
    pub damage_3_e: i32,
    pub unit_id_4_e: i32,
    pub damage_4_e: i32,
    pub unit_id_5_e: i32,
    pub damage_5_e: i32,

    pub seed: i64,
    pub sts: i32,
    pub log_time: i64
}

#[derive(Insertable)]
#[table_name = "player_arena_log"]
pub struct NewPlayerArenaLog {
    pub battle_token: String,
    pub battle_1_viewer_id: i64,
    pub battle_2_viewer_id: i64,
    pub user_arena_deck: String,
    pub vs_user_arena_deck: String,

    pub seed: i64
}

#[derive(Insertable)]
#[table_name = "clan_battle_log"]
pub struct NewClanBattleLog {
    pub viewer_id: i64,
    pub order_num: i32,
    pub lap_num: i32,
    pub clan_battle_id: i32,

    pub enemy_id: i32,
    pub boss_damage: i32,
    pub total_damage: i32,

    pub u_1 : i32,
    pub u_1_rarity: i32,
    pub u_1_promotion: i32,
    pub u_1_damage: i32,
    pub u_2 : i32,
    pub u_2_rarity: i32,
    pub u_2_promotion: i32,
    pub u_2_damage: i32,
    pub u_3 : i32,
    pub u_3_rarity: i32,
    pub u_3_promotion: i32,
    pub u_3_damage: i32,
    pub u_4 : i32,
    pub u_4_rarity: i32,
    pub u_4_promotion: i32,
    pub u_4_damage: i32,
    pub u_5 : i32,
    pub u_5_rarity: i32,
    pub u_5_promotion: i32,
    pub u_5_damage: i32,

    pub log_time: i64,
    pub battle_log: String
}

#[derive(Queryable, Debug, AsChangeset, QueryableByName)]
#[table_name = "clan_battle_log"]
pub struct BaseClanBattleLog {
    pub id: i32,

    pub viewer_id: i64,
    pub order_num: i32,
    pub lap_num: i32,
    pub clan_battle_id: i32,

    pub enemy_id: i32,
    pub boss_damage: i32,
    pub total_damage: i32,

    pub u_1 : i32,
    pub u_1_rarity: i32,
    pub u_1_promotion: i32,
    pub u_1_damage: i32,
    pub u_2 : i32,
    pub u_2_rarity: i32,
    pub u_2_promotion: i32,
    pub u_2_damage: i32,
    pub u_3 : i32,
    pub u_3_rarity: i32,
    pub u_3_promotion: i32,
    pub u_3_damage: i32,
    pub u_4 : i32,
    pub u_4_rarity: i32,
    pub u_4_promotion: i32,
    pub u_4_damage: i32,
    pub u_5 : i32,
    pub u_5_rarity: i32,
    pub u_5_promotion: i32,
    pub u_5_damage: i32,

    pub log_time: i64,
    pub battle_log: String
}