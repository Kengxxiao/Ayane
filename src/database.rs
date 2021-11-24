
use std::collections::HashMap;

use diesel::{MysqlConnection, result::Error::NotFound};
use diesel::{prelude::*, sql_query};
use diesel::r2d2::{ConnectionManager, Pool};
use global_data::{CharaStoryStatus, SkillAction, StoryDetail, UnitData, UnitPromotionStatus, UnitRarity};
use md5::{Digest, Md5};
use once_cell::sync::OnceCell;
use rand::{Rng, thread_rng};
use serde_json::json;
use uuid::Uuid;
use crate::clan_battle;

use crate::model::BaseClanBattleLog;
use crate::priconne_model::{BattleLog2Request, ClanBattleRehearsalFinishRequest, GameUnitListForArenaSearch};
use crate::{global_data::{self, UniqueEquipmentEnhanceData}, model::{BasePlayerArenaData, NewClanBattleLog, BasePlayerArenaLog, BasePlayerData, BasePlayerDeckData, BasePlayerEquip, BasePlayerItem, BasePlayerPresent, BasePlayerStory, BasePlayerUnitData, BaseUserSession, NewPlayerArenaData, NewPlayerArenaLog, NewPlayerData, NewPlayerDeckData, NewPlayerEquip, NewPlayerItem, NewPlayerPresent, NewPlayerStory, NewPlayerUnitData, NewUserSession}, priconne_model::{ArenaFinishRequest, PriconneEquipRecipe, PriconneItem, PromotionEquipRecipe, SkillLevelUp}, schema};

type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;

static CONNECTION_POOL: OnceCell<MysqlPool> = OnceCell::new();

pub fn init_mysql_connection_pool() {
	// MySQL配置
    let database_url = "mysql://username:password@127.0.0.1/priconne".to_string();
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = Pool::builder().max_size(15).build(manager).unwrap();

    CONNECTION_POOL.set(pool);
}

pub fn update_request_id_and_sid(requestid: String) -> (String, String, i64, i64) {
    use schema::user_session::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let gen_request_id = Uuid::new_v4().to_hyphenated().to_string();
    let mut gen_sid = "".to_string();
    gen_sid.push_str((0..32).map(|_| b"0123456789abcdef"[thread_rng().gen_range(0..16)] as char).collect::<String>().as_str());
    let mut md5 = Md5::new();
    md5.update(&format!("{}{}", gen_sid, "c!SID!n"));
    let new_sid = format!("{:x}", md5.finalize_reset());
    match user_session.filter(request_id.eq(requestid.clone())).first::<BaseUserSession>(&connection) {
        Ok(mut user) => {
            user.request_id = gen_request_id.clone();
            user.next_sid = new_sid;
            diesel::update(user_session).filter(request_id.eq(requestid)).set(&user).execute(&connection).unwrap();
            (gen_request_id, gen_sid, user.short_udid, user.viewer_id)
        },
        Err(_) => (String::new(), String::new(), 0, 0)
    }
}

fn create_present(vid: i64, rtype: i32, rid: i32, rcount: i32) {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_present::dsl::*;
    diesel::insert_into(player_present).values(NewPlayerPresent {
        viewer_id: vid,
        reward_id: rid,
        reward_count: rcount,
        reward_type: rtype,
        create_time: chrono::Utc::now().timestamp()
    }).execute(&connection).unwrap();
}

pub fn priconne_receive_all_presents(vid: i64) -> Vec<serde_json::Value> {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_present::dsl::*;
    let presents = player_present.filter(viewer_id.eq(vid)).filter(receive_status.eq(false)).load::<BasePlayerPresent>(&connection).unwrap();
    let mut ret = vec![];
    for mut p in presents {
        p.receive_status = true;
        diesel::update(player_present).filter(viewer_id.eq(vid)).filter(id.eq(p.id)).set(&p).execute(&connection).unwrap();
        let add = priconne_add_item(vid, p.reward_id, p.reward_count);
        ret.push(json!({
            "id": add["id"].as_i64().unwrap(),
            "count": p.reward_count,
            "stock": add["stock"].as_i64().unwrap(),
            "received": p.reward_count,
            "type": p.reward_type
        }))
    }
    ret
}

pub fn priconne_receive_present(vid: i64, pid: i32) -> serde_json::Value {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_present::dsl::*;
    let mut present = player_present.filter(viewer_id.eq(vid)).filter(id.eq(pid)).first::<BasePlayerPresent>(&connection).unwrap();
    if !present.receive_status {
        present.receive_status = true;
        diesel::update(player_present).filter(viewer_id.eq(vid)).filter(id.eq(pid)).set(&present).execute(&connection).unwrap();
        let add = priconne_add_item(vid, present.reward_id, present.reward_count);
        return json!({
            "id": add["id"].as_i64().unwrap(),
            "count": present.reward_count,
            "stock": add["stock"].as_i64().unwrap(),
            "received": present.reward_count,
            "type": present.reward_type
        });
    }
    json!(null)
}

pub fn priconne_get_items(vid: i64) -> Vec<serde_json::Value> {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_item::dsl::*;
    let c = player_item.filter(viewer_id.eq(vid)).load::<BasePlayerItem>(&connection).unwrap();
    let mut items = vec![];
    for item in c {
        items.push(json!({
            "id": item.item_id,
            "count": 0,
            "stock": item.stock
        }))
    }
    items
}

pub fn priconne_add_item(vid: i64, iid: i32, count: i32) -> serde_json::Value {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_item::dsl::*;
    match player_item.filter(viewer_id.eq(vid)).filter(item_id.eq(iid)).first::<BasePlayerItem>(&connection) {
        Ok(mut item) => {
            item.stock += count;
            diesel::update(player_item).filter(viewer_id.eq(vid)).filter(item_id.eq(iid)).set(&item).execute(&connection).unwrap();
            json!({
                "id": item.item_id,
                "count": count,
                "stock": item.stock
            })
        },
        Err(_) => {
            diesel::insert_into(player_item).values(&NewPlayerItem {
                viewer_id: vid,
                item_id: iid,
                stock: count
            }).execute(&connection).unwrap();
            json!({
                "id": iid,
                "count": count,
                "stock": count
            })
        }
    }
}

pub fn priconne_get_equip(vid: i64) -> Vec<serde_json::Value> {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_equip::dsl::*;
    let mut items = vec![];
    for equip in global_data::EQUIPMENT_DATA.get().unwrap() {
        match player_equip.filter(viewer_id.eq(vid)).filter(equip_id.eq(equip.equipment_id as i32)).first::<BasePlayerEquip>(&connection) {
            Ok(p) => {
                items.push(json!({
                    "id": p.equip_id,
                    "count": 0,
                    "stock": p.stock
                }))
            },
            Err(_) => {
                diesel::insert_into(player_equip).values(&NewPlayerEquip {
                    viewer_id: vid,
                    equip_id: equip.equipment_id as i32,
                    stock: if equip.equipment_id == 140001 {999999} else {999}
                }).execute(&connection).unwrap();
                items.push(json!({
                    "id": equip.equipment_id,
                    "count": 0,
                    "stock": if equip.equipment_id == 140001 {999999} else {999}
                }))
            }
        }
    }
    items
}

pub fn priconne_add_equip(vid: i64, iid: i32, count: i32) -> serde_json::Value {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_equip::dsl::*;
    match player_equip.filter(viewer_id.eq(vid)).filter(equip_id.eq(iid)).first::<BasePlayerEquip>(&connection) {
        Ok(mut item) => {
            item.stock += count;
            diesel::update(player_equip).filter(viewer_id.eq(vid)).filter(equip_id.eq(iid)).set(&item).execute(&connection).unwrap();
            json!({
                "id": item.equip_id,
                "count": count,
                "stock": item.stock
            })
        },
        Err(_) => {
            diesel::insert_into(player_equip).values(&NewPlayerEquip {
                viewer_id: vid,
                equip_id: iid,
                stock: count
            }).execute(&connection).unwrap();
            json!({
                "id": iid,
                "count": count,
                "stock": count
            })
        }
    }
}

pub fn get_request_id_and_sid(requestid: &str) -> (String, String, i64, i64) {
    use schema::user_session::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    match user_session.filter(request_id.eq(requestid)).first::<BaseUserSession>(&connection) {
        Ok(user) => {
            (user.request_id, user.next_sid, user.viewer_id, user.short_udid)
        },
        Err(_) => (String::new(), String::new(), 0, 0)
    }
}

pub fn priconne_sdk_login(uid: String, gen_request_id: String) -> Result<BaseUserSession, diesel::result::Error> {
    use schema::user_session::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut gen_sid = "".to_string();
    gen_sid.push_str((0..32).map(|_| b"0123456789abcdef"[thread_rng().gen_range(0..16)] as char).collect::<String>().as_str());

    let int_uid = uid.parse::<i64>().unwrap_or_default();

    if int_uid == 0 {
        return Err(diesel::result::Error::NotFound);
    }

    match user_session.filter(viewer_id.eq(int_uid)).first::<BaseUserSession>(&connection) {
        Ok(mut user) => {
            user.request_id = gen_request_id;
            user.next_sid = gen_sid;
            diesel::update(user_session).filter(viewer_id.eq(int_uid)).set(&user).execute(&connection).unwrap();
            Ok(user)
        },
        Err(err) => match err {
            NotFound => {
                let u = NewUserSession {
                    viewer_id: int_uid,
                    request_id: gen_request_id,
                    next_sid: gen_sid,
                    short_udid: 10_0000_0000 + int_uid
                };
                create_present(int_uid, 2, 20004, 88888);
                create_present(int_uid, 2, 22003, 8888);
                create_present(int_uid, 2, 26101, 888);
                diesel::insert_into(user_session).values(&u).execute(&connection).unwrap();
                user_session.filter(viewer_id.eq(int_uid)).first::<BaseUserSession>(&connection)
            },
            errs => Err(errs)
        }
    }
}

pub fn priconne_get_present_count(vid: i64) -> i64 {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_present::dsl::*;
    let c = player_present.filter(viewer_id.eq(vid)).filter(receive_status.eq(false)).count().get_result::<i64>(&connection);
    c.unwrap_or_default()
}

pub fn priconne_get_present_list(vid: i64, need_status: bool) -> Vec<serde_json::Value> {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_present::dsl::*;
    let c = player_present.filter(viewer_id.eq(vid)).filter(receive_status.eq(need_status)).load::<BasePlayerPresent>(&connection).unwrap();
    let mut ret = vec![];
    for present in c {
        ret.push(json!({
            "present_id": present.id,
            "viewer_id": present.viewer_id,
            "receive_status": present.receive_status as i32,
            "reward_type": present.reward_type,
            "reward_id": present.reward_id,
            "reward_count": present.reward_count,
            "reward_rarity": present.reward_rarity,
            "message_id": present.message_id,
            "message_text": "",
            "message_param_value_1": 0,
            "message_param_value_2": 0,
            "message_param_value_3": 0,
            "message_param_value_4": 0,
            "reward_limit_flag": 0,
            "reward_limit_time": 0,
            "create_time": 0
        }))
    }
    ret
}

pub fn priconne_get_player_data(vid: i64) -> BasePlayerData {
    use schema::player_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    match player_data.filter(viewer_id.eq(vid)).first::<BasePlayerData>(&connection) {
        Ok(user) => user,
        Err(_) => {
            diesel::insert_into(player_data).values(NewPlayerData {
                gold_id_free: 999999999,
                viewer_id: vid
            }).execute(&connection).unwrap();
            player_data.filter(viewer_id.eq(vid)).first::<BasePlayerData>(&connection).unwrap()
        }
    }
}

fn update_player_unit(uid: Vec<(i32, i32)>, vid: i64) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let v = player_unit_data.filter(viewer_id.eq(vid)).load::<BasePlayerUnitData>(&connection).unwrap();
    for p in uid {
        if v.iter().find(|x| x.unit_id == p.0).is_none() {
            diesel::insert_into(player_unit_data).values(NewPlayerUnitData {
                viewer_id: vid,
                unit_id: p.0,
                rarity: p.1,
                ub_level: if p.0 == 105801 {2} else {1}
            }).execute(&connection).unwrap();
        }
    }
}

pub fn priconne_get_full_user_chara_info() -> Vec<serde_json::Value> {
    let unit_all: Vec<&UnitData> = global_data::UNIT_DATA.get().unwrap().iter().filter(|x| x.comment.len() != 0).collect();
    let mut user_chara_info = vec![];
    for unit in unit_all {
        user_chara_info.push(json!({
            "chara_id": (unit.unit_id - unit.unit_id % 100) / 100,
            "chara_love": 999999,
            "love_level": if unit.rarity >= 6 {12} else {8}
        }))
    }
    user_chara_info
}

pub fn priconne_get_player_story(vid: i64) -> (Vec<i32>, Vec<i32>) {
    let story_all: Vec<&StoryDetail> = global_data::STORY_DATA.get().unwrap().iter().collect();
    let mut read_story = vec![];
    let mut unlocked_story = vec![];
    use schema::player_story::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let v = player_story.filter(viewer_id.eq(vid)).load::<BasePlayerStory>(&connection).unwrap();
    for s in story_all {
        match v.iter().find(|x| x.story_id == s.story_id as i32) {
            Some(s) => {
                if s.unlocked {
                    unlocked_story.push(s.story_id);
                }
                if s.seen {
                    read_story.push(s.story_id);
                }
            },
            None => {
                diesel::insert_into(player_story).values(NewPlayerStory {
                    viewer_id: vid,
                    story_id: s.story_id as i32,
                    seen: true
                }).execute(&connection).unwrap();
                unlocked_story.push(s.story_id as i32);
            }
        }
    }
    (read_story, unlocked_story)
}

pub fn priconne_get_full_deck_list(vid: i64) -> Vec<serde_json::Value> {
    let need_deck_list: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 10101, 10102, 10103, 2001, 1001, 1002, 1003, 1004];
    use schema::player_deck_list::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let v = player_deck_list.filter(viewer_id.eq(vid)).load::<BasePlayerDeckData>(&connection).unwrap();
    let mut deck_list = vec![];
    for dnumber in need_deck_list {
        match v.iter().find(|x| x.deck_number == dnumber) {
            None => {
                diesel::insert_into(player_deck_list).values(NewPlayerDeckData {
                    viewer_id: vid,
                    deck_number: dnumber
                }).execute(&connection).unwrap();
                deck_list.push(json!({
                    "deck_number": dnumber,
                    "unit_id_1": 100101,
                    "unit_id_2": 0,
                    "unit_id_3": 0,
                    "unit_id_4": 0,
                    "unit_id_5": 0
                }));
            },
            Some(p) => {
                deck_list.push(json!({
                    "deck_number": dnumber,
                    "unit_id_1": p.unit_id_1,
                    "unit_id_2": p.unit_id_2,
                    "unit_id_3": p.unit_id_3,
                    "unit_id_4": p.unit_id_4,
                    "unit_id_5": p.unit_id_5
                }))
            }
        }
    }
    deck_list
}

pub fn create_unit_data(u: &BasePlayerUnitData) -> serde_json::Value {
    let u_skill_tmp = global_data::UNIT_SKILL_DATA.get().unwrap().iter().find(|x| x.unit_id as i32 == u.unit_id).unwrap();
    let u_promotion_tmp = global_data::UNIT_PROMOTION.get().unwrap().iter().find(|x| x.unit_id as i32 == u.unit_id && x.promotion_level as i32 == u.promotion_level).unwrap();

    let mut main_skill = vec![];
    if u.promotion_level >= 2 {
        main_skill.push(json!({
            "skill_id": if u.ue_level > 0 { u_skill_tmp.main_skill_evolution_1 } else { u_skill_tmp.main_skill_1 },
            "skill_level": u.ms_level_1
        }));
        if u.promotion_level >= 4 {
            main_skill.push(json!({
                "skill_id": u_skill_tmp.main_skill_2,
                "skill_level": u.ms_level_2
            }))
        }
    }

    let mut ex_skill = vec![];
    if u.promotion_level >= 7 {
        ex_skill.push(match u.rarity {
            5 => json!({
                "skill_id": u_skill_tmp.ex_skill_evolution_1,
                "skill_level": u.ex_level
            }),
            _ => json!({
                "skill_id": u_skill_tmp.ex_skill_1,
                "skill_level": u.ex_level
            })
        })
    }

    let unique = match global_data::UNIT_UNIQUE_EQUIP.get().unwrap().iter().find(|x| x.unit_id as i32 == u.unit_id) {
        Some(e) => {
            json!([
                {
                    "id": e.equip_id,
                    "is_slot": if u.ue_level > 0 {e.equip_slot} else {0},
                    "enhancement_level": u.ue_level,
                    "enhancement_pt": u.ue_pt,
                    "rank": u.ue_rank
                }
            ])
        },
        None => {json!([])}
    };
    

    json!({
        "id": u.unit_id,
        "unit_rarity": u.rarity,
        "unit_level": u.unit_level,
        "promotion_level": u.promotion_level,
        "unit_exp": u.unit_exp,
        "get_time": 0,
        "union_burst": [
            {
                "skill_id": if u.rarity >= 6 {u_skill_tmp.union_burst_evolution} else {u_skill_tmp.union_burst},
                "skill_level": u.ub_level
            }
        ],
        "main_skill": main_skill,
        "ex_skill": ex_skill,
        "free_skill": [],
        "equip_slot": [
            {
                "id": u_promotion_tmp.equip_slot_1,
                "is_slot": (u.e_lv_1 > -1) as i32,
                "enhancement_level": if u.e_lv_1 == -1 {0} else {u.e_lv_1},
                "enhancement_pt": u.e_pt_1
            },
            {
                "id": u_promotion_tmp.equip_slot_2,
                "is_slot": (u.e_lv_2 > -1) as i32,
                "enhancement_level": if u.e_lv_2 == -1 {0} else {u.e_lv_2},
                "enhancement_pt": u.e_pt_2
            },
            {
                "id": u_promotion_tmp.equip_slot_3,
                "is_slot": (u.e_lv_3 > -1) as i32,
                "enhancement_level": if u.e_lv_3 == -1 {0} else {u.e_lv_3},
                "enhancement_pt": u.e_pt_3
            },
            {
                "id": u_promotion_tmp.equip_slot_4,
                "is_slot": (u.e_lv_4 > -1) as i32,
                "enhancement_level": if u.e_lv_4 == -1 {0} else {u.e_lv_4},
                "enhancement_pt": u.e_pt_4
            },
            {
                "id": u_promotion_tmp.equip_slot_5,
                "is_slot": (u.e_lv_5 > -1) as i32,
                "enhancement_level": if u.e_lv_5 == -1 {0} else {u.e_lv_5},
                "enhancement_pt": u.e_pt_5
            },
            {
                "id": u_promotion_tmp.equip_slot_6,
                "is_slot": (u.e_lv_6 > -1) as i32,
                "enhancement_level": if u.e_lv_6 == -1 {0} else {u.e_lv_6},
                "enhancement_pt": u.e_pt_6
            }
        ],
        "unique_equip_slot": unique,
        "power": 0,
        "skin_data": {
            "icon_skin_id": u.icon_skin_id,
            "sd_skin_id": u.sd_skin_id,
            "still_skin_id": u.still_skin_id,
            "motion_id": u.motion_id
        },
        "favorite_flag": u.favorite_flag,
        "unlock_rarity_6_item": {
            "slot_1": 0,
            "slot_2": 0,
            "slot_3": 0
        }
    })
}

pub fn priconne_get_full_unit_data(vid: i64) -> Vec<serde_json::Value> {
    let unit_ids = global_data::UNIT_DATA.get().unwrap().iter().filter(|x| x.comment.len() != 0).map(|x| (x.unit_id as i32, x.rarity as i32)).collect();
    update_player_unit(unit_ids, vid);

    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let v = player_unit_data.filter(viewer_id.eq(vid)).load::<BasePlayerUnitData>(&connection).unwrap();
    let mut ret = vec![];
    for u in v {
        // let u_data_tmp = global_data::UNIT_DATA.get().unwrap().iter().find(|x| x.unit_id as i32 == u.unit_id).unwrap();
        ret.push(create_unit_data(&u));
    }
    ret
}

pub fn priconne_local_get_all_quests() -> Vec<serde_json::Value> {
    let mut ret = vec![];
    let c = global_data::QUEST_DATA.get().unwrap();
    for quest in c {
        ret.push(json!({
            "quest_id": quest.quest_id,
            "clear_flg": 3,
            "result_type": 3,
            "daily_clear_count": 0,
            "daily_recovery_count": 0
        }))
    }
    ret
}

pub fn priconne_read_story(vid: i64, sid: i32) {
    use schema::player_story::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    match player_story.filter(viewer_id.eq(vid)).filter(story_id.eq(sid)).first::<BasePlayerStory>(&connection) {
        Ok(mut s) => {
            s.seen = true;
            diesel::update(player_story).filter(viewer_id.eq(vid)).filter(story_id.eq(sid)).set(&s).execute(&connection).unwrap();
        },
        Err(_) => {}
    }
}

pub fn priconne_local_get_shop_list() -> Vec<serde_json::Value> {
    let unit_all: Vec<i64> = global_data::ITEM_DATA.get().unwrap().iter().filter(|x| x.item_type == 11).map(|x| x.item_id).collect();
    let mut item_list = vec![];
    for unit in unit_all {
        item_list.push(json!({
            "exchange_count": 1000,
            "available_num": 1000,
            "price_group": 1,
            "slot_id": unit - 21000,
            "type": 2,
            "item_id": unit,
            "num": 1000,
            "sold": 0,
            "end_time": -1,
            "price": {
                "currency_type": 2,
                "currency_id": 90005,
                "currency_num": 0
            }
        }))
    }
    let mut shop = vec![];
    for system_id in 20001..=20006 {
        shop.push(json!({
            "system_id": system_id,
            "reset_count": 0,
            "next_renewal_time": 0,
            "reset_cost": 0,
            "reset_cost_id": 0,
            "close_time": 0,
            "item_list": [],
            "remaining_appear_count": 0,
            "max_appear_num": 0
        }))
    }
    shop.push(json!({
        "system_id": 207,
        "reset_count": 0,
        "next_renewal_time": 0,
        "reset_cost": 0,
        "item_list": item_list,
        "close_time": 0,
        "remaining_appear_count": 1,
        "max_appear_num": 1
    }));
    shop
}

fn create_unit_param(unit_data: &BasePlayerUnitData) -> serde_json::Value {
    let mut ret = json!({});
    let mut bonus = HashMap::new();
    let safe : Vec<&CharaStoryStatus> = global_data::CHARA_STORY_STATUS.get().unwrap().iter().filter(|x| x.chara_id.contains(&(unit_data.unit_id / 100))).collect();
    for status in safe {
        for add in status.status_add.iter() {
            let b = bonus.entry(&add.0).or_insert(0);
            *b += add.1;
        }
    }
    ret["bonus_param"] = json!(bonus);
    let safe_unit_promotion : Option<&UnitPromotionStatus> = global_data::UNIT_PROMOTION_STATUS.get().unwrap().iter().find(|x| x.promotion_level as i32 == unit_data.promotion_level && x.unit_id as i32 == unit_data.unit_id);
    let safe_unit_rarity : &UnitRarity = global_data::UNIT_RARITY.get().unwrap().iter().find(|x| x.unit_id as i32 == unit_data.unit_id && x.rarity as i32 == unit_data.rarity).unwrap();
    let mut base = HashMap::new();
    if let Some(p) = safe_unit_promotion {
        for promotion in &p.base {
            base.entry(promotion.0.clone()).or_insert(promotion.1);
        }
    } else {
        let sts = global_data::UNIT_STATUS_TABLE.get().unwrap();
        for s in 1..=17 {
            base.entry(sts[s.to_string()].as_str().unwrap().to_string()).or_insert(0f64);
        }
    }
    for growth in &safe_unit_rarity.growth {
        let entry = base.entry(growth.0.clone()).or_insert(0f64);
        *entry += growth.1 + growth.2 * (unit_data.promotion_level + unit_data.unit_level) as f64;
    }
    let mut real_base = HashMap::new();
    for (k, v) in base {
        real_base.entry(k).or_insert( v.round() as i64);
    }
    ret["base_param"] = json!(real_base);
    let safe_unit_promotion = global_data::UNIT_PROMOTION.get().unwrap().iter().find(|x| x.unit_id as i32 == unit_data.unit_id && x.promotion_level as i32 == unit_data.promotion_level).unwrap();
    let mut equip_growth = HashMap::new();
    {
        if unit_data.e_lv_1 != -1 && safe_unit_promotion.equip_slot_1 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_1).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_1).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_1 as f64;
            }
        }
        if unit_data.e_lv_2 != -1 && safe_unit_promotion.equip_slot_2 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_2).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_2).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_2 as f64;
            }
        }
        if unit_data.e_lv_3 != -1 && safe_unit_promotion.equip_slot_3 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_3).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_3).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_3 as f64;
            }
        }
        if unit_data.e_lv_4 != -1 && safe_unit_promotion.equip_slot_4 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_4).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_4).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_4 as f64;
            }
        }
        if unit_data.e_lv_5 != -1 && safe_unit_promotion.equip_slot_5 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_5).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_5).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_5 as f64;
            }
        }
        if unit_data.e_lv_6 != -1 && safe_unit_promotion.equip_slot_6 != 999999 {
            let safe_equip_1 = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_6).unwrap();
            for e in &safe_equip_1.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            let safe_equip_1_rate = global_data::EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == safe_unit_promotion.equip_slot_6).unwrap();
            for e in &safe_equip_1_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * unit_data.e_lv_6 as f64;
            }
        }
        if unit_data.ue_level != 0 {
            let unique_equip = global_data::UNIT_UNIQUE_EQUIP.get().unwrap().iter().find(|x| x.unit_id as i32 == unit_data.unit_id).unwrap();
            let unique = global_data::UNIQUE_EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == unique_equip.equip_id).unwrap();
            let unique_enhance_rate = global_data::UNIQUE_EQUIPMENT_ENHANCE_RATE.get().unwrap().iter().find(|x| x.equipment_id == unique_equip.equip_id).unwrap();
            for e in &unique.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1;
            }
            for e in &unique_enhance_rate.base {
                let es = equip_growth.entry(&e.0).or_insert(0f64);
                *es += e.1 * (unit_data.ue_level - 1) as f64;
            }
        }
    }
    let mut real_equip_base = HashMap::new();
    for (k, v) in equip_growth {
        real_equip_base.entry(k).or_insert(v.round() as i64);
    }
    ret["equip_param"] = json!(real_equip_base);
    let mut skill_map = HashMap::new();
    let u2 = global_data::UNIT_SKILL_DATA.get().unwrap().iter().find(|x| x.unit_id as i32 == unit_data.unit_id).unwrap();
    let passive_skill_id = if unit_data.rarity >= 5 { u2.ex_skill_evolution_1 } else { u2.ex_skill_1 };
    let passive_skill_data = global_data::SKILL_DATA.get().unwrap().iter().find(|x| x.skill_id == passive_skill_id).unwrap();
    let passive_skill_actions : Vec<&SkillAction> = global_data::SKILL_ACTION.get().unwrap().iter().filter(|x| passive_skill_data.actions.contains(&x.action_id) && x.action_type == 90).collect();
    for p in passive_skill_actions {
        let es = skill_map.entry(global_data::UNIT_STATUS_TABLE.get().unwrap()[p.action_detail_1.to_string()].as_str().unwrap()).or_insert(0f64);
        *es += p.action_value_2 + p.action_value_3 * unit_data.ex_level as f64;
    }
    let mut real_skill_map = HashMap::new();
    for (k, v) in skill_map {
        real_skill_map.entry(k).or_insert(v.round() as i64);
    }
    ret["passive_skill_param"] = json!(real_skill_map);
    ret["power"] = json!(0);
    ret
}

