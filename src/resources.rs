use bevy::prelude::*;
use crate::classes::{ClassAbility, ShipClass, SkillId};
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub timer: f32,
    pub intensity: f32,
}

#[derive(Resource)]
pub struct GameTimers {
    pub enemy_spawn: Timer,
    pub player_shoot: Timer,
    pub wave_timer: Timer,
    pub boss_spawn_timer: Timer,
    pub formation_timer: Timer,
    pub regen_timer: Timer,
}

impl Default for GameTimers {
    fn default() -> Self {
        Self {
            enemy_spawn: Timer::from_seconds(1.1, TimerMode::Repeating),
            player_shoot: Timer::from_seconds(0.20, TimerMode::Repeating),
            wave_timer: Timer::from_seconds(16.0, TimerMode::Repeating),
            boss_spawn_timer: Timer::from_seconds(24.0, TimerMode::Repeating),
            formation_timer: Timer::from_seconds(8.5, TimerMode::Repeating),
            regen_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

#[derive(Resource, Default)]
pub struct SkillDraftOptions {
    pub is_active: bool,
    pub options: Vec<SkillId>,
}

#[derive(Resource)]
pub struct CurrentPlayer {
    pub name: String,
    pub ship_class: ShipClass,
    pub score: u32,
    pub health: f32,
    pub max_health: f32,
    pub shield: f32,
    pub max_shield: f32,
    pub triple_shot_timer: f32,
    pub wave: u32,
    pub time_elapsed: f32,
    pub enemies_killed: u32,
    pub status_message: String,
    pub status_timer: f32,
    pub shield_hit_timer: f32,
    pub hull_hit_timer: f32,
    pub invulnerable_timer: f32,
    pub combo_count: u32,
    pub combo_timer: f32,
    pub combo_multiplier: u32,
    pub max_combo: u32,

    pub experience: u32,
    pub experience_to_next: u32,
    pub class_upgrade_count: u8,
    pub class_ability_levels: [u8; 3],

    pub active_skills: Vec<SkillId>,
    pub passive_skills: Vec<SkillId>,
    pub active_cooldown_1: f32,
    pub active_cooldown_2: f32,
    pub beam_active_timer: f32,
    pub panic_recovery_used: bool,

    pub class_ability_cooldowns: [f32; 3],
    pub damage_reduction_timer: f32,
    pub damage_reduction_mult: f32,
    pub class_fire_rate_timer: f32,
    pub class_fire_rate_mult: f32,
    pub class_damage_timer: f32,
    pub class_damage_mult: f32,
    pub class_speed_timer: f32,
    pub class_speed_mult: f32,
    pub hunter_mark_timer: f32,
    pub hunter_mark_crit_bonus: f32,

    pub fire_rate_mult: f32,
    pub damage_mult: f32,
    pub crit_chance: f32,
    pub speed_mult: f32,
    pub cooldown_reduction: f32,
    pub powerup_drop_mult: f32,
}

impl Default for CurrentPlayer {
    fn default() -> Self {
        Self {
            name: String::new(),
            ship_class: ShipClass::Defensor,
            score: 0,
            health: 160.0,
            max_health: 160.0,
            shield: 90.0,
            max_shield: 90.0,
            triple_shot_timer: 0.0,
            wave: 1,
            time_elapsed: 0.0,
            enemies_killed: 0,
            status_message: String::new(),
            status_timer: 0.0,
            shield_hit_timer: 0.0,
            hull_hit_timer: 0.0,
            invulnerable_timer: 0.0,
            combo_count: 0,
            combo_timer: 0.0,
            combo_multiplier: 1,
            max_combo: 0,

            experience: 0,
            experience_to_next: 8,
            class_upgrade_count: 0,
            class_ability_levels: [0; 3],

            active_skills: Vec::new(),
            passive_skills: Vec::new(),
            active_cooldown_1: 0.0,
            active_cooldown_2: 0.0,
            beam_active_timer: 0.0,
            panic_recovery_used: false,

            class_ability_cooldowns: [0.0; 3],
            damage_reduction_timer: 0.0,
            damage_reduction_mult: 1.0,
            class_fire_rate_timer: 0.0,
            class_fire_rate_mult: 1.0,
            class_damage_timer: 0.0,
            class_damage_mult: 1.0,
            class_speed_timer: 0.0,
            class_speed_mult: 1.0,
            hunter_mark_timer: 0.0,
            hunter_mark_crit_bonus: 0.0,

            fire_rate_mult: 1.0,
            damage_mult: 1.0,
            crit_chance: 0.05,
            speed_mult: 1.0,
            cooldown_reduction: 0.0,
            powerup_drop_mult: 1.0,
        }
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub player_ship: Handle<Image>,
    pub shield_dome: Handle<Image>,
    pub powerup_triple: Handle<Image>,
    pub powerup_shield: Handle<Image>,
    pub powerup_health: Handle<Image>,
    pub powerup_nuke: Handle<Image>,
    pub experience_orb: Handle<Image>,
    pub regular_enemies: Vec<Handle<Image>>,
    pub boss_enemies: Vec<Handle<Image>>,
    pub planets: Vec<Handle<Image>>,
    pub card_defensor: Handle<Image>,
    pub card_mago: Handle<Image>,
    pub card_asesino: Handle<Image>,
    pub card_artillero: Handle<Image>,
    pub ship_defensor: Handle<Image>,
    pub ship_mago: Handle<Image>,
    pub ship_asesino: Handle<Image>,
    pub ship_artillero: Handle<Image>,
    pub rank_s: Handle<Image>,
    pub rank_a: Handle<Image>,
    pub rank_b: Handle<Image>,
    pub rank_c: Handle<Image>,
    pub skill_textures: HashMap<SkillId, Handle<Image>>,
    pub class_ability_textures: HashMap<ClassAbility, Handle<Image>>,
    pub snd_player_laser: Handle<AudioSource>,
    pub snd_player_death: Handle<AudioSource>,
    pub snd_player_damage: Handle<AudioSource>,
    pub snd_enemy_death: Handle<AudioSource>,
    pub snd_boss_death: Handle<AudioSource>,
    pub snd_powerup_pickup: Handle<AudioSource>,
    pub bgm_space: Handle<AudioSource>,
    pub menu_music: Handle<AudioSource>,
    pub game_over_music: Handle<AudioSource>,
}
