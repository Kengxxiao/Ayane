table! {
    player_arena_data (id) {
        id -> Integer,
        viewer_id -> Bigint,
        arena_rank -> Integer,
        battle_num -> Integer,
    }
}

table! {
    player_arena_log (id) {
        id -> Integer,
        battle_token -> Varchar,
        battle_1_viewer_id -> Bigint,
        battle_2_viewer_id -> Bigint,
        user_arena_deck -> Mediumtext,
        vs_user_arena_deck -> Mediumtext,
        versus -> Bool,
        unit_id_1 -> Integer,
        damage_1 -> Integer,
        unit_id_2 -> Integer,
        damage_2 -> Integer,
        unit_id_3 -> Integer,
        damage_3 -> Integer,
        unit_id_4 -> Integer,
        damage_4 -> Integer,
        unit_id_5 -> Integer,
        damage_5 -> Integer,
        unit_id_1_e -> Integer,
        damage_1_e -> Integer,
        unit_id_2_e -> Integer,
        damage_2_e -> Integer,
        unit_id_3_e -> Integer,
        damage_3_e -> Integer,
        unit_id_4_e -> Integer,
        damage_4_e -> Integer,
        unit_id_5_e -> Integer,
        damage_5_e -> Integer,
        seed -> Bigint,
        sts -> Integer,
        log_time -> Bigint,
    }
}

table! {
    player_data (id) {
        id -> Integer,
        viewer_id -> Bigint,
        now_name -> Varchar,
        user_comment -> Varchar,
        now_team_level -> Integer,
        favorite_unit_id -> Integer,
        free_jewel -> Integer,
        paid_jewel -> Integer,
        gold_id_free -> Integer,
        gold_id_pay -> Integer,
        emblem_id -> Integer,
    }
}

table! {
    player_deck_list (id) {
        id -> Integer,
        viewer_id -> Bigint,
        deck_number -> Integer,
        unit_id_1 -> Integer,
        unit_id_2 -> Integer,
        unit_id_3 -> Integer,
        unit_id_4 -> Integer,
        unit_id_5 -> Integer,
    }
}

table! {
    player_equip (id) {
        id -> Integer,
        viewer_id -> Bigint,
        equip_id -> Integer,
        stock -> Integer,
    }
}

table! {
    player_item (id) {
        id -> Integer,
        viewer_id -> Bigint,
        item_id -> Integer,
        stock -> Integer,
    }
}

table! {
    player_present (id) {
        id -> Integer,
        viewer_id -> Bigint,
        receive_status -> Bool,
        reward_type -> Integer,
        reward_id -> Integer,
        reward_count -> Integer,
        reward_rarity -> Integer,
        message_id -> Integer,
        create_time -> Bigint,
    }
}

table! {
    player_story (id) {
        id -> Integer,
        viewer_id -> Bigint,
        story_id -> Integer,
        unlocked -> Bool,
        seen -> Bool,
    }
}

table! {
    player_unit_data (id) {
        id -> Integer,
        viewer_id -> Bigint,
        unit_id -> Integer,
        rarity -> Integer,
        unit_level -> Integer,
        unit_exp -> Integer,
        promotion_level -> Integer,
        ub_level -> Integer,
        ms_level_1 -> Integer,
        ms_level_2 -> Integer,
        ex_level -> Integer,
        e_lv_1 -> Integer,
        e_pt_1 -> Integer,
        e_lv_2 -> Integer,
        e_pt_2 -> Integer,
        e_lv_3 -> Integer,
        e_pt_3 -> Integer,
        e_lv_4 -> Integer,
        e_pt_4 -> Integer,
        e_lv_5 -> Integer,
        e_pt_5 -> Integer,
        e_lv_6 -> Integer,
        e_pt_6 -> Integer,
        ue_level -> Integer,
        ue_rank -> Integer,
        ue_pt -> Integer,
        icon_skin_id -> Integer,
        sd_skin_id -> Integer,
        still_skin_id -> Integer,
        motion_id -> Integer,
        favorite_flag -> Integer,
    }
}

table! {
    user_session (id) {
        id -> Integer,
        viewer_id -> Bigint,
        request_id -> Varchar,
        next_sid -> Varchar,
        short_udid -> Bigint,
    }
}

table! {
    clan_battle_log (id) {
        id -> Integer,
        viewer_id -> Bigint,
        order_num -> Integer,
        lap_num -> Integer,
        clan_battle_id -> Integer,
        enemy_id -> Integer,
        boss_damage -> Integer,
        total_damage -> Integer,
        u_1 -> Integer,
        u_1_rarity -> Integer,
        u_1_promotion -> Integer,
        u_1_damage -> Integer,
        u_2 -> Integer,
        u_2_rarity -> Integer,
        u_2_promotion -> Integer,
        u_2_damage -> Integer,
        u_3 -> Integer,
        u_3_rarity -> Integer,
        u_3_promotion -> Integer,
        u_3_damage -> Integer,
        u_4 -> Integer,
        u_4_rarity -> Integer,
        u_4_promotion -> Integer,
        u_4_damage -> Integer,
        u_5 -> Integer,
        u_5_rarity -> Integer,
        u_5_promotion -> Integer,
        u_5_damage -> Integer,
        log_time -> Bigint,
        battle_log -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    player_arena_data,
    player_arena_log,
    player_data,
    player_deck_list,
    player_equip,
    player_item,
    player_present,
    player_story,
    player_unit_data,
    user_session,
    clan_battle_log
);
