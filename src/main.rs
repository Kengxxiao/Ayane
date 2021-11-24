#[macro_use]
extern crate lazy_static;

mod global_data;
mod cryptor;
mod priconne_model;

pub mod clan_battle;
pub mod database;
pub mod schema;
pub mod model;

use actix_web::{App, HttpServer, Responder, get, options, post};
use actix_web::middleware::Logger;
use cryptor::GlobalSerde;
use database::priconne_sdk_login;
use priconne_model::*;
use rand::{Rng, thread_rng};
use serde_json::json;

use crate::global_data::UnitData;

#[macro_use]
extern crate diesel;

#[post("/tool/sdk_login")]
async fn tool_sdk_login(body: GlobalSerde<SdkLoginRequest>) -> impl Responder {
    match priconne_sdk_login(body.0.uid.clone(), body.2.unwrap()) {
        Ok(_) => {
            return GlobalSerde::success(json!({
                "is_new_user": 0
            }))
        },
        Err(r) => {
            return cryptor::get_fail_resp(&r.to_string(), 3);
        }
    };
}

#[post("/check/game_start")]
async fn check_game_start(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10001)
    }
    let p = database::priconne_get_player_data(vid);
    GlobalSerde::success(json!({
        "now_viewer_id": p.viewer_id,
        "is_set_transition_password": false,
        "now_name": p.now_name,
        "now_team_level": p.now_team_level,
        "now_tutorial": true,
        "transition_account_data": [],
        "bundle_ver": "",
        "resource_fix": false,
        "bundle_fix": false
      }))
}

#[post("/present/index")]
async fn present_index(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10002)
    }
    let p = database::priconne_get_present_list(vid, false);
    GlobalSerde::success(json!({
        "present_info_list": p,
        "present_count": p.len()
    }))
}

#[post("/present/receive")]
async fn present_receive(body: GlobalSerde<PresentReceiveRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10003)
    }
    let ret = database::priconne_receive_present(vid, body.present_id);
    if ret.is_null() {
        return cryptor::get_fail_resp("无法领取礼物", 20003)
    }
    GlobalSerde::success(json!({
        "rewards": [ret],
        "flag_over_limit": 0,
        "flag_expiration": 0
    }))
}

#[post("/present/receive_all")]
async fn present_receive_all(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10004)
    }
    let ret = database::priconne_receive_all_presents(vid);
    GlobalSerde::success(json!({
        "rewards": ret,
        "flag_over_limit": 0,
        "flag_expiration": 0,
        "present_info_list": [],
        "stamina_info": {
            "user_stamina": 0,
            "stamina_full_recovery_time": 0
        }
    }))
}

#[post("/home/index")]
async fn home_index(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10005)
    }
    GlobalSerde::success(json!({
        "unread_message_list": [],
        "have_clan_battle_reward": 0,
        "last_friend_time": {
            "accept": 0,
            "pending": 0
        },
        "missions": [],
        "season_pack": [],
        "daily_reset_time": chrono::Utc::now().timestamp() - 1,
        "user_clan": { //todo
            "clan_id": 1,
            "leave_time": 0,
            "clan_member_count": 1,
            "latest_request_time": 0,
            "donation_num": 0
        },
        "have_clan_invitation": 0,
        "quest_list": database::priconne_local_get_all_quests(),
        "training_quest_count": {
            "gold_quest": 0,
            "exp_quest": 0
        },
        "training_quest_max_count": {
            "gold_quest": 0,
            "exp_quest": 0
        },
        "training_quest_pack_end_time": 0,
        "dungeon_info": {
            "dungeon_area": [],
            "enter_area_id": 0,
            "rest_challenge_count": []
        }
    }))
}

#[post("/dungeon/info")]
async fn dungeon_info(_: GlobalSerde<EmptyRequest>) -> impl Responder {
    GlobalSerde::success(json!({
        "dungeon_area": [
            {
                "dungeon_type": 1,
                "dungeon_area_ids": []
            }
        ],
        "enter_area_id": 0,
        "rest_challenge_count": [
            {
                "dungeon_type": 1,
                "count": 0,
                "max_count": 0
            }
        ],
        "dungeon_cleared_area_id_list": []
    }))
}