pub fn get_deck_list(vid: i64, dn: i32) -> BasePlayerDeckData {
    use schema::player_deck_list::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let deck = player_deck_list.filter(viewer_id.eq(vid)).filter(deck_number.eq(dn)).first::<BasePlayerDeckData>(&connection).unwrap();
    deck
}

pub fn create_opponent_arena_data(vid: i64, dn: i32, use_unit_param: bool) -> Vec<serde_json::Value> {
    use schema::player_deck_list::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let deck = player_deck_list.filter(viewer_id.eq(vid)).filter(deck_number.eq(dn)).first::<BasePlayerDeckData>(&connection).unwrap();
    let deck_list = vec![deck.unit_id_1, deck.unit_id_2, deck.unit_id_3, deck.unit_id_4, deck.unit_id_5];
    let mut deck_data = vec![];
    for deck_id in deck_list {
        if deck_id == 0 {
            continue;
        }
        use schema::player_unit_data::dsl::*;
        let v = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(deck_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        let mut deck_main_data = create_unit_data(&v);
        let unit_param = create_unit_param(&v);
        if use_unit_param {
            deck_main_data["unit_param"] = json!(unit_param);
        }
        deck_main_data["bonus_param"] = json!(unit_param["bonus_param"]);

        deck_data.push(deck_main_data);
    }
    deck_data
}

pub fn priconne_get_arena_search_opponent(vid: i64) -> Vec<serde_json::Value> {

    let self_mode: bool = true;

    use schema::player_arena_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let user = player_arena_data.filter(viewer_id.eq(vid)).first::<BasePlayerArenaData>(&connection).unwrap();
    no_arg_sql_function!(RAND, (), "Represents the sql RANDOM() function");
    let mut safe_opps = if !self_mode {
        player_arena_data.filter(arena_rank.lt(user.arena_rank)).filter(arena_rank.gt(user.arena_rank - 10)).order(RAND).limit(3).load::<BasePlayerArenaData>(&connection).unwrap()
    } else {
        vec![player_arena_data.filter(viewer_id.eq(vid)).first::<BasePlayerArenaData>(&connection).unwrap()]
    };
    let opps = if self_mode {
        safe_opps.extend(player_arena_data.filter(arena_rank.lt(user.arena_rank)).filter(arena_rank.gt(user.arena_rank - 10)).order(RAND).limit(2).load::<BasePlayerArenaData>(&connection).unwrap());
        safe_opps
    } else {safe_opps};
    let mut ret = vec![];
    for opp in opps {
        let opp_vid = opp.viewer_id;
        use schema::player_data::dsl::*;
        let opp_data = player_data.filter(schema::player_data::dsl::viewer_id.eq(opp_vid)).first::<BasePlayerData>(&connection).unwrap();
        use schema::player_unit_data::dsl::*;
        let opp_fav_data = player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(opp_vid)).filter(unit_id.eq(opp_data.favorite_unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        let opp_deck = create_opponent_arena_data(opp.viewer_id, 3, false);
        ret.push(json!({
            "viewer_id": opp_vid,
            "rank": opp.arena_rank,
            "user_name": opp_data.now_name,
            "team_level": opp_data.now_team_level,
            "favorite_unit": {
                "id": opp_data.favorite_unit_id,
                "unit_rarity": opp_fav_data.rarity,
                "unit_level": opp_fav_data.unit_level,
                "promotion_level": opp_fav_data.promotion_level,
                "skin_data": {
                    "icon_skin_id": opp_fav_data.icon_skin_id,
                    "sd_skin_id": opp_fav_data.sd_skin_id,
                    "still_skin_id": opp_fav_data.still_skin_id,
                    "motion_id": opp_fav_data.motion_id,
                }
            },
            "emblem": {
                "emblem_id": opp_data.emblem_id,
                "ex_value": 0
            },
            "arena_deck": opp_deck
        }));
    }
    ret
}

pub fn priconne_get_arena_info(vid: i64) -> serde_json::Value {
    use schema::player_arena_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let count = player_arena_data.count().get_result::<i64>(&connection).unwrap();
    let user = match player_arena_data.filter(viewer_id.eq(vid)).first::<BasePlayerArenaData>(&connection) {
        Ok(u) => u,
        Err(_) => {
            diesel::insert_into(player_arena_data).values(NewPlayerArenaData {
                viewer_id: vid,
                arena_rank: (count + 1) as i32
            }).execute(&connection).unwrap();
            player_arena_data.filter(viewer_id.eq(vid)).first::<BasePlayerArenaData>(&connection).unwrap()
        }
    };
    json!({
        "max_battle_number": 5,
        "battle_number": user.battle_num,
        "interval_end_time": 0,
        "highest_rank": 1,
        "season_highest_rank": 1,
        "yesterday_defend_number": 1,
        "group": 1,
        "group_moving_release_time": 0,
        "rank": user.arena_rank
    })
}

pub fn priconne_update_deck(vid: i64, dn: i32, u1: i32, u2: i32, u3: i32, u4: i32, u5: i32) {
    use schema::player_deck_list::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut dck = player_deck_list.filter(viewer_id.eq(vid)).filter(deck_number.eq(dn)).first::<BasePlayerDeckData>(&connection).unwrap();
    dck.unit_id_1 = u1;
    dck.unit_id_2 = u2;
    dck.unit_id_3 = u3;
    dck.unit_id_4 = u4;
    dck.unit_id_5 = u5;
    diesel::update(player_deck_list).filter(viewer_id.eq(vid)).filter(deck_number.eq(dn)).set(&dck).execute(&connection).unwrap();
}

pub fn priconne_create_arena_log(vid: i64, mut bvid: i64, uad: &Vec<serde_json::Value>, vuad: &Vec<serde_json::Value>, sd: i64, btoken: String) -> i32 {
    use schema::player_arena_log::dsl::*;
    if bvid == vid {
        bvid = 0;
    }
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    diesel::insert_into(player_arena_log).values(&NewPlayerArenaLog {
        battle_token: btoken.clone(),
        user_arena_deck: serde_json::to_string(uad).unwrap(),
        vs_user_arena_deck: serde_json::to_string(vuad).unwrap(),
        battle_1_viewer_id: vid,
        battle_2_viewer_id: bvid,
        seed: sd
    }).execute(&connection).unwrap();
    let log = player_arena_log.filter(battle_token.eq(btoken)).first::<BasePlayerArenaLog>(&connection).unwrap();
    log.id
}

pub fn priconne_set_log(req: &ArenaFinishRequest) -> (i32, i32) {
    use schema::player_arena_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut log = player_arena_log.filter(schema::player_arena_log::dsl::id.eq(req.battle_id)).first::<BasePlayerArenaLog>(&connection).unwrap();

    let parsed_uad : Vec<serde_json::Value> = serde_json::from_str(&log.user_arena_deck).unwrap();
    //let parsed_vuad : Vec<serde_json::Value> = serde_json::from_str(&log.vs_user_arena_deck).unwrap();

    let user_deck_2_damage = &req.arena_wave_result_list[0].unit_damage_list[0..parsed_uad.len()];
    let vs_user_deck_2_damage = &req.arena_wave_result_list[0].unit_damage_list[user_deck_2_damage.len()..];

    for i in 0..user_deck_2_damage.len() {
        match i {
            0 => {
                log.unit_id_1 = user_deck_2_damage[i].unit_id;
                log.damage_1 = user_deck_2_damage[i].damage;
            },
            1 => {
                log.unit_id_2 = user_deck_2_damage[i].unit_id;
                log.damage_2 = user_deck_2_damage[i].damage;
            },
            2 => {
                log.unit_id_3 = user_deck_2_damage[i].unit_id;
                log.damage_3 = user_deck_2_damage[i].damage;
            },
            03 => {
                log.unit_id_4 = user_deck_2_damage[i].unit_id;
                log.damage_4 = user_deck_2_damage[i].damage;
            },
            4 => {
                log.unit_id_5 = user_deck_2_damage[i].unit_id;
                log.damage_5 = user_deck_2_damage[i].damage;
            },
            _ => {}
        }
    }

    for i in 0..vs_user_deck_2_damage.len() {
        match i {
            0 => {
                log.unit_id_1_e = vs_user_deck_2_damage[i].unit_id;
                log.damage_1_e = vs_user_deck_2_damage[i].damage;
            },
            1 => {
                log.unit_id_2_e = vs_user_deck_2_damage[i].unit_id;
                log.damage_2_e = vs_user_deck_2_damage[i].damage;
            },
            2 => {
                log.unit_id_3_e = vs_user_deck_2_damage[i].unit_id;
                log.damage_3_e = vs_user_deck_2_damage[i].damage;
            },
            03 => {
                log.unit_id_4_e = vs_user_deck_2_damage[i].unit_id;
                log.damage_4_e = vs_user_deck_2_damage[i].damage;
            },
            4 => {
                log.unit_id_5_e = vs_user_deck_2_damage[i].unit_id;
                log.damage_5_e = vs_user_deck_2_damage[i].damage;
            },
            _ => {}
        }
    }

    log.log_time = chrono::Utc::now().timestamp();

    let mut win = false;
    for i in 0..user_deck_2_damage.len() {
        let self_hp = req.arena_wave_result_list[0].unit_hp_list.get(i).unwrap();
        if self_hp.hp != 0 {
            win = true;
            break;
        }
    }
    log.versus = win;
    log.sts = 1;
    diesel::update(player_arena_log).filter(schema::player_arena_log::dsl::id.eq(req.battle_id)).set(&log).execute(&connection).unwrap();

    let mut self_player = schema::player_arena_data::dsl::player_arena_data.filter(schema::player_arena_data::dsl::viewer_id.eq(log.battle_1_viewer_id)).first::<BasePlayerArenaData>(&connection).unwrap();
    if log.battle_2_viewer_id != 0 {
        let mut enemy_player = schema::player_arena_data::dsl::player_arena_data.filter(schema::player_arena_data::dsl::viewer_id.eq(log.battle_2_viewer_id)).first::<BasePlayerArenaData>(&connection).unwrap();
        if win {
            let tmp_a = self_player.arena_rank;
            self_player.arena_rank = enemy_player.arena_rank;
            enemy_player.arena_rank = tmp_a;
            diesel::update(schema::player_arena_data::dsl::player_arena_data).filter(schema::player_arena_data::dsl::viewer_id.eq(log.battle_1_viewer_id)).set(&self_player).execute(&connection).unwrap();
            diesel::update(schema::player_arena_data::dsl::player_arena_data).filter(schema::player_arena_data::dsl::viewer_id.eq(log.battle_2_viewer_id)).set(&enemy_player).execute(&connection).unwrap();
        }
        return (self_player.arena_rank, enemy_player.arena_rank)
    }
    (self_player.arena_rank, self_player.arena_rank)
}

pub fn priconne_get_log_list(vid: i64) -> Vec<serde_json::Value> {
    use schema::player_arena_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let logs = player_arena_log.filter(battle_1_viewer_id.eq(vid).or(battle_2_viewer_id.eq(vid))).order(log_time.desc()).limit(10).load::<BasePlayerArenaLog>(&connection).unwrap();
    let mut ret = vec![];
    for log in logs {
        let mut target_vid = log.battle_2_viewer_id;
        let is_challenge = (log.battle_1_viewer_id == vid) as i32;

        let mut win_or_lose = 0;
        if log.battle_1_viewer_id == vid && log.versus {
            win_or_lose = 1;
        }
        if log.battle_2_viewer_id == vid && !log.versus {
            win_or_lose = 1;
        }
        if target_vid == 0 || is_challenge == 0 {
            target_vid = log.battle_1_viewer_id;
        }
        //println!("{} {:#?}", target_vid, log);
        use schema::player_data::dsl::player_data;
        let ou = player_data.filter(schema::player_data::dsl::viewer_id.eq(target_vid)).first::<BasePlayerData>(&connection).unwrap();
        use schema::player_unit_data::dsl::player_unit_data;
        let fav_data = player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(target_vid)).filter(schema::player_unit_data::dsl::unit_id.eq(ou.favorite_unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        ret.push(json!({
            "log_id": log.id,
            "win_or_lose": win_or_lose,
            "is_challenge": is_challenge,
            "versus_time": log.log_time,
            "opponent_user": {
                "viewer_id": ou.viewer_id,
                "user_name": ou.now_name,
                "team_level": ou.now_team_level,
                "favorite_unit": {
                    "id": fav_data.unit_id,
                    "unit_rarity": fav_data.rarity,
                    "unit_level": fav_data.unit_level,
                    "promotion_level": fav_data.promotion_level,
                    "skin_data": {
                        "icon_skin_id": fav_data.icon_skin_id,
                        "sd_skin_id": fav_data.sd_skin_id,
                        "still_skin_id": fav_data.still_skin_id,
                        "motion_id": fav_data.motion_id
                    }
                },
                "total_power": 0,
                "emblem": {
                    "emblem_id": ou.emblem_id,
                    "ex_value": 0
                }
            }
        }));
    }
    ret
}

pub fn priconne_debug_get_unit_list_from_successor(param: GameUnitListForArenaSearch) -> serde_json::Value {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();

    let sql = format!("SELECT * FROM player_arena_log where
    (versus = 0 and unit_id_1 = {} and unit_id_2 = {} and unit_id_3 = {} and unit_id_4 = {} and unit_id_5 = {}) or
    (versus = 1 and unit_id_1_e = {} and unit_id_2_e = {} and unit_id_3_e = {} and unit_id_4_e = {} and unit_id_5_e = {}) order by log_time desc",
    param.unit_id_1, param.unit_id_2, param.unit_id_3, param.unit_id_4, param.unit_id_5, param.unit_id_1, param.unit_id_2, param.unit_id_3, param.unit_id_4, param.unit_id_5);
    let logs = sql_query(sql).get_results::<BasePlayerArenaLog>(&connection).unwrap();

    let mut resp = vec![];
    for log in logs {
        resp.push(if !log.versus {
            json!({
                "unit_id": [
                    log.unit_id_1_e,
                    log.unit_id_2_e,
                    log.unit_id_3_e,
                    log.unit_id_4_e,
                    log.unit_id_5_e
                ],
                "time": log.log_time
            })
        } else {
            json!({
                "unit_id": [
                    log.unit_id_1,
                    log.unit_id_2,
                    log.unit_id_3,
                    log.unit_id_4,
                    log.unit_id_5
                ],
                "time": log.log_time
            })
        })
    }
    json!({
        "response": resp
    })
}

pub fn priconne_load_history_log(vid: i64, bid: i32) -> serde_json::Value {
    use schema::player_arena_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let log = player_arena_log.filter(schema::player_arena_log::dsl::id.eq(bid)).first::<BasePlayerArenaLog>(&connection).unwrap();

    let uad : serde_json::Value = serde_json::from_str(&log.user_arena_deck).unwrap();
    let vuad : serde_json::Value = serde_json::from_str(&log.vs_user_arena_deck).unwrap();

    use schema::player_data::dsl::player_data;
    let target_vid = if log.battle_2_viewer_id == 0 || log.battle_1_viewer_id != vid {log.battle_1_viewer_id} else {log.battle_2_viewer_id};
    let pdata = player_data.filter(schema::player_data::dsl::viewer_id.eq(target_vid)).first::<BasePlayerData>(&connection).unwrap();

    let mut dmg_list = vec![];
    if log.unit_id_1 != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_1,
            "damage": log.damage_1,
            "viewer_id": log.battle_1_viewer_id
        }));
    }
    if log.unit_id_2 != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_2,
            "damage": log.damage_2,
            "viewer_id": log.battle_1_viewer_id
        }));
    }
    if log.unit_id_3 != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_3,
            "damage": log.damage_3,
            "viewer_id": log.battle_1_viewer_id
        }));
    }
    if log.unit_id_4 != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_4,
            "damage": log.damage_4,
            "viewer_id": log.battle_1_viewer_id
        }));
    }
    if log.unit_id_5 != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_5,
            "damage": log.damage_5,
            "viewer_id": log.battle_1_viewer_id
        }));
    }
    if log.unit_id_1_e != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_1_e,
            "damage": log.damage_1_e,
            "viewer_id": log.battle_2_viewer_id
        }));
    }
    if log.unit_id_2_e != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_2_e,
            "damage": log.damage_2_e,
            "viewer_id": log.battle_2_viewer_id
        }));
    }
    if log.unit_id_3_e != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_3_e,
            "damage": log.damage_3_e,
            "viewer_id": log.battle_2_viewer_id
        }));
    }
    if log.unit_id_4_e != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_4_e,
            "damage": log.damage_4_e,
            "viewer_id": log.battle_2_viewer_id
        }));
    }
    if log.unit_id_5_e != 0 {
        dmg_list.push(json!({
            "unit_id": log.unit_id_5_e,
            "damage": log.damage_5_e,
            "viewer_id": log.battle_2_viewer_id
        }));
    }

    let is_challenge = (log.battle_1_viewer_id == vid) as i32;

    let mut win_or_lose = 0;
    if log.battle_1_viewer_id == vid && log.versus {
        win_or_lose = 1;
    }
    if log.battle_2_viewer_id == vid && !log.versus {
        win_or_lose = 1;
    }

    json!({
        "log_id": log.id,
        "is_challenge": is_challenge,
        "vs_user_viewer_id": pdata.viewer_id,
        "vs_user_team_level": pdata.now_team_level,
        "vs_user_name": pdata.now_name,
        "win_or_lose": win_or_lose,
        "emblem": {
            "emblem_id": pdata.emblem_id,
            "ex_value": 0
        },
        "user_arena_deck": if is_challenge == 1 {&uad} else {&vuad},
        "vs_user_arena_deck": if is_challenge == 1 {&vuad} else {&uad},
        "damage_list": dmg_list
    })
}

