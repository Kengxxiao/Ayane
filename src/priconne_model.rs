use serde::{Deserialize};

#[derive(Deserialize)]
pub struct GameUnitListForArenaSearch {
    pub unit_id_1: i64,
    pub unit_id_2: i64,
    pub unit_id_3: i64,
    pub unit_id_4: i64,
    pub unit_id_5: i64
}

#[derive(Deserialize)]
pub struct EmptyRequest {
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct SdkLoginRequest {
    pub uid: String,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct PresentReceiveRequest {
    pub present_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct StoryRequest {
    pub story_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct BuyRequest {
    pub slot_id: i32,
    pub number: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ArenaStartRequest {
    pub token: String,
    pub battle_viewer_id: i64,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct DeckUpdateRequest {
    pub deck_number: i32,
    pub unit_id_1: i32,
    pub unit_id_2: i32,
    pub unit_id_3: i32,
    pub unit_id_4: i32,
    pub unit_id_5: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ClanBattleDeckUpdateRequest {
    pub unit_id_1: i32,
    pub unit_id_2: i32,
    pub unit_id_3: i32,
    pub unit_id_4: i32,
    pub unit_id_5: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct DeckUpdateList {
    pub deck_number: i32,
    pub unit_list: Vec<i32>
}

#[derive(Deserialize)]
pub struct DeckUpdateListRequest {
    pub deck_list: Vec<DeckUpdateList>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ArenaUnitDamage {
    pub viewer_id: i64,
    pub unit_id: i32,
    pub damage: i32,
    pub rarity: i32
}

#[derive(Deserialize)]
pub struct ArenaUnitHp {
    pub viewer_id: i64,
    pub unit_id: i32,
    pub hp: i32
}

#[derive(Deserialize)]
pub struct ArenaWaveResult {
    pub unit_damage_list: Vec<ArenaUnitDamage>,
    pub unit_hp_list: Vec<ArenaUnitHp>,
    pub wave_num: i32,
    pub remain_time: i32
}

#[derive(Deserialize)]
pub struct ArenaFinishRequest {
    pub battle_id: i32,
    pub arena_wave_result_list: Vec<ArenaWaveResult>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ArenaHistoryDetailRequest {
    pub log_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct SkillLevelUp {
    pub location: i32,
    pub step: i32,
    pub current_level: i32
}

#[derive(Deserialize)]
pub struct SkillLevelUpRequest {
    pub unit_id: i32,
    pub skill_levelup_list: Vec<SkillLevelUp>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct UnitEquipRequest {
    pub unit_id: i32,
    pub equip_slot_num: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct PriconneItem {
    pub item_id: i32,
    pub item_num: i32,
    pub current_num: i32
}

#[derive(Deserialize)]
pub struct PriconneEquipRecipe {
    pub id: i32,
    pub count: i32
}

#[derive(Deserialize)]
pub struct UnitAutomaticEnhanceRequest {
    pub unit_id: i32,
    pub item_list: Vec<PriconneItem>,
    pub equip_recipe_list: Vec<PriconneEquipRecipe>,
    pub equip_slot_num_list: Vec<i32>,
    pub skill_levelup_list: Vec<SkillLevelUp>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct PromotionEquipRecipe {
    pub equip_list: Vec<PriconneEquipRecipe>
}

#[derive(Deserialize)]
pub struct MultiPromotionRequest {
    pub target_promotion_level: i32,
    pub equip_recipe_list: Vec<PromotionEquipRecipe>,
    pub item_list: Vec<PriconneItem>,
    pub unit_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ItemExpRequest {
    pub item_list: Vec<PriconneItem>,
    pub unit_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct UnitEvolutionRequest {
    pub unit_id: i32,
    pub current_unit_rarity: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct EquipmentEnhanceRequest {
    pub unit_id: i32,
    pub equip_slot_num: i32,
    pub item_list: Vec<PriconneEquipRecipe>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct CraftEquipUniqueRequest {
    pub unit_id: i32,
    pub equip_slot_num: i32,
    pub equip_recipe_list: Vec<PriconneEquipRecipe>,
    pub item_recipe_list: Vec<PriconneEquipRecipe>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct EquipmentRankupUniqueRequest {
    pub unit_id: i32,
    pub equip_recipe_list: Vec<PriconneEquipRecipe>,
    pub item_recipe_list: Vec<PriconneEquipRecipe>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct GetProfileRequest {
    pub target_viewer_id: i64
}

#[derive(Deserialize)]
pub struct ProfileRenameRequest {
    pub user_name: String,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ProfileUpdateCommentRequest {
    pub user_comment: String,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ProfileFavoriteUnitRequest {
    pub unit_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct UnitCraftEquipRequest {
    pub unit_id: i32,
    pub equip_slot_num: i32,
    pub equip_recipe_list: Vec<PriconneEquipRecipe>,
    pub item_list: Vec<PriconneItem>,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ClanBattleBossInfoRequest {
    pub order_num: i32,
    pub lap_num: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct ClanBattleRehearsalStartRequest {
    pub order_num: i32,
    pub lap_num: i32,
    pub clan_battle_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct SetClanBattleConfigRequest {
    pub id: i64,
    pub lap: i64
}

#[derive(Deserialize)]
pub struct ClanBattleRehearsalFinishRequest {
    pub viewer_id: String,
    pub order_num: i32,
    pub clan_battle_id: i32,
    pub lap_num: i32,
    pub user_unit: ClanBattleUserUnit,
    pub boss_damage: i32,
    pub total_damage: i32,
    pub battle_log_id: i32
}

#[derive(Deserialize)]
pub struct ClanBattleUserUnit {
    pub unit_damage_list: Vec<ClanBattleRehearsalFinishDamageListRecord>
}

#[derive(Deserialize)]
pub struct ClanBattleRehearsalFinishDamageListRecord {
    pub viewer_id: i64,
    pub unit_id: i32,
    pub rarity: i32,
    pub damage: i32,
}

#[derive(Deserialize)]
pub struct ClanBattlePlayerDamageHistoryResponse {
    pub viewer_id: i64,
    pub enemy_id: i32,
    pub name: i32,
    pub damage: i32,
    pub create_time: i64,
    pub history_id: i32,
    pub order_num: String
}

#[derive(Deserialize)]
pub struct ClanBattleHistoryReportRequest {
    pub history_id: i32,
    pub viewer_id: String
}

#[derive(Deserialize)]
pub struct BattleLog2Request {
    pub battle_log_id: i32,
    pub battle_log: String,
    pub system_id: i32,
    pub viewer_id: String
}

// pub struct ServerErrorBase {
//     pub status: i32,
//     pub title: String,
//     pub message: String
// }

// pub struct ServerError {
//     pub server_error: ServerErrorBase
// }