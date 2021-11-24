# Host: localhost  (Version: 5.7.26)
# Date: 2021-11-25 00:54:31
# Generator: MySQL-Front 5.3  (Build 4.234)

/*!40101 SET NAMES utf8 */;

#
# Structure for table "__diesel_schema_migrations"
#

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`version`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

#
# Structure for table "clan_battle_log"
#

DROP TABLE IF EXISTS `clan_battle_log`;
CREATE TABLE `clan_battle_log` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `order_num` int(11) NOT NULL DEFAULT '0',
  `lap_num` int(11) NOT NULL DEFAULT '1',
  `clan_battle_id` int(11) NOT NULL DEFAULT '1001',
  `enemy_id` int(11) NOT NULL DEFAULT '0',
  `boss_damage` int(11) NOT NULL DEFAULT '0',
  `total_damage` int(11) NOT NULL DEFAULT '0',
  `u_1` int(11) NOT NULL DEFAULT '0',
  `u_1_rarity` int(11) NOT NULL DEFAULT '0',
  `u_1_promotion` int(11) NOT NULL DEFAULT '0',
  `u_1_damage` int(11) NOT NULL DEFAULT '0',
  `u_2` int(11) NOT NULL DEFAULT '0',
  `u_2_rarity` int(11) NOT NULL DEFAULT '0',
  `u_2_promotion` int(11) NOT NULL DEFAULT '0',
  `u_2_damage` int(11) NOT NULL DEFAULT '0',
  `u_3` int(11) NOT NULL DEFAULT '0',
  `u_3_rarity` int(11) NOT NULL DEFAULT '0',
  `u_3_promotion` int(11) NOT NULL DEFAULT '0',
  `u_3_damage` int(11) NOT NULL DEFAULT '0',
  `u_4` int(11) NOT NULL DEFAULT '0',
  `u_4_rarity` int(11) NOT NULL DEFAULT '0',
  `u_4_promotion` int(11) NOT NULL DEFAULT '0',
  `u_4_damage` int(11) NOT NULL DEFAULT '0',
  `u_5` int(11) NOT NULL DEFAULT '0',
  `u_5_rarity` int(11) NOT NULL DEFAULT '0',
  `u_5_promotion` int(11) NOT NULL DEFAULT '0',
  `u_5_damage` int(11) NOT NULL DEFAULT '0',
  `log_time` bigint(20) NOT NULL DEFAULT '0',
  `battle_log` mediumtext NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

#
# Structure for table "player_arena_data"
#

DROP TABLE IF EXISTS `player_arena_data`;
CREATE TABLE `player_arena_data` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `arena_rank` int(11) NOT NULL,
  `battle_num` int(11) NOT NULL DEFAULT '100',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=10 DEFAULT CHARSET=utf8;

#
# Structure for table "player_arena_log"
#

DROP TABLE IF EXISTS `player_arena_log`;
CREATE TABLE `player_arena_log` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `battle_token` varchar(64) NOT NULL,
  `battle_1_viewer_id` bigint(20) NOT NULL,
  `battle_2_viewer_id` bigint(20) NOT NULL,
  `user_arena_deck` mediumtext NOT NULL,
  `vs_user_arena_deck` mediumtext NOT NULL,
  `versus` tinyint(1) NOT NULL DEFAULT '0',
  `unit_id_1` int(11) NOT NULL DEFAULT '0',
  `damage_1` int(11) NOT NULL DEFAULT '0',
  `unit_id_2` int(11) NOT NULL DEFAULT '0',
  `damage_2` int(11) NOT NULL DEFAULT '0',
  `unit_id_3` int(11) NOT NULL DEFAULT '0',
  `damage_3` int(11) NOT NULL DEFAULT '0',
  `unit_id_4` int(11) NOT NULL DEFAULT '0',
  `damage_4` int(11) NOT NULL DEFAULT '0',
  `unit_id_5` int(11) NOT NULL DEFAULT '0',
  `damage_5` int(11) NOT NULL DEFAULT '0',
  `unit_id_1_e` int(11) NOT NULL DEFAULT '0',
  `damage_1_e` int(11) NOT NULL DEFAULT '0',
  `unit_id_2_e` int(11) NOT NULL DEFAULT '0',
  `damage_2_e` int(11) NOT NULL DEFAULT '0',
  `unit_id_3_e` int(11) NOT NULL DEFAULT '0',
  `damage_3_e` int(11) NOT NULL DEFAULT '0',
  `unit_id_4_e` int(11) NOT NULL DEFAULT '0',
  `damage_4_e` int(11) NOT NULL DEFAULT '0',
  `unit_id_5_e` int(11) NOT NULL DEFAULT '0',
  `damage_5_e` int(11) NOT NULL DEFAULT '0',
  `seed` bigint(20) NOT NULL,
  `sts` int(11) NOT NULL DEFAULT '0',
  `log_time` bigint(20) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=5321 DEFAULT CHARSET=utf8;

#
# Structure for table "player_data"
#

DROP TABLE IF EXISTS `player_data`;
CREATE TABLE `player_data` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `now_name` varchar(32) NOT NULL DEFAULT '佑树',
  `user_comment` varchar(32) NOT NULL DEFAULT '请多指教。',
  `now_team_level` int(11) NOT NULL DEFAULT '1',
  `favorite_unit_id` int(11) NOT NULL DEFAULT '100101',
  `free_jewel` int(11) NOT NULL DEFAULT '0',
  `paid_jewel` int(11) NOT NULL DEFAULT '0',
  `gold_id_free` int(11) NOT NULL DEFAULT '0',
  `gold_id_pay` int(11) NOT NULL DEFAULT '0',
  `emblem_id` int(11) NOT NULL DEFAULT '10000001',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=11 DEFAULT CHARSET=utf8;

#
# Structure for table "player_deck_list"
#

DROP TABLE IF EXISTS `player_deck_list`;
CREATE TABLE `player_deck_list` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `deck_number` int(11) NOT NULL,
  `unit_id_1` int(11) NOT NULL DEFAULT '100101',
  `unit_id_2` int(11) NOT NULL DEFAULT '0',
  `unit_id_3` int(11) NOT NULL DEFAULT '0',
  `unit_id_4` int(11) NOT NULL DEFAULT '0',
  `unit_id_5` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=397 DEFAULT CHARSET=utf8;

#
# Structure for table "player_equip"
#

DROP TABLE IF EXISTS `player_equip`;
CREATE TABLE `player_equip` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `equip_id` int(11) NOT NULL,
  `stock` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=5600 DEFAULT CHARSET=utf8;

#
# Structure for table "player_item"
#

DROP TABLE IF EXISTS `player_item`;
CREATE TABLE `player_item` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `item_id` int(11) NOT NULL,
  `stock` int(11) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=473 DEFAULT CHARSET=utf8;

#
# Structure for table "player_present"
#

DROP TABLE IF EXISTS `player_present`;
CREATE TABLE `player_present` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `receive_status` tinyint(1) NOT NULL DEFAULT '0',
  `reward_type` int(11) NOT NULL,
  `reward_id` int(11) NOT NULL,
  `reward_count` int(11) NOT NULL,
  `reward_rarity` int(11) NOT NULL DEFAULT '0',
  `message_id` int(11) NOT NULL DEFAULT '9203',
  `create_time` bigint(20) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=37 DEFAULT CHARSET=utf8;

#
# Structure for table "player_story"
#

DROP TABLE IF EXISTS `player_story`;
CREATE TABLE `player_story` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `story_id` int(11) NOT NULL,
  `unlocked` tinyint(1) NOT NULL DEFAULT '1',
  `seen` tinyint(1) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=7944 DEFAULT CHARSET=utf8;

#
# Structure for table "player_unit_data"
#

DROP TABLE IF EXISTS `player_unit_data`;
CREATE TABLE `player_unit_data` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `unit_id` int(11) NOT NULL,
  `rarity` int(11) NOT NULL,
  `unit_level` int(11) NOT NULL DEFAULT '1',
  `unit_exp` int(11) NOT NULL DEFAULT '0',
  `promotion_level` int(11) NOT NULL DEFAULT '1',
  `ub_level` int(11) NOT NULL DEFAULT '1',
  `ms_level_1` int(11) NOT NULL DEFAULT '1',
  `ms_level_2` int(11) NOT NULL DEFAULT '1',
  `ex_level` int(11) NOT NULL DEFAULT '1',
  `e_lv_1` int(11) NOT NULL DEFAULT '-1',
  `e_pt_1` int(11) NOT NULL DEFAULT '0',
  `e_lv_2` int(11) NOT NULL DEFAULT '-1',
  `e_pt_2` int(11) NOT NULL DEFAULT '0',
  `e_lv_3` int(11) NOT NULL DEFAULT '-1',
  `e_pt_3` int(11) NOT NULL DEFAULT '0',
  `e_lv_4` int(11) NOT NULL DEFAULT '-1',
  `e_pt_4` int(11) NOT NULL DEFAULT '0',
  `e_lv_5` int(11) NOT NULL DEFAULT '-1',
  `e_pt_5` int(11) NOT NULL DEFAULT '0',
  `e_lv_6` int(11) NOT NULL DEFAULT '-1',
  `e_pt_6` int(11) NOT NULL DEFAULT '0',
  `ue_level` int(11) NOT NULL DEFAULT '0',
  `ue_rank` int(11) NOT NULL DEFAULT '0',
  `ue_pt` int(11) NOT NULL DEFAULT '0',
  `icon_skin_id` int(11) NOT NULL DEFAULT '0',
  `sd_skin_id` int(11) NOT NULL DEFAULT '0',
  `still_skin_id` int(11) NOT NULL DEFAULT '0',
  `motion_id` int(11) NOT NULL DEFAULT '0',
  `favorite_flag` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=913 DEFAULT CHARSET=utf8;

#
# Structure for table "user_session"
#

DROP TABLE IF EXISTS `user_session`;
CREATE TABLE `user_session` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `viewer_id` bigint(20) NOT NULL,
  `request_id` varchar(40) NOT NULL,
  `next_sid` varchar(32) NOT NULL,
  `short_udid` bigint(20) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=MyISAM AUTO_INCREMENT=13 DEFAULT CHARSET=utf8;