pub fn priconne_get_replay(vid: i64, bid: i32) -> serde_json::Value {
    use schema::player_arena_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let log = player_arena_log.filter(schema::player_arena_log::dsl::id.eq(bid)).first::<BasePlayerArenaLog>(&connection).unwrap();

    let is_challenge = (log.battle_1_viewer_id == vid) as i32;
    let mut uad : Vec<serde_json::Value> = serde_json::from_str(&log.user_arena_deck).unwrap();
    let vuad : serde_json::Value = serde_json::from_str(&log.vs_user_arena_deck).unwrap();
    for s in uad.iter_mut() {
        //*s.get_mut("bonus_param").unwrap() = json!({});
        *s.get_mut("unit_param").unwrap() = json!({});
    }
    json!({
        "user_unit_list": uad,
        "opponent_unit_list": vuad,
        "seed": log.seed,
        "is_challenge": is_challenge
    })
}

fn priconne_add_mana_player(vid: i64, mana: i32) -> serde_json::Value {
    use schema::player_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut player = player_data.filter(viewer_id.eq(vid)).first::<BasePlayerData>(&connection).unwrap();
    player.gold_id_free += mana;
    diesel::update(player_data).filter(viewer_id.eq(vid)).set(&player).execute(&connection).unwrap();
    json!({
        "gold_id_free": player.gold_id_free,
        "gold_id_pay": player.gold_id_pay
    })
}