#[post("/load/index")]
async fn load_index(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10099);
    }
    let story = database::priconne_get_player_story(vid);
    let p = database::priconne_get_player_data(vid);
    if p.now_team_level <= 1 {
        return cryptor::get_fail_resp("该账号没有登录权限", 114514);
    }
    GlobalSerde::success(json!({
        "user_info": {
            "viewer_id": p.viewer_id,
            "user_name": p.now_name,
            "user_comment": p.user_comment,
            "team_level": p.now_team_level,
            "user_stamina": 0,
            "max_stamina": 0,
            "team_exp": 0,
            "favorite_unit_id": p.favorite_unit_id,
            "tutorial_flag": 100,
            "invite_accept_flag": 0,
            "user_birth": 0,
            "platform_id": 1,
            "channel_id": 1000,
            "last_ac": "/debug/priconne_server_test",
            "last_ac_time": 0,
            "server_id": 1000001,
            "reg_time": 0,
            "stamina_full_recovery_time": 0,
            "emblem": {
                "emblem_id": p.emblem_id,
                "ex_value": 0
            }
        },
        "user_jewel": {
            "jewel": p.free_jewel + p.paid_jewel,
            "free_jewel": p.free_jewel,
            "paid_jewel": p.paid_jewel
        },
        "user_gold": {
            "gold_id_free": p.gold_id_free,
            "gold_id_pay": p.gold_id_pay
        },
        "unit_list": database::priconne_get_full_unit_data(vid),
        "user_chara_info": database::priconne_get_full_user_chara_info(),
        "deck_list": database::priconne_get_full_deck_list(vid),
        "item_list": database::priconne_get_items(vid),
        "user_equip": database::priconne_get_equip(vid), //todo
        "shop": {
            "alchemy": {
                "max_count": 0,
                "exec_count": 0
            },
            "recover_stamina": {
                "count": 0,
                "max_count": 0,
                "exec_count": 0,
                "recovery": 120,
                "cost": 40 //体力回复
            }
        },
        "ini_setting": global_data::INI_SETTING.get().unwrap(),
        "max_storage_num": 0,
        "campaign_list": [],
        "can_free_gacha": 0,
        "can_campaign_gacha": 0,
        "gacha_point_info": {
            "exchange_id": 34,
            "current_point": 0,
            "max_point": 0
        },
        "read_story_ids": story.0,
        "unlock_story_ids": story.1,
        "event_statuses": [], //活动 todo
        "user_my_party": [],
        "user_my_party_tab": [],
        "daily_reset_time": 0, //日常刷新时间
        "login_bonus_list": {
            "first": [],
            "normal": [],
            "campaign": [],
            "lottery": [],
            "adv": [],
            "countdown": []
        },
        "present_count": database::priconne_get_present_count(vid), //礼物箱
        "clan_like_count": 0,
        "dispatch_units": [],
        "clan_battle": { //公会战模拟 配置
            "now_open": 0,
            "is_interval": 1,
            "mode_change_limit_time": 0,
            "mode_change_limit_start_time": 0,
            "mode_change_limit_remind_time": 0
        },
        "voice": [],
        "csc": { //大师币
            "weekly": {
                "count": 0,
                "max": 0
            }
        },
        "today_start_level": 0
    }))
}

#[post("/story/check")]
async fn story_check(_: GlobalSerde<StoryRequest>) -> impl Responder {
    GlobalSerde::success(json!([]))
}

#[post("/story/start")]
async fn story_start(body: GlobalSerde<StoryRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 10088)
    }
    database::priconne_read_story(vid, body.story_id);
    GlobalSerde::success(json!({
        "reward_info": []
    }))
}

#[post("/shop/item_list")]
async fn shop_item_list(_: GlobalSerde<EmptyRequest>) -> impl Responder {
    GlobalSerde::success(json!({
        "shop_list": database::priconne_local_get_shop_list(),
        "is_got_csc": 0
    }))
}

#[post("/shop/buy")]
async fn shop_buy(body: GlobalSerde<BuyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 99877)
    }
    let uid = (body.slot_id - 9000) + 30000;
    let item = database::priconne_add_item(vid, uid, body.number * 1000);
    let godness = database::priconne_add_item(vid, 90005, 0);
    GlobalSerde::success(json!({
        "purchase_list": [
            {
                "id": uid,
                "type": 2,
                "count": body.number * 1000,
                "stock": item["stock"].as_i64().unwrap(),
                "received": body.number * 1000
            }
        ],
        "item_data": [godness]
    }))
}

#[post("/arena/info")]
async fn arena_info(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 99876)
    }
    GlobalSerde::success(json!({
        "arena_info": database::priconne_get_arena_info(vid),
        "reward_info": {
            "id": 90003,
            "type": 2,
            "count": 0
        },
        "reward_hour_num": 0,
        "is_time_reward_max": false,
        "search_opponent": database::priconne_get_arena_search_opponent(vid)
    }))
}

#[post("/arena/search")]
async fn arena_search(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 99875)
    }
    GlobalSerde::success(json!({
        "search_opponent": database::priconne_get_arena_search_opponent(vid)
    }))
}

#[post("/arena/cancel")]
async fn arena_cancel(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 99874)
    }
    GlobalSerde::success(json!({
        "search_opponent": database::priconne_get_arena_search_opponent(vid)
    }))
}

#[post("/arena/apply")]
async fn arena_apply(_: GlobalSerde<EmptyRequest>) -> impl Responder {
    GlobalSerde::success(json!([]))
}

#[post("/arena/start")]
async fn arena_start(body: GlobalSerde<ArenaStartRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 123)
    }
    let uad = database::create_opponent_arena_data(vid, 2, true);
    let vuad = database::create_opponent_arena_data(body.battle_viewer_id, 3, true);
    let seed : i32 = thread_rng().gen();
    let battle_id = database::priconne_create_arena_log(vid, body.battle_viewer_id, &uad, &vuad, seed.into(), body.token.clone());
    GlobalSerde::success(json!({
        "battle_viewer_id": body.battle_viewer_id,
        "battle_id": battle_id,
        "wave_info_list": [{
            "battle_log_id": battle_id,
            "seed": seed,
            "user_arena_deck": uad,
            "vs_user_arena_deck": vuad,
            "wave_num": 1
        }],
        "notification": {"mission": []}
    }))
}

#[post("/payment/item_list")]
async fn payment_item_list() -> impl Responder {
    GlobalSerde::success(json!([]))
}

#[post("/deck/update")]
async fn deck_update(body: GlobalSerde<DeckUpdateRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 8848)
    }
    database::priconne_update_deck(vid, body.deck_number, body.unit_id_1, body.unit_id_2, body.unit_id_3, body.unit_id_4, body.unit_id_5);
    GlobalSerde::success(json!({}))
}

#[post("/arena/finish")]
async fn arena_finish(body: GlobalSerde<ArenaFinishRequest>) -> impl Responder {
    let record = database::priconne_set_log(&body.0);
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 124)
    }
    GlobalSerde::success(json!({
        "arena_info": database::priconne_get_arena_info(vid),
        "old_record": record.1,
        "new_record": record.0
    }))
}

#[post("/log/battle_log2")]
async fn log_battle_log_2(body: GlobalSerde<BattleLog2Request>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 7749)
    }
    if body.system_id == 107 {
        database::priconne_insert_clan_battle_battle_log(&body);
    }
    GlobalSerde::success(json!([]))
}

#[post("/deck/update_list")]
async fn deck_update_list(body: GlobalSerde<DeckUpdateListRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 8848)
    }
    for p in &body.deck_list {
        database::priconne_update_deck(vid, p.deck_number, p.unit_list[0], p.unit_list[1], p.unit_list[2], p.unit_list[3], p.unit_list[4]);
    }
    GlobalSerde::success(json!({}))
}

#[post("/arena/history")]
async fn arena_history(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 476)
    }
    GlobalSerde::success(json!({
        "versus_result_list": database::priconne_get_log_list(vid),
    }))
}

#[post("/arena/history_detail")]
async fn arena_history_detail(body: GlobalSerde<ArenaHistoryDetailRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 475)
    }
    GlobalSerde::success(json!({
        "versus_result_detail": database::priconne_load_history_log(vid, body.log_id)
    }))
}

#[post("/arena/replay")]
async fn arena_replay(body: GlobalSerde<ArenaHistoryDetailRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 460)
    }
    GlobalSerde::success(database::priconne_get_replay(vid, body.log_id))
}

#[post("/skill/level_up")]
async fn skill_level_up(body: GlobalSerde<SkillLevelUpRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90000)
    }
    let resp = database::priconne_learn_skill(vid, &body.0.skill_levelup_list, body.0.unit_id);
    GlobalSerde::success(json!({
        "unit_data": resp.0,
        "user_gold": resp.1,
        "notification": {"mission": []}
    }))
}

#[post("/unit/equip")]
async fn unit_equip(body: GlobalSerde<UnitEquipRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90001)
    }
    let ret = database::priconne_unit_equip(vid, &vec![body.equip_slot_num], body.unit_id);
    GlobalSerde::success(json!({
        "unit_data": ret.0,
        "equip_data": ret.1[0]
    }))
}

#[post("/unit/automatic_enhance")]
async fn unit_automatic_enhance(body: GlobalSerde<UnitAutomaticEnhanceRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90002)
    }
    let eqp = database::priconne_unit_equip(vid, &body.equip_slot_num_list, body.unit_id);
    let item = database::priconne_level_up(vid, &body.item_list, body.unit_id);
    let skill = database::priconne_learn_skill(vid, &body.skill_levelup_list, body.unit_id);
    GlobalSerde::success(json!({
        "unit_data": skill.0,
        "user_gold": skill.1,
        "notification": {"mission": []},
        "item_data": item.1,
        "equip_list": eqp.1
    }))
}

#[post("/unit/multi_promotion")]
async fn unit_multi_promotion(body: GlobalSerde<MultiPromotionRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90003)
    }
    let p = database::priconne_multi_promotion(vid, body.unit_id, body.target_promotion_level, &body.equip_recipe_list, &body.item_list);
    GlobalSerde::success(json!({
        "unit_data": p.0,
        "refund_items": [],
        "equip_list": p.1,
        "item_data": p.2,
        "user_gold": p.3
    }))
}

#[post("/item/exp")]
async fn item_exp(body: GlobalSerde<ItemExpRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90004)
    }
    let exp = database::priconne_level_up(vid, &body.item_list, body.unit_id);
    GlobalSerde::success(json!({
        "unit_data": exp.0,
        "item_data": exp.1
    }))
}

#[post("/unit/evolution")]
async fn unit_evolution(body: GlobalSerde<UnitEvolutionRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90005)
    }
    let evo = database::priconne_unit_evolution(vid, body.unit_id);
    GlobalSerde::success(json!({
        "item_data": evo.0,
        "unit_data_list": evo.1,
        "user_gold": evo.2
    }))
}

#[post("/equipment/enhance")]
async fn equipment_enhance(body: GlobalSerde<EquipmentEnhanceRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90006)
    }
    let p = database::priconne_equip_enhance(vid, body.unit_id, &body.item_list, body.equip_slot_num);
    GlobalSerde::success(json!({
        "user_gold": p.2,
        "item_list": p.1,
        "unit_data": p.0
    }))
}

#[post("/unit/craft_equip_unique")]
async fn unit_craft_equip_unique(body: GlobalSerde<CraftEquipUniqueRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90007)
    }
    let p = database::priconne_craft_equip_unique(vid, body.unit_id, &body.item_recipe_list, &body.equip_recipe_list);
    GlobalSerde::success(json!({
        "unit_data": p.0,
        "equip_list": p.1,
        "item_data": p.2,
        "user_gold": p.3
    }))
}

#[post("/equipment/enhance_unique")]
async fn equipment_enhance_unique(body: GlobalSerde<EquipmentEnhanceRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90008)
    }
    let p = database::priconne_equip_enhance_unique(vid, body.unit_id, &body.item_list);
    GlobalSerde::success(json!({
        "unit_data": p.0,
        "item_list": p.1,
        "user_gold": p.2
    }))
}

#[post("/equipment/rankup_unique")]
async fn equipment_rankup_unique(body: GlobalSerde<EquipmentRankupUniqueRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90009)
    }
    let p = database::priconne_equip_rankup_unique(vid, body.unit_id, &body.item_recipe_list, &body.equip_recipe_list);
    GlobalSerde::success(json!({
        "unit_data": p.0,
        "equip_list": p.2,
        "item_list": p.1,
        "user_gold": p.3
    }))
}

#[post("/profile/get_profile")]
async fn profile_get_profile(body: GlobalSerde<GetProfileRequest>) -> impl Responder {
    GlobalSerde::success(database::priconne_get_profile(body.target_viewer_id))
}

#[post("/profile/rename")]
async fn profile_rename(body: GlobalSerde<ProfileRenameRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 91001)
    }
    database::priconne_rename(vid, body.user_name.clone());
    GlobalSerde::success(json!([]))
}

#[post("/profile/update_comment")]
async fn profile_update_comment(body: GlobalSerde<ProfileUpdateCommentRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 91002)
    }
    database::priconne_set_comment(vid, body.user_comment.clone());
    GlobalSerde::success(json!([]))
}

#[post("/profile/favorite_unit")]
async fn profile_favorite_unit(body: GlobalSerde<ProfileFavoriteUnitRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 91003)
    }
    database::priconne_set_fav(vid, body.unit_id);
    GlobalSerde::success(json!([]))
}

#[post("/present/history")]
async fn present_history(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 82)
    }
    GlobalSerde::success(json!({
        "present_history": database::priconne_get_present_list(vid, true)
    }))
}

#[post("/source_ini/get_maintenance_status")]
async fn get_maintenance_status() -> impl Responder {
    GlobalSerde::success(global_data::RESOURCE_INFO.get().unwrap())
}

#[post("/unit/craft_equip")]
async fn unit_craft_equip(body: GlobalSerde<UnitCraftEquipRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 90010)
    }
    let p = database::priconne_unit_craft_equip(vid, body.unit_id, body.equip_slot_num, &body.equip_recipe_list, &body.item_list);
    GlobalSerde::success(json!({
        "unit_data": p.0,
        "equip_list": p.1,
        "item_data": p.2,
        "user_gold": p.3
    }))
}

#[post("/clan_battle/top")]
async fn clan_battle_top(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210001)
    }
    let config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    let boss_info = clan_battle::get_clan_battle_boss_info(config.lap_num, config.clanbattle_id);
    let clan_battle_id = if clan_battle::check_clan_battle_id_safe(config.clanbattle_id) {
        config.clanbattle_id
    } else {
        config.clanbattle_id - 12
    };
    GlobalSerde::success(json!({
        "clan_battle_id": clan_battle_id,
        "period": 1,
        "lap_num": config.lap_num, //回合数 调整
        "boss_info": boss_info,
        "damage_history": database::priconne_get_clan_battle_damage_history(),
        "period_rank": 0,
        "point": 0,
        "remaining_count": 0,
        "carry_over_time": 0,
        "change_season": 0,
        "change_period": 0,
        "last_rank_result": [],
        "used_unit": []
    }))
}

#[post("/clan_battle/boss_info")]
async fn clan_battle_boss_info(body: GlobalSerde<ClanBattleRehearsalStartRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210002)
    }
    let config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    let boss_data = &clan_battle::get_clan_battle_boss_info(body.lap_num as i64, config.clanbattle_id)[(body.order_num - 1) as usize];
    GlobalSerde::success(json!({
        "damage_history": [],
        "current_hp": boss_data["max_hp"].as_i64()
    }))
}

#[post("/clan_battle/reload_detail_info")]
async fn clan_battle_reload_detail_info(body: GlobalSerde<ClanBattleRehearsalStartRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210003)
    }
    let config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    let boss_data = &clan_battle::get_clan_battle_boss_info(body.lap_num as i64, config.clanbattle_id)[(body.order_num - 1) as usize];
    GlobalSerde::success(json!({
        "fighter_num": 0,
        "current_hp": boss_data["max_hp"].as_i64()
    }))
}

#[post("/clan_battle/support_unit_list_2")]
async fn clan_battle_support_unit_list(body: GlobalSerde<EmptyRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210004)
    }
    GlobalSerde::success(json!({
        "support_unit_list": []
    }))
}

#[post("/clan_battle/rehearsal_start")]
async fn clan_battle_rehearsal_start(body: GlobalSerde<ClanBattleRehearsalStartRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210005)
    }
    let config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    let boss_data = &clan_battle::get_clan_battle_boss_info(body.lap_num as i64, config.clanbattle_id)[(body.order_num - 1) as usize];
    let enemy_data = clan_battle::get_clan_battle_enemy_data(boss_data["enemy_id"].as_i64().unwrap());
    let player_unit_list = database::get_deck_list(vid, 14);
    let mut skin_data_for_request = vec![];
    for unit_id in vec![player_unit_list.unit_id_1, player_unit_list.unit_id_2, player_unit_list.unit_id_3, player_unit_list.unit_id_4, player_unit_list.unit_id_5] {
        skin_data_for_request.push(json!({
            "unit_id": unit_id,
            "icon_skin_id": 0,
            "still_skin_id": 0,
            "sd_skin_id": 0,
            "motion_id": 0
        }))
    }
    let seed = rand::thread_rng().gen::<i32>();
    GlobalSerde::success(json!({
        "limit_time": 90,
        "enemy_data": enemy_data,
        "battle_log_id": seed,
        "skin_data_for_request": skin_data_for_request,
        "seed": seed,
        "current_hp": boss_data["max_hp"].as_i64()
    }))
}

#[post("/clan_battle/rehearsal_finish")]
async fn clan_battle_rehearsal_finish(body: GlobalSerde<ClanBattleRehearsalFinishRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210006)
    }
    database::priconne_clan_battle_rehearsal_finish(vid, &body.0);
    GlobalSerde::success(json!({}))
}

#[post("/clan_battle/update_deck")]
async fn clan_battle_update_deck(body: GlobalSerde<ClanBattleDeckUpdateRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210007)
    }
    database::priconne_update_deck(vid, 14, body.unit_id_1, body.unit_id_2, body.unit_id_3, body.unit_id_4, body.unit_id_5);
    GlobalSerde::success(json!({}))
}

#[post("/clan_battle/history_report")]
async fn clan_battle_history_report(body: GlobalSerde<ClanBattleHistoryReportRequest>) -> impl Responder {
    let vid = cryptor::decrypt_base64_encrypted(body.viewer_id.clone());
    if vid == 0 {
        return cryptor::get_fail_resp("无法解析请求", 210008)
    }
    GlobalSerde::success(database::priconne_get_history_report(body.history_id))
}

#[get("/himari_debug/getUnitList")]
async fn himari_debug_get_unit_list() -> impl Responder {
    let unit_all: Vec<&UnitData> = global_data::UNIT_DATA.get().unwrap().iter().filter(|x| x.comment.len() != 0).collect();
    let mut game_unit = vec![];
    for s in unit_all {
        game_unit.push(json!({
            "id": s.unit_id,
            "width": s.search_area_width
        }));
    }
    GlobalSerde::success(json!({
        "unit_list": game_unit
    }))
}
#[options("/himari_debug/getArenaLogWithUnitId")]
async fn himari_debug_get_arena_log_with_unit_id_options() -> impl Responder {
    actix_http::Response::NoContent()
}
#[post("/himari_debug/getArenaLogWithUnitId")]
async fn himari_debug_get_arena_log_with_unit_id(param: GlobalSerde<GameUnitListForArenaSearch>) -> impl Responder {
    let c = database::priconne_debug_get_unit_list_from_successor(param.0);
    GlobalSerde::success(c)
}
#[get("/himari_debug/getClanBattleIdList")]
async fn himari_debug_get_clan_battle_id_list() -> impl Responder {
    let config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
    GlobalSerde::success(json!({
        "ids": clan_battle::get_clan_battle_id_list(),
        "now_id": config.clanbattle_id,
        "now_lap": config.lap_num
    }))
}
#[options("/himari_debug/setClanBattleConfig")]
async fn himari_debug_set_clan_battle_config_options() -> impl Responder {
    actix_http::Response::NoContent()
}
#[post("/himari_debug/setClanBattleConfig")]
async fn himari_debug_set_clan_battle_config(param: GlobalSerde<SetClanBattleConfigRequest>) -> impl Responder {
    let exists = clan_battle::check_clan_battle_id_exists(param.id) && param.lap > 0 && param.lap < 10000;
    if exists {
        let mut config = clan_battle::CLANBATTLE_CONFIG.lock().unwrap();
        config.clanbattle_id = param.id;
        config.lap_num = param.lap;
        return GlobalSerde::success(json!({
            "now_id": param.id,
            "now_lap": param.lap,
            "msg": "修改成功"
        }));
    }
    GlobalSerde::success(json!({
        "msg": "修改失败"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    database::init_mysql_connection_pool();

    global_data::init_priconne_server_db();
    HttpServer::new(|| {
        App::new()
        .wrap(Logger::default())
        .service(tool_sdk_login)
        .service(check_game_start)
        .service(load_index)
        .service(present_index)
        .service(present_receive)
        .service(present_receive_all)
        .service(home_index)
        .service(dungeon_info)
        .service(story_check)
        .service(story_start)
        .service(shop_item_list)
        .service(shop_buy)
        .service(arena_info)
        .service(arena_search)
        .service(arena_cancel)
        .service(arena_apply)
        .service(arena_start)
        .service(payment_item_list)
        .service(deck_update)
        .service(arena_finish)
        .service(log_battle_log_2)
        .service(deck_update_list)
        .service(arena_history)
        .service(arena_history_detail)
        .service(arena_replay)
        .service(skill_level_up)
        .service(unit_equip)
        .service(unit_automatic_enhance)
        .service(unit_multi_promotion)
        .service(item_exp)
        .service(unit_evolution)
        .service(equipment_enhance)
        .service(unit_craft_equip_unique)
        .service(equipment_enhance_unique)
        .service(equipment_rankup_unique)
        .service(profile_get_profile)
        .service(profile_rename)
        .service(profile_update_comment)
        .service(profile_favorite_unit)
        .service(present_history)
        .service(get_maintenance_status)
        .service(unit_craft_equip)
        .service(clan_battle_top)
        .service(clan_battle_boss_info)
        .service(clan_battle_reload_detail_info)
        .service(clan_battle_support_unit_list)
        .service(clan_battle_rehearsal_finish)
        .service(clan_battle_rehearsal_start)
        .service(clan_battle_update_deck)
        .service(clan_battle_history_report)
        .service(himari_debug_get_unit_list)
        .service(himari_debug_get_arena_log_with_unit_id)
        .service(himari_debug_get_arena_log_with_unit_id_options)
        .service(himari_debug_get_clan_battle_id_list)
        .service(himari_debug_set_clan_battle_config)
        .service(himari_debug_set_clan_battle_config_options)
    }).bind("0.0.0.0:4484")?.run().await
}

// #[test]
// fn decrypt_viewer_id() {
//     let id = "SECRET";
//     println!("{}", cryptor::decrypt_base64_encrypted(id.to_string()));
// }

// #[test]
// fn arena_test() {
//     database::init_mysql_connection_pool();
//     global_data::init_priconne_server_db();
//     let c = database::priconne_get_log_list(0);
//     println!("{}", serde_json::to_string(&c).unwrap());
// }

#[test]
fn rmp_test() {
    let pars = "SECRET";
    let c = base64::decode(pars).unwrap();
    let len = c.len();
    use aes::Aes256;
    use block_modes::{Cbc, block_padding::Pkcs7};
    use block_modes::BlockMode;
    type Aes256Cbc = Cbc<Aes256, Pkcs7>;
    
    let decryptor = Aes256Cbc::new_var(&c[len - 32..len], "ha4nBYA2APUD6Uv1".as_bytes()).unwrap();
    match &decryptor.decrypt_vec(&c[..len - 32]) {
        Ok(_) => {
            let ss : Vec<u8> = vec![];
            let d : serde_json::Value = rmp_serde::from_read_ref(&ss).unwrap();
            println!("{:?}", d);
        },
        Err(_) => panic!("cannot")
    };
}
