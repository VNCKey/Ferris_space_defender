use bevy::prelude::*;
use crate::classes::ShipClass;
use serde::{Deserialize, Serialize};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct ThrusterParticle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub initial_size: f32,
}

#[derive(Component)]
pub struct FloatingText {
    pub timer: Timer,
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct BackgroundMusic;

#[derive(Component)]
pub struct MenuMusic;

#[derive(Component)]
pub struct GameOverMusic;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerShieldVisual;

#[derive(Component)]
pub struct Laser {
    pub velocity: Vec2,
    pub damage: f32,
    pub is_crit: bool,
}

#[derive(Component)]
pub struct ActiveBeam;

#[derive(Component)]
pub struct EnemyLaser {
    pub velocity: Vec2,
}

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
    pub score_value: u32,
    pub size: f32,
    pub is_boss: bool,
    pub is_enraged: bool,
    pub shoot_timer: f32,
    pub dir_x: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PowerUpType {
    TripleShot,
    ShieldRestore,
    HealthRestore,
    NukeScreen,
}

#[derive(Component)]
pub struct PowerUpItem {
    pub kind: PowerUpType,
    pub speed: f32,
}

#[derive(Component)]
pub struct ExperienceOrb {
    pub value: u32,
    pub speed: f32,
    pub pulse_phase: f32,
}

#[derive(Component)]
pub struct BackgroundPlanet {
    pub speed: f32,
}

#[derive(Component)]
pub struct BackgroundStar {
    pub speed: f32,
}

#[derive(Component)]
pub struct ExplosionParticle {
    pub velocity: Vec2,
    pub lifetime: Timer,
    pub initial_size: f32,
}