pub fn priconne_learn_skill(vid: i64, req: &Vec<SkillLevelUp>, unid: i32) -> (serde_json::Value, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut udata = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();
    let mut mana_cost = 0;
    for skill_level_up in req {
        let mut step = skill_level_up.step;
        while step != 0 {
            match skill_level_up.location {
                101 => {
                    udata.ub_level += 1;
                    let mana = global_data::SKILL_COST.get().unwrap().iter().find(|x| x.target_level as i32 == udata.ub_level).unwrap().cost;
                    mana_cost += mana;
                },
                201 => {
                    udata.ms_level_1 += 1;
                    let mana = global_data::SKILL_COST.get().unwrap().iter().find(|x| x.target_level as i32 == udata.ms_level_1).unwrap().cost;
                    mana_cost += mana;
                },
                202 => {
                    udata.ms_level_2 += 1;
                    let mana = global_data::SKILL_COST.get().unwrap().iter().find(|x| x.target_level as i32 == udata.ms_level_2).unwrap().cost;
                    mana_cost += mana;
                },
                301 => {
                    udata.ex_level += 1;
                    let mana = global_data::SKILL_COST.get().unwrap().iter().find(|x| x.target_level as i32 == udata.ex_level).unwrap().cost;
                    mana_cost += mana;
                },
                _ => {}
            }
            step -= 1;
        }
    }
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&udata).execute(&connection).unwrap();
    priconne_add_mana_player(vid, -mana_cost as i32);
    let unit_data = create_unit_data(&udata);
    let player = schema::player_data::dsl::player_data.filter(schema::player_data::dsl::viewer_id.eq(vid)).first::<BasePlayerData>(&connection).unwrap();
    (unit_data, json!({
        "gold_id_free": player.gold_id_free,
        "gold_id_pay": player.gold_id_pay
    }))
}

pub fn priconne_unit_equip(vid: i64, equip_slot: &Vec<i32>, unid: i32) -> (serde_json::Value, Vec<serde_json::Value>) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut udata = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();
    let u_promotion = global_data::UNIT_PROMOTION.get().unwrap().iter().find(|x| x.promotion_level as i32 == udata.promotion_level && x.unit_id as i32 == unid).unwrap();
    let mut e_list = vec![];
    for eids in equip_slot {
        let eid = match eids {
            1 => {
                udata.e_lv_1 = 0;
                u_promotion.equip_slot_1
            },
            2 => {
                udata.e_lv_2 = 0;
                u_promotion.equip_slot_2
            },
            3 => {
                udata.e_lv_3 = 0;
                u_promotion.equip_slot_3
            },
            4 => {
                udata.e_lv_4 = 0;
                u_promotion.equip_slot_4
            },
            5 => {
                udata.e_lv_5 = 0;
                u_promotion.equip_slot_5
            },
            6 => {
                udata.e_lv_6 = 0;
                u_promotion.equip_slot_6
            },
            _ => panic!("error in finding equip")
        } as i32;
        let mut uequip_data = schema::player_equip::dsl::player_equip.filter(schema::player_equip::dsl::viewer_id.eq(vid)).filter(schema::player_equip::dsl::equip_id.eq(eid)).first::<BasePlayerEquip>(&connection).unwrap();
        uequip_data.stock -= 1;
        diesel::update(schema::player_equip::dsl::player_equip).filter(schema::player_equip::dsl::viewer_id.eq(vid)).filter(schema::player_equip::dsl::equip_id.eq(eid)).set(&uequip_data).execute(&connection).unwrap();
        e_list.push(json!({
            "id": uequip_data.equip_id,
            "count": 0,
            "stock": uequip_data.stock
        }))
    }
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&udata).execute(&connection).unwrap();
    (create_unit_data(&udata), e_list)
}

pub fn priconne_level_up(vid: i64, item_list: &Vec<PriconneItem>, unid: i32) -> (serde_json::Value, Vec<serde_json::Value>) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut udata = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    let mut full_exp = udata.unit_exp;
    let mut ret = vec![];
    for item in item_list {
        match item.item_id {
            20001 => {
                full_exp += 60 * item.item_num;
            },
            20002 => {
                full_exp += 300 * item.item_num;
            },
            20003 => {
                full_exp += 1500 * item.item_num;
            },
            20004 => {
                full_exp += 7500 * item.item_num;
            },
            _ => {}
        }
        let mut resp = priconne_add_item(vid, item.item_id, -item.item_num);
        resp["count"] = json!(0);
        resp["type"] = json!(2);
        ret.push(resp);
    }
    let final_level = global_data::EXPERIENCE_UNIT.get().unwrap().iter().find(|x| (x.total_exp as i32) <= full_exp).unwrap().unit_level as i32;
    udata.unit_level = final_level;
    udata.unit_exp = full_exp;
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&udata).execute(&connection).unwrap();
    (create_unit_data(&udata), ret)
}

pub fn priconne_consume_equip(vid: i64, e_list: &Vec<PriconneEquipRecipe>) -> Vec<serde_json::Value> {
    use schema::player_equip::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut ret = vec![];
    for eqp in e_list {
        let mut equip = player_equip.filter(viewer_id.eq(vid)).filter(equip_id.eq(eqp.id)).first::<BasePlayerEquip>(&connection).unwrap();
        equip.stock -= eqp.count;
        diesel::update(player_equip).filter(viewer_id.eq(vid)).filter(equip_id.eq(eqp.id)).set(&equip).execute(&connection).unwrap();
        ret.push(json!({
            "id": eqp.id,
            "count": 0,
            "stock": equip.stock
        }))
    }
    ret
}

pub fn priconne_get_mana_object(vid: i64) -> serde_json::Value {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let player = schema::player_data::dsl::player_data.filter(schema::player_data::dsl::viewer_id.eq(vid)).first::<BasePlayerData>(&connection).unwrap();
    json!({
        "gold_id_free": player.gold_id_free,
        "gold_id_pay": player.gold_id_pay
    })
}

pub fn priconne_multi_promotion(vid: i64, unid: i32, target_promotion_level: i32, recipe: &Vec<PromotionEquipRecipe>, item: &Vec<PriconneItem>) -> (serde_json::Value, Vec<serde_json::Value>, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    unit.promotion_level = target_promotion_level;
    unit.e_lv_1 = -1;
    unit.e_pt_1 = 0;
    unit.e_lv_2 = -1;
    unit.e_pt_2 = 0;
    unit.e_lv_3 = -1;
    unit.e_pt_3 = 0;
    unit.e_lv_4 = -1;
    unit.e_pt_4 = 0;
    unit.e_lv_5 = -1;
    unit.e_pt_5 = 0;
    unit.e_lv_6 = -1;
    unit.e_pt_6 = 0;
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();

    let mut equip = vec![];
    for re in recipe {
        let e = priconne_consume_equip(vid, &re.equip_list);
        equip.extend(e);
    }
    let lv_up = priconne_level_up(vid, item, unid);
    (lv_up.0, equip, lv_up.1, priconne_get_mana_object(vid))
}

pub fn priconne_unit_evolution(vid: i64, unid: i32) -> (Vec<serde_json::Value>, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    unit.rarity += 1;
    let arcaea_id = unid / 100 + 30000;
    let item = priconne_add_item(vid, arcaea_id, match unit.rarity {
        2 => -30,
        3 => -100,
        4 => -120,
        5 => -150,
        _ => 0
    });
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    (vec![item], vec![create_unit_data(&unit)], priconne_add_mana_player(vid, -unit.rarity * 10000))
}

pub fn priconne_craft_equip_unique(vid: i64, unid: i32, item_list: &Vec<PriconneEquipRecipe>, equip_recipe: &Vec<PriconneEquipRecipe>) -> (serde_json::Value, Vec<serde_json::Value>, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    unit.ue_level = 1;
    unit.ue_rank = 1;
    let equ = priconne_consume_equip(vid, equip_recipe);
    let mut item = vec![];
    for it in item_list {
        item.push(priconne_add_item(vid, it.id, -it.count));
    }
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    (create_unit_data(&unit), equ, item, priconne_add_mana_player(vid, -1000000))
}

pub fn priconne_get_clan_battle_damage_history() -> Vec<serde_json::Value> {
    use schema::clan_battle_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let history = clan_battle_log.order(log_time.desc()).limit(30).load::<BaseClanBattleLog>(&connection).unwrap();
    let mut resp = vec![];
    for his in history {
        let vid = his.viewer_id;
        let kill_user = schema::player_data::dsl::player_data.filter(schema::player_data::dsl::viewer_id.eq(vid)).first::<BasePlayerData>(&connection).unwrap();
        resp.push(json!({
            "viewer_id": his.viewer_id,
            "enemy_id": his.enemy_id,
            "name": kill_user.now_name,
            "damage": his.total_damage,
            "kill": 0,
            "create_time": his.log_time,
            "history_id": his.id,
            "lap_num": his.lap_num,
            "order_num": format!("{}", his.order_num)
        }));
    }
    resp
}

pub fn priconne_get_history_report(history_id: i32) -> serde_json::Value {
    use schema::clan_battle_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let history = clan_battle_log.filter(id.eq(history_id)).first::<BaseClanBattleLog>(&connection).unwrap();
    let mut history_report = vec![];
    history_report.push(json!({
        "viewer_id": 0,
        "unit_id": history.enemy_id,
        "unit_rarity": 1,
        "promotion_level": 1,
        "damage": history.boss_damage,
        "skin_data": {
            "icon_skin_id": 0
        }
    }));
    if history.u_1 != 0 {
        history_report.push(json!({
            "viewer_id": history.viewer_id,
            "unit_id": history.u_1,
            "unit_rarity": history.u_1_rarity,
            "promotion_level": history.u_1_promotion,
            "damage": history.u_1_damage,
            "skin_data": {
                "icon_skin_id": 0
            }
        }))
    }
    if history.u_2 != 0 {
        history_report.push(json!({
            "viewer_id": history.viewer_id,
            "unit_id": history.u_2,
            "unit_rarity": history.u_2_rarity,
            "promotion_level": history.u_2_promotion,
            "damage": history.u_2_damage,
            "skin_data": {
                "icon_skin_id": 0
            }
        }))
    }
    if history.u_3 != 0 {
        history_report.push(json!({
            "viewer_id": history.viewer_id,
            "unit_id": history.u_3,
            "unit_rarity": history.u_3_rarity,
            "promotion_level": history.u_3_promotion,
            "damage": history.u_3_damage,
            "skin_data": {
                "icon_skin_id": 0
            }
        }))
    }
    if history.u_4 != 0 {
        history_report.push(json!({
            "viewer_id": history.viewer_id,
            "unit_id": history.u_4,
            "unit_rarity": history.u_4_rarity,
            "promotion_level": history.u_4_promotion,
            "damage": history.u_4_damage,
            "skin_data": {
                "icon_skin_id": 0
            }
        }))
    }
    if history.u_5 != 0 {
        history_report.push(json!({
            "viewer_id": history.viewer_id,
            "unit_id": history.u_5,
            "unit_rarity": history.u_5_rarity,
            "promotion_level": history.u_5_promotion,
            "damage": history.u_5_damage,
            "skin_data": {
                "icon_skin_id": 0
            }
        }))
    }
    json!({
        "lap_num": history.lap_num,
        "order_num": format!("{}", history.order_num),
        "history_report": history_report
    })
}

pub fn priconne_insert_clan_battle_battle_log(log: &BattleLog2Request) {
    use schema::clan_battle_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let logss = clan_battle_log.filter(battle_log.eq(format!("unusedBattleLogId={}", &log.battle_log_id))).first::<BaseClanBattleLog>(&connection);
    match logss {
        Ok(mut resp) => {
            resp.battle_log = log.battle_log.clone();
            diesel::update(clan_battle_log).filter(id.eq(resp.id)).set(&resp).execute(&connection).unwrap();
        },
        Err(_) => {}
    }
}

pub fn priconne_clan_battle_rehearsal_finish(vid: i64, req: &ClanBattleRehearsalFinishRequest) {
    use schema::clan_battle_log::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let cfg = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    let boss_info = clan_battle::get_clan_battle_boss_info(req.lap_num as i64, cfg.clanbattle_id as i64)[(req.order_num - 1) as usize]["enemy_id"].as_i64().unwrap() as i32;
    let mut now_data = NewClanBattleLog {
        viewer_id: vid,
        order_num: req.order_num,
        lap_num: req.lap_num,
        clan_battle_id: cfg.clanbattle_id as i32,
        log_time: chrono::Utc::now().timestamp(),
        battle_log: format!("unusedBattleLogId={}", req.battle_log_id),

        enemy_id: boss_info,
        boss_damage: req.boss_damage,
        total_damage: req.total_damage,

        u_1: 0,
        u_1_damage: 0,
        u_1_rarity: 0,
        u_1_promotion: 0,
        u_2: 0,
        u_2_damage: 0,
        u_2_rarity: 0,
        u_2_promotion: 0,
        u_3: 0,
        u_3_damage: 0,
        u_3_rarity: 0,
        u_3_promotion: 0,
        u_4: 0,
        u_4_damage: 0,
        u_4_rarity: 0,
        u_4_promotion: 0,
        u_5: 0,
        u_5_damage: 0,
        u_5_rarity: 0,
        u_5_promotion: 0,
    };
    let dmg_len = req.user_unit.unit_damage_list.len();
    if dmg_len >= 1 {
        let dmg = &req.user_unit.unit_damage_list[0];
        let unit = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(vid)).filter(schema::player_unit_data::dsl::unit_id.eq(dmg.unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        now_data.u_1 = dmg.unit_id;
        now_data.u_1_damage = dmg.damage;
        now_data.u_1_rarity = unit.rarity;
        now_data.u_1_promotion = unit.promotion_level;
    }
    if dmg_len >= 2 {
        let dmg = &req.user_unit.unit_damage_list[1];
        let unit = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(vid)).filter(schema::player_unit_data::dsl::unit_id.eq(dmg.unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        now_data.u_2 = dmg.unit_id;
        now_data.u_2_damage = dmg.damage;
        now_data.u_2_rarity = unit.rarity;
        now_data.u_2_promotion = unit.promotion_level;
    }
    if dmg_len >= 3 {
        let dmg = &req.user_unit.unit_damage_list[2];
        let unit = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(vid)).filter(schema::player_unit_data::dsl::unit_id.eq(dmg.unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        now_data.u_3 = dmg.unit_id;
        now_data.u_3_damage = dmg.damage;
        now_data.u_3_rarity = unit.rarity;
        now_data.u_3_promotion = unit.promotion_level;
    }
    if dmg_len >= 4 {
        let dmg = &req.user_unit.unit_damage_list[3];
        let unit = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(vid)).filter(schema::player_unit_data::dsl::unit_id.eq(dmg.unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        now_data.u_4 = dmg.unit_id;
        now_data.u_4_damage = dmg.damage;
        now_data.u_4_rarity = unit.rarity;
        now_data.u_4_promotion = unit.promotion_level;
    }
    if dmg_len >= 5 {
        let dmg = &req.user_unit.unit_damage_list[4];
        let unit = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(vid)).filter(schema::player_unit_data::dsl::unit_id.eq(dmg.unit_id)).first::<BasePlayerUnitData>(&connection).unwrap();
        now_data.u_5 = dmg.unit_id;
        now_data.u_5_damage = dmg.damage;
        now_data.u_5_rarity = unit.rarity;
        now_data.u_5_promotion = unit.promotion_level;
    }
    diesel::insert_into(clan_battle_log).values(now_data).execute(&connection).unwrap();
}

pub fn priconne_equip_enhance(vid: i64, unid: i32, item_list: &Vec<PriconneEquipRecipe>, slot: i32) -> (serde_json::Value, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    let mut full_exp = match slot {
        1 => unit.e_pt_1,
        2 => unit.e_pt_2,
        3 => unit.e_pt_3,
        4 => unit.e_pt_4,
        5 => unit.e_pt_5,
        6 => unit.e_pt_6,
        _ => 0
    };
    let before_exp = full_exp.clone();
    let mut ret_item = vec![];
    for item in item_list {
        match item.id {
            22001 => full_exp += 10 * item.count,
            22002 => full_exp += 60 * item.count,
            22003 => full_exp += 200 * item.count,
            _ => {}
        }
        let mut i = priconne_add_item(vid, item.id, -item.count);
        i["type"] = json!(2);
        ret_item.push(i);
    }

    let equip = global_data::UNIT_PROMOTION.get().unwrap().iter().find(|x| x.unit_id as i32 == unid && x.promotion_level as i32 == unit.promotion_level).unwrap();
    let equip_id = match slot {
        1 => equip.equip_slot_1,
        2 => equip.equip_slot_2,
        3 => equip.equip_slot_3,
        4 => equip.equip_slot_4,
        5 => equip.equip_slot_5,
        6 => equip.equip_slot_6,
        _ => 999999
    };
    if equip_id != 999999 {
        let ueq = global_data::EQUIPMENT_DATA.get().unwrap().iter().find(|x| x.equipment_id == equip_id).unwrap();
        let final_level = match global_data::EQUIPMENT_ENHANCE_DATA.get().unwrap().iter().filter(|x| x.promotion_level == ueq.promotion_level).find(|x| x.total_point as i32 <= full_exp) {
            Some(r) => r.equipment_enhance_level as i32,
            None => {
                println!("error in enhancing {} {}", ueq.promotion_level, full_exp);
                0
            }
        };
        match slot {
            1 => {
                unit.e_lv_1 = final_level;
                unit.e_pt_1 = full_exp;
            },
            2 => {
                unit.e_lv_2 = final_level;
                unit.e_pt_2 = full_exp;
            },
            3 => {
                unit.e_lv_3 = final_level;
                unit.e_pt_3 = full_exp;
            },
            4 => {
                unit.e_lv_4 = final_level;
                unit.e_pt_4 = full_exp;
            },
            5 => {
                unit.e_lv_5 = final_level;
                unit.e_pt_5 = full_exp;
            },
            6 => {
                unit.e_lv_6 = final_level;
                unit.e_pt_6 = full_exp;
            },
            _ => {}
        }
    }
    let mana_cost = (full_exp - before_exp) * 200;
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    (create_unit_data(&unit), ret_item, priconne_add_mana_player(vid, -mana_cost))
}

pub fn priconne_equip_enhance_unique(vid: i64, unid: i32, item_list: &Vec<PriconneEquipRecipe>) -> (serde_json::Value, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    let mut ret_item = vec![];
    for item in item_list {
        match item.id {
            22001 => unit.ue_pt += 10 * item.count,
            22002 => unit.ue_pt += 60 * item.count,
            22003 => unit.ue_pt += 200 * item.count,
            _ => {}
        }
        let mut i = priconne_add_item(vid, item.id, -item.count);
        i["type"] = json!(2);
        ret_item.push(i);
    }
    let fg : Vec<&UniqueEquipmentEnhanceData> = global_data::UNIQUE_EQUIPMENT_ENHANCE_DATA.get().unwrap().iter().filter(|x| (x.total_point as i32) <= unit.ue_pt && (x.rank as i32) <= unit.ue_rank).collect();
    let final_level = fg.len() as i32 + 1;
    let mut full_mana = 0;
    for f in fg {
        full_mana += f.needed_mana;
    }
    unit.ue_level = final_level;
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    (create_unit_data(&unit), ret_item, priconne_add_mana_player(vid, -full_mana as i32))
}

pub fn priconne_equip_rankup_unique(vid: i64, unid: i32, item_list: &Vec<PriconneEquipRecipe>, eqp: &Vec<PriconneEquipRecipe>) -> (serde_json::Value, Vec<serde_json::Value>, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    let mut items = vec![];
    for i in item_list {
        items.push(priconne_add_item(vid, i.id, -i.count));
    }

    let eqs = priconne_consume_equip(vid, eqp);

    let u_eid = global_data::UNIT_UNIQUE_EQUIP.get().unwrap().iter().find(|x| x.unit_id as i32 == unid).unwrap();
    let max = global_data::UNIQUE_EQUIPMENT_RANKUP.get().unwrap().iter().find(|x| x.equip_id == u_eid.equip_id && x.unique_equip_rank as i32 == unit.ue_rank).unwrap();
    let max_pt = global_data::UNIQUE_EQUIPMENT_ENHANCE_DATA.get().unwrap().iter().find(|x| x.enhance_level == max.unit_level).unwrap().total_point;
    if unit.ue_level == max.unit_level as i32 {
        unit.ue_pt = max_pt as i32;
    }
    unit.ue_rank += 1;
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    (create_unit_data(&unit), items, eqs, priconne_add_mana_player(vid, -max.crafted_cost as i32))
}

pub fn priconne_get_profile(target_vid: i64) -> serde_json::Value {
    let pdata = priconne_get_player_data(target_vid);
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    use schema::player_arena_data::dsl::*;
    let arena = player_arena_data.filter(viewer_id.eq(target_vid)).first::<BasePlayerArenaData>(&connection).unwrap_or(BasePlayerArenaData {
        id: 0,
        arena_rank: 0,
        viewer_id: target_vid,
        battle_num: 0
    });

    let story_count = schema::player_story::dsl::player_story.filter(schema::player_story::dsl::viewer_id.eq(target_vid)).filter(schema::player_story::dsl::seen.eq(true)).count().get_result::<i64>(&connection).unwrap();
    let unit_num = schema::player_unit_data::dsl::player_unit_data.filter(schema::player_unit_data::dsl::viewer_id.eq(target_vid)).count().get_result::<i64>(&connection).unwrap();
    json!({
        "user_info": {
            "viewer_id": pdata.viewer_id,
            "user_name": pdata.now_name,
            "user_comment": pdata.user_comment,
            "team_level": pdata.now_team_level,
            "team_exp": 0,
            "emblem": {
                "emblem_id": pdata.emblem_id,
                "ex_value": 0
            },
            "last_login_time": 0,
            "arena_rank": arena.arena_rank,
            "arena_group": if arena.arena_rank == 0 {0} else {1},
            "arena_time": if arena.arena_rank == 0 {0} else {1624291200},
            "grand_arena_rank": 0,
            "grand_arena_group": 0,
            "grand_arena_time": 0,
            "open_story_num": story_count,
            "unit_num": unit_num,
            "total_power": 0,
            "tower_cleared_floor_num": 0,
            "tower_cleared_ex_quest_count": 0,
            "friend_num": 0
        },
        "quest_info": {
            "normal_quest": [0, 0, 0],
            "hard_quest": [0, 0, 0],
            "very_hard_quest": [0, 0, 0]
        },
        "clan_name": "AYANE_SERVER",
        "friend_support_units": [],
        "clan_support_units": []
    })
}

pub fn priconne_rename(vid: i64, user_name: String) {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut pdata = priconne_get_player_data(vid);
    pdata.now_name = user_name;
    use schema::player_data::dsl::*;
    diesel::update(player_data).filter(viewer_id.eq(vid)).set(&pdata).execute(&connection).unwrap();
}

pub fn priconne_set_comment(vid: i64, comment: String) {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut pdata = priconne_get_player_data(vid);
    pdata.user_comment = comment;
    use schema::player_data::dsl::*;
    diesel::update(player_data).filter(viewer_id.eq(vid)).set(&pdata).execute(&connection).unwrap();
}

pub fn priconne_unit_craft_equip(vid: i64, unid: i32, equip_slot_num: i32, equip_recipe_list: &Vec<PriconneEquipRecipe>, item_list: &Vec<PriconneItem>) -> (serde_json::Value, Vec<serde_json::Value>, Vec<serde_json::Value>, serde_json::Value) {
    use schema::player_unit_data::dsl::*;
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut unit = player_unit_data.filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).first::<BasePlayerUnitData>(&connection).unwrap();

    match equip_slot_num {
        1 => unit.e_lv_1 = 0,
        2 => unit.e_lv_2 = 0,
        3 => unit.e_lv_3 = 0,
        4 => unit.e_lv_4 = 0,
        5 => unit.e_lv_5 = 0,
        6 => unit.e_lv_6 = 0,
        _ => {}
    };
    diesel::update(player_unit_data).filter(viewer_id.eq(vid)).filter(unit_id.eq(unid)).set(&unit).execute(&connection).unwrap();
    let eqp = priconne_consume_equip(vid, equip_recipe_list);
    let lv_up = priconne_level_up(vid, item_list, unid);
    (lv_up.0, eqp, lv_up.1, priconne_get_mana_object(vid))
}

pub fn priconne_set_fav(vid: i64, fav_id: i32) {
    let connection = CONNECTION_POOL.get().unwrap().get().unwrap();
    let mut pdata = priconne_get_player_data(vid);
    pdata.favorite_unit_id = fav_id;
    use schema::player_data::dsl::*;
    diesel::update(player_data).filter(viewer_id.eq(vid)).set(&pdata).execute(&connection).unwrap();
}