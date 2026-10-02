use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

// ============================================================================
// Estados del Juego y Leaderboard
// ============================================================================

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    NameInput,
    Playing,
    GameOver,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScoreEntry {
    pub name: String,
    pub score: u32,
    pub wave: u32,
    pub date: String,
}

#[derive(Resource)]
pub struct Leaderboard {
    pub entries: Vec<ScoreEntry>,
}

impl Default for Leaderboard {
    fn default() -> Self {
        Self::load()
    }
}

impl Leaderboard {
    const FILE_PATH: &'static str = "highscores.json";

    pub fn load() -> Self {
        if let Ok(content) = fs::read_to_string(Self::FILE_PATH) {
            if let Ok(entries) = serde_json::from_str::<Vec<ScoreEntry>>(&content) {
                return Self { entries };
            }
        }
        Self {
            entries: vec![
                ScoreEntry {
                    name: "Ferris_Pro".to_string(),
                    score: 4200,
                    wave: 6,
                    date: "Record".to_string(),
                },
                ScoreEntry {
                    name: "Rustacean_PE".to_string(),
                    score: 2800,
                    wave: 4,
                    date: "Top 2".to_string(),
                },
            ],
        }
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            let _ = fs::write(Self::FILE_PATH, json);
        }
    }

    pub fn add_score(&mut self, name: String, score: u32, wave: u32) {
        let entry = ScoreEntry {
            name: if name.trim().is_empty() {
                "Anonimo".to_string()
            } else {
                name.trim().to_string()
            },
            score,
            wave,
            date: "Hoy".to_string(),
        };
        self.entries.push(entry);
        self.entries.sort_by(|a, b| b.score.cmp(&a.score));
        if self.entries.len() > 15 {
            self.entries.truncate(15);
        }
        self.save();
    }
}

// ============================================================================
// Clases de Naves Jugables (4 Arquetipos)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipClass {
    Defensor,
    Mago,
    Asesino,
    Artillero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClassAbility {
    BastionProtocol,
    KineticPulse,
    HeavyOverdrive,
    ArcaneNova,
    ZeroCostBeam,
    AetherShield,
    PhaseShift,
    TwinFang,
    HunterMark,
    MissileSalvo,
    CargoBombardment,
    FortifyArmor,
}

impl ClassAbility {
    pub fn name(&self) -> &'static str {
        match self {
            ClassAbility::BastionProtocol => "BASTIÓN",
            ClassAbility::KineticPulse => "PULSO CINÉTICO",
            ClassAbility::HeavyOverdrive => "SOBRECARGA",
            ClassAbility::ArcaneNova => "NOVA ARCANA",
            ClassAbility::ZeroCostBeam => "RAYO ZERO-COST",
            ClassAbility::AetherShield => "ESCUDO ÉTER",
            ClassAbility::PhaseShift => "FASE ASÍNCRONA",
            ClassAbility::TwinFang => "COLMILLO DOBLE",
            ClassAbility::HunterMark => "MARCA DEL CAZADOR",
            ClassAbility::MissileSalvo => "SALVA DE MISILES",
            ClassAbility::CargoBombardment => "BOMBARDEO CARGO",
            ClassAbility::FortifyArmor => "BLINDAJE CARGO",
        }
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            ClassAbility::BastionProtocol => 16.0,
            ClassAbility::KineticPulse => 22.0,
            ClassAbility::HeavyOverdrive => 18.0,
            ClassAbility::ArcaneNova => 16.0,
            ClassAbility::ZeroCostBeam => 25.0,
            ClassAbility::AetherShield => 20.0,
            ClassAbility::PhaseShift => 14.0,
            ClassAbility::TwinFang => 17.0,
            ClassAbility::HunterMark => 19.0,
            ClassAbility::MissileSalvo => 15.0,
            ClassAbility::CargoBombardment => 23.0,
            ClassAbility::FortifyArmor => 18.0,
        }
    }

    pub fn cooldown_for_level(&self, level: u8) -> f32 {
        let reduction = 1.0 - (level.min(3) as f32 * 0.10);
        self.cooldown() * reduction
    }

    pub fn description(&self) -> &'static str {
        match self {
            ClassAbility::BastionProtocol => {
                "Recarga 65 de escudo y reduce el daño recibido durante 5 s."
            }
            ClassAbility::KineticPulse => {
                "Pulso pesado que limpia enemigos normales y daña al jefe."
            }
            ClassAbility::HeavyOverdrive => "Cañón pesado: +75% cadencia y +25% daño durante 5 s.",
            ClassAbility::ArcaneNova => {
                "Libera una corona de 12 orbes arcanos alrededor de la nave."
            }
            ClassAbility::ZeroCostBeam => "Rayo continuo de plasma durante 3 s.",
            ClassAbility::AetherShield => {
                "Restaura escudo y vuelve invulnerable a la nave durante 1.5 s."
            }
            ClassAbility::PhaseShift => {
                "Desplazamiento de fase: velocidad extrema e invulnerabilidad durante 2 s."
            }
            ClassAbility::TwinFang => {
                "Duplica la cadencia y potencia las dos líneas verdes durante 4 s."
            }
            ClassAbility::HunterMark => "Aumenta mucho la probabilidad de críticos durante 6 s.",
            ClassAbility::MissileSalvo => "Lanza 5 misiles rojos en abanico con daño masivo.",
            ClassAbility::CargoBombardment => {
                "Bombardeo EMP que elimina enemigos y proyectiles enemigos."
            }
            ClassAbility::FortifyArmor => {
                "Repara casco, recarga escudo y reduce el daño durante 4 s."
            }
        }
    }

    pub fn color_rgb(&self) -> [u8; 3] {
        match self {
            ClassAbility::BastionProtocol
            | ClassAbility::KineticPulse
            | ClassAbility::HeavyOverdrive => [255, 190, 35],
            ClassAbility::ArcaneNova | ClassAbility::ZeroCostBeam | ClassAbility::AetherShield => {
                [55, 210, 255]
            }
            ClassAbility::PhaseShift | ClassAbility::TwinFang | ClassAbility::HunterMark => {
                [65, 255, 125]
            }
            ClassAbility::MissileSalvo
            | ClassAbility::CargoBombardment
            | ClassAbility::FortifyArmor => [255, 90, 50],
        }
    }

    pub fn texture_key(&self) -> &'static str {
        match self {
            ClassAbility::BastionProtocol => "bastion_protocol",
            ClassAbility::KineticPulse => "kinetic_pulse",
            ClassAbility::HeavyOverdrive => "heavy_overdrive",
            ClassAbility::ArcaneNova => "arcane_nova",
            ClassAbility::ZeroCostBeam => "zero_cost_beam",
            ClassAbility::AetherShield => "aether_shield",
            ClassAbility::PhaseShift => "phase_shift",
            ClassAbility::TwinFang => "twin_fang",
            ClassAbility::HunterMark => "hunter_mark",
            ClassAbility::MissileSalvo => "missile_salvo",
            ClassAbility::CargoBombardment => "cargo_bombardment",
            ClassAbility::FortifyArmor => "fortify_armor",
        }
    }
}

impl ShipClass {
    pub fn name(&self) -> &'static str {
        match self {
            ShipClass::Defensor => "MUTEX TITAN (DEFENSOR)",
            ShipClass::Mago => "ZERO-COST TECNOMANTE (MAGO)",
            ShipClass::Asesino => "TOKIO ASYNC (ASESINO)",
            ShipClass::Artillero => "CARGO BUSTER (ARTILLERO)",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            ShipClass::Defensor => "Defensor",
            ShipClass::Mago => "Mago",
            ShipClass::Asesino => "Asesino",
            ShipClass::Artillero => "Artillero",
        }
    }

    pub fn badge(&self) -> &'static str {
        match self {
            ShipClass::Defensor => "DEFENSOR",
            ShipClass::Mago => "MAGO",
            ShipClass::Asesino => "ASESINO",
            ShipClass::Artillero => "ARTILLERO",
        }
    }

    pub fn class_abilities(&self) -> [ClassAbility; 3] {
        match self {
            ShipClass::Defensor => [
                ClassAbility::BastionProtocol,
                ClassAbility::KineticPulse,
                ClassAbility::HeavyOverdrive,
            ],
            ShipClass::Mago => [
                ClassAbility::ArcaneNova,
                ClassAbility::ZeroCostBeam,
                ClassAbility::AetherShield,
            ],
            ShipClass::Asesino => [
                ClassAbility::PhaseShift,
                ClassAbility::TwinFang,
                ClassAbility::HunterMark,
            ],
            ShipClass::Artillero => [
                ClassAbility::MissileSalvo,
                ClassAbility::CargoBombardment,
                ClassAbility::FortifyArmor,
            ],
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            ShipClass::Defensor => "Blindaje pesado de Titanio. HP: 160 | Escudo: 90 | Velocidad: 270 px/s. Cañón de impacto pesado.",
            ShipClass::Mago => "Tecnomante de energía. HP: 100 | Escudo: 60 | Velocidad: 350 px/s. Orbes de plasma místico.",
            ShipClass::Asesino => "Sombra de alta velocidad. HP: 80 | Escudo: 40 | Velocidad: 440 px/s. Cadencia extrema dual.",
            ShipClass::Artillero => "Destructor pesado en abanico. HP: 120 | Escudo: 50 | Velocidad: 310 px/s. Ráfaga triple de misiles.",
        }
    }

    pub fn max_health(&self) -> f32 {
        match self {
            ShipClass::Defensor => 160.0,
            ShipClass::Mago => 100.0,
            ShipClass::Asesino => 80.0,
            ShipClass::Artillero => 120.0,
        }
    }

    pub fn max_shield(&self) -> f32 {
        match self {
            ShipClass::Defensor => 90.0,
            ShipClass::Mago => 60.0,
            ShipClass::Asesino => 40.0,
            ShipClass::Artillero => 50.0,
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            ShipClass::Defensor => 270.0,
            ShipClass::Mago => 350.0,
            ShipClass::Asesino => 440.0,
            ShipClass::Artillero => 310.0,
        }
    }

    pub fn fire_interval(&self) -> f32 {
        match self {
            ShipClass::Defensor => 0.28,
            ShipClass::Mago => 0.16,
            ShipClass::Asesino => 0.10,
            ShipClass::Artillero => 0.22,
        }
    }

    pub fn laser_damage(&self) -> f32 {
        match self {
            ShipClass::Defensor => 55.0,
            ShipClass::Mago => 32.0,
            ShipClass::Asesino => 18.0,
            ShipClass::Artillero => 35.0,
        }
    }

    pub fn laser_color(&self) -> Color {
        match self {
            ShipClass::Defensor => Color::srgb(1.0, 0.75, 0.15),
            ShipClass::Mago => Color::srgb(0.15, 0.90, 1.0),
            ShipClass::Asesino => Color::srgb(0.25, 1.0, 0.40),
            ShipClass::Artillero => Color::srgb(1.0, 0.30, 0.15),
        }
    }

    pub fn laser_size(&self) -> Vec2 {
        match self {
            ShipClass::Defensor => Vec2::new(10.0, 32.0),
            ShipClass::Mago => Vec2::new(8.0, 28.0),
            ShipClass::Asesino => Vec2::new(5.5, 22.0),
            ShipClass::Artillero => Vec2::new(9.0, 30.0),
        }
    }
}

// ============================================================================
// Habilidades Rogue-lite (16 Cartas)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SkillId {
    OverclockMutex,
    AsyncMultithread,
    ZeroCostBeam,
    UnsafeBlock,
    BorrowChecker,
    PatternMatching,
    CargoClean,
    ArcMutex,
    TokioReactor,
    VectorCapacity,
    PanicRecovery,
    MutexOverdrive,
    StaticLifetime,
    MacroRules,
    OptionSome,
    ZeroCostAbstraction,
}

impl SkillId {
    pub fn name(&self) -> &'static str {
        match self {
            SkillId::OverclockMutex => "Overclock Mutex",
            SkillId::AsyncMultithread => "Async Multithread",
            SkillId::ZeroCostBeam => "Zero-Cost Beam",
            SkillId::UnsafeBlock => "Unsafe Block",
            SkillId::BorrowChecker => "Borrow Checker",
            SkillId::PatternMatching => "Pattern Matching",
            SkillId::CargoClean => "Cargo Clean",
            SkillId::ArcMutex => "Arc<Mutex>",
            SkillId::TokioReactor => "Tokio Reactor",
            SkillId::VectorCapacity => "Vector Capacity",
            SkillId::PanicRecovery => "Panic Recovery",
            SkillId::MutexOverdrive => "Mutex Overdrive",
            SkillId::StaticLifetime => "Static Lifetime",
            SkillId::MacroRules => "Macro_Rules!",
            SkillId::OptionSome => "Option::Some",
            SkillId::ZeroCostAbstraction => "Zero-Cost Abstraction",
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self, SkillId::ZeroCostBeam | SkillId::CargoClean)
    }

    pub fn cooldown(&self) -> f32 {
        match self {
            SkillId::ZeroCostBeam => 25.0,
            SkillId::CargoClean => 18.0,
            _ => 0.0,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            SkillId::OverclockMutex => "PASIVA: +25% Cadencia de Fuego del Láser.",
            SkillId::AsyncMultithread => {
                "PASIVA: +20% Velocidad de Proyectil y +1 Línea Láser adicional."
            }
            SkillId::ZeroCostBeam => {
                "ACTIVABLE (BOTON): Emite un rayo de plasma devastador continuo por 3s."
            }
            SkillId::UnsafeBlock => {
                "PASIVA: +40% Daño de Disparo pero reduce Salud Máxima en -15%."
            }
            SkillId::BorrowChecker => {
                "PASIVA: Refleja un 25% del daño recibido hacia los enemigos cercanos."
            }
            SkillId::PatternMatching => "PASIVA: +15% Probabilidad de Golpe Crítico (3x daño).",
            SkillId::CargoClean => {
                "ACTIVABLE (BOTON): Onda expansiva EMP que borra disparos y enemigos débiles."
            }
            SkillId::ArcMutex => "PASIVA: Genera un mini-escudo orbitatorio que bloquea disparos.",
            SkillId::TokioReactor => "PASIVA: +20% Velocidad de movimiento de la nave.",
            SkillId::VectorCapacity => "PASIVA: +50 Puntos de Escudo Máximo.",
            SkillId::PanicRecovery => {
                "PASIVA: Invulnerabilidad de 2.5s al recibir daño mortal (1 vez)."
            }
            SkillId::MutexOverdrive => {
                "PASIVA: +30% Daño total cuando el escudo está agotado (0 HP)."
            }
            SkillId::StaticLifetime => "PASIVA: Regenera 1% de Salud del casco por segundo.",
            SkillId::MacroRules => {
                "PASIVA: Incrementa el área de impacto y tamaño de los proyectiles."
            }
            SkillId::OptionSome => "PASIVA: +35% Frecuencia de caída de Íconos Power-Up.",
            SkillId::ZeroCostAbstraction => {
                "PASIVA: Reduce los tiempos de recarga de habilidades en -20%."
            }
        }
    }

    pub fn texture_key(&self) -> &'static str {
        match self {
            SkillId::OverclockMutex => "overclock_mutex",
            SkillId::AsyncMultithread => "async_multithread",
            SkillId::ZeroCostBeam => "zero_cost_beam",
            SkillId::UnsafeBlock => "unsafe_block",
            SkillId::BorrowChecker => "borrow_checker",
            SkillId::PatternMatching => "pattern_matching",
            SkillId::CargoClean => "cargo_clean",
            SkillId::ArcMutex => "arc_mutex",
            SkillId::TokioReactor => "tokio_reactor",
            SkillId::VectorCapacity => "vector_capacity",
            SkillId::PanicRecovery => "panic_recovery",
            SkillId::MutexOverdrive => "mutex_overdrive",
            SkillId::StaticLifetime => "static_lifetime",
            SkillId::MacroRules => "macro_rules",
            SkillId::OptionSome => "option_some",
            SkillId::ZeroCostAbstraction => "zero_cost_abstraction",
        }
    }
}

#[derive(Resource, Default)]
pub struct SkillDraftOptions {
    pub is_active: bool,
    pub options: Vec<SkillId>,
}

// ============================================================================
// Estado del Jugador Actual
// ============================================================================

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

    // Experiencia y niveles independientes de las 3 habilidades de clase
    pub experience: u32,
    pub experience_to_next: u32,
    pub class_upgrade_count: u8,
    pub class_ability_levels: [u8; 3],

    // Habilidades Equipadas
    pub active_skills: Vec<SkillId>,
    pub passive_skills: Vec<SkillId>,
    pub active_cooldown_1: f32,
    pub active_cooldown_2: f32,
    pub beam_active_timer: f32,
    pub panic_recovery_used: bool,

    // Habilidades propias de la clase seleccionada
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

    // Multiplicadores
    pub fire_rate_mult: f32,
    pub damage_mult: f32,
    pub crit_chance: f32,
    pub speed_mult: f32,
    pub cooldown_reduction: f32,
    pub powerup_drop_mult: f32,
}

fn class_ability_upgrade_cost(level: u8) -> u32 {
    match level.min(3) {
        0 => 8,
        1 => 16,
        2 => 24,
        _ => u32::MAX,
    }
}

fn refresh_experience_target(current_player: &mut CurrentPlayer) {
    current_player.experience_to_next = current_player
        .class_ability_levels
        .iter()
        .filter(|&&level| level < 3)
        .map(|&level| class_ability_upgrade_cost(level))
        .min()
        .unwrap_or(0);
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
            crit_chance: 0.0,
            speed_mult: 1.0,
            cooldown_reduction: 1.0,
            powerup_drop_mult: 1.0,
        }
    }
}

// ============================================================================
// Tipos de Habilidades / PowerUps
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpType {
    TripleShot,
    Shield,
    Health,
    Nuke,
}

// ============================================================================
// Texturas y Recursos del Juego
// ============================================================================

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

    // Texturas de Clases
    pub card_defensor: Handle<Image>,
    pub card_mago: Handle<Image>,
    pub card_asesino: Handle<Image>,
    pub card_artillero: Handle<Image>,
    pub ship_defensor: Handle<Image>,
    pub ship_mago: Handle<Image>,
    pub ship_asesino: Handle<Image>,
    pub ship_artillero: Handle<Image>,

    // Medallas de Rango
    pub rank_s: Handle<Image>,
    pub rank_a: Handle<Image>,
    pub rank_b: Handle<Image>,
    pub rank_c: Handle<Image>,

    // Cartas de Habilidades
    pub skill_textures: HashMap<SkillId, Handle<Image>>,
    // Iconos de habilidades propias de cada clase
    pub class_ability_textures: HashMap<ClassAbility, Handle<Image>>,

    // Efectos de Sonido
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

// ============================================================================
// Componentes Bevy
// ============================================================================

#[derive(Component)]
pub struct MainCamera;

#[derive(Resource, Default)]
pub struct ScreenShake {
    pub timer: f32,
    pub intensity: f32,
}

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

fn play_sound(commands: &mut Commands, source: Handle<AudioSource>, volume: f32) {
    commands.spawn(AudioBundle {
        source,
        settings: PlaybackSettings::DESPAWN.with_volume(bevy::audio::Volume::new(volume)),
    });
}

#[cfg(target_os = "android")]
pub fn trigger_vibration(duration_ms: i64) {
    if let Some(app) = bevy::winit::ANDROID_APP.get() {
        let vm = unsafe {
            match jni::JavaVM::from_raw(app.vm_as_ptr().cast()) {
                Ok(vm) => vm,
                Err(_) => return,
            }
        };
        let mut env = match vm.attach_current_thread() {
            Ok(env) => env,
            Err(_) => return,
        };
        let activity_ptr = app.activity_as_ptr() as jni::sys::jobject;
        if activity_ptr.is_null() {
            return;
        }
        let activity = unsafe { jni::objects::JObject::from_raw(activity_ptr) };

        let vibrator_str = match env.new_string("vibrator") {
            Ok(s) => s,
            Err(_) => return,
        };

        let vibrator = match env.call_method(
            &activity,
            "getSystemService",
            "(Ljava/lang/String;)Ljava/lang/Object;",
            &[jni::objects::JValue::Object(vibrator_str.as_ref())],
        ) {
            Ok(val) => match val.l() {
                Ok(obj) => obj,
                Err(_) => return,
            },
            Err(_) => return,
        };

        if vibrator.as_raw().is_null() {
            return;
        }

        let _ = env.call_method(
            &vibrator,
            "vibrate",
            "(J)V",
            &[jni::objects::JValue::Long(duration_ms)],
        );
    }
}

#[cfg(not(target_os = "android"))]
pub fn trigger_vibration(_duration_ms: i64) {}

// ============================================================================
// Inicio y Setup Principal
// ============================================================================

#[bevy_main]
pub fn main() {
    run();
}

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ferris Space Defender - Rust Peru".to_string(),
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_state::<AppState>()
        .init_resource::<Leaderboard>()
        .init_resource::<CurrentPlayer>()
        .init_resource::<GameTimers>()
        .init_resource::<ScreenShake>()
        .init_resource::<SkillDraftOptions>()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_app)
        .add_systems(Update, menu_world_visibility)
        .add_systems(Update, ui_name_input.run_if(in_state(AppState::NameInput)))
        .add_systems(Update, ui_game_over.run_if(in_state(AppState::GameOver)))
        // El estado inicial puede entrar a NameInput antes de que Startup haya
        // insertado GameAssets. Ejecutamos también después de setup_app y el
        // sistema evita duplicar la música si ambos eventos coinciden.
        .add_systems(Startup, setup_menu_music.after(setup_app))
        .add_systems(OnEnter(AppState::NameInput), setup_menu_music)
        .add_systems(OnExit(AppState::NameInput), cleanup_menu_music)
        .add_systems(OnEnter(AppState::Playing), setup_playing)
        .add_systems(
            Update,
            (
                player_input_system,
                player_shoot_system,
                thruster_particle_system,
                camera_shake_system,
                floating_text_system,
                combo_system,
                laser_movement_system,
                enemy_laser_system,
                enemy_spawn_system,
                boss_spawn_system,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(
            Update,
            (
                enemy_movement_system,
                powerup_system,
                experience_orb_system,
                collision_system,
                particle_system,
                planet_system,
                star_system,
                difficulty_and_wave_system,
                shield_visual_system,
                passive_skills_system,
                class_ability_touch_system,
                ui_playing_hud,
                ui_skill_draft,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnEnter(AppState::GameOver), setup_game_over_music)
        .add_systems(OnExit(AppState::GameOver), cleanup_game_over_music)
        .add_systems(OnExit(AppState::Playing), cleanup_playing)
        .run();
}

fn setup_app(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(target_os = "android")] mut egui_settings: ResMut<bevy_egui::EguiSettings>,
) {
    #[cfg(target_os = "android")]
    {
        egui_settings.scale_factor = 1.0;
    }

    commands.spawn((Camera2dBundle::default(), MainCamera));

    let player_ship = asset_server.load("textures/player/ship.png");
    let shield_dome = asset_server.load("textures/effects/shield_dome.png");
    let powerup_triple = asset_server.load("textures/powerups/triple.png");
    let powerup_shield = asset_server.load("textures/powerups/shield.png");
    let powerup_health = asset_server.load("textures/powerups/health.png");
    let powerup_nuke = asset_server.load("textures/powerups/nuke.png");
    let experience_orb = asset_server.load("textures/effects/experience_orb.png");

    // Texturas de Clases
    let card_defensor = asset_server.load("textures/classes/card_defensor.png");
    let card_mago = asset_server.load("textures/classes/card_mago.png");
    let card_asesino = asset_server.load("textures/classes/card_asesino.png");
    let card_artillero = asset_server.load("textures/classes/card_artillero.png");

    let ship_defensor = asset_server.load("textures/classes/ship_defensor.png");
    let ship_mago = asset_server.load("textures/classes/ship_mago.png");
    let ship_asesino = asset_server.load("textures/classes/ship_asesino.png");
    let ship_artillero = asset_server.load("textures/classes/ship_artillero.png");

    // Medallas de Rango
    let rank_s = asset_server.load("textures/ranks/rango_s.png");
    let rank_a = asset_server.load("textures/ranks/rango_a.png");
    let rank_b = asset_server.load("textures/ranks/rango_b.png");
    let rank_c = asset_server.load("textures/ranks/rango_c.png");

    // Cargar 16 Cartas de Habilidades
    let all_skills = [
        SkillId::OverclockMutex,
        SkillId::AsyncMultithread,
        SkillId::ZeroCostBeam,
        SkillId::UnsafeBlock,
        SkillId::BorrowChecker,
        SkillId::PatternMatching,
        SkillId::CargoClean,
        SkillId::ArcMutex,
        SkillId::TokioReactor,
        SkillId::VectorCapacity,
        SkillId::PanicRecovery,
        SkillId::MutexOverdrive,
        SkillId::StaticLifetime,
        SkillId::MacroRules,
        SkillId::OptionSome,
        SkillId::ZeroCostAbstraction,
    ];
    let mut skill_textures = HashMap::new();
    for skill in all_skills {
        let handle = asset_server.load(format!("textures/skills/{}.png", skill.texture_key()));
        skill_textures.insert(skill, handle);
    }

    let all_class_abilities = [
        ClassAbility::BastionProtocol,
        ClassAbility::KineticPulse,
        ClassAbility::HeavyOverdrive,
        ClassAbility::ArcaneNova,
        ClassAbility::ZeroCostBeam,
        ClassAbility::AetherShield,
        ClassAbility::PhaseShift,
        ClassAbility::TwinFang,
        ClassAbility::HunterMark,
        ClassAbility::MissileSalvo,
        ClassAbility::CargoBombardment,
        ClassAbility::FortifyArmor,
    ];
    let mut class_ability_textures = HashMap::new();
    for ability in all_class_abilities {
        let handle = asset_server.load(format!(
            "textures/class_abilities/{}.png",
            ability.texture_key()
        ));
        class_ability_textures.insert(ability, handle);
    }

    // Efectos de Sonido
    let snd_player_laser = asset_server.load("audio/laser_player.ogg");
    let snd_player_death = asset_server.load("audio/player_death.ogg");
    let snd_player_damage = asset_server.load("audio/player_damage.ogg");
    let snd_enemy_death = asset_server.load("audio/enemy_death.ogg");
    let snd_boss_death = asset_server.load("audio/boss_death.ogg");
    let snd_powerup_pickup = asset_server.load("audio/powerup_pickup.ogg");
    let bgm_space = asset_server.load("audio/bgm_space.ogg");
    let menu_music = asset_server.load("audio/menu_flags.ogg");
    let game_over_music = asset_server.load("audio/game_over_mystical.ogg");

    // 7 Enemigos regulares
    let mut regular_enemies = Vec::new();
    for i in 1..=7 {
        regular_enemies.push(asset_server.load(format!("textures/enemies/enemy_{}.png", i)));
    }

    // 3 Jefes Nodriza Gigantes
    let mut boss_enemies = Vec::new();
    for i in 8..=10 {
        boss_enemies.push(asset_server.load(format!("textures/enemies/enemy_{}.png", i)));
    }

    // 3 Planetas espaciales
    let mut planets = Vec::new();
    for i in 1..=3 {
        planets.push(asset_server.load(format!("textures/backgrounds/planet_{}.png", i)));
    }

    commands.insert_resource(GameAssets {
        player_ship,
        shield_dome,
        powerup_triple,
        powerup_shield,
        powerup_health,
        powerup_nuke,
        experience_orb,
        regular_enemies,
        boss_enemies,
        planets,
        card_defensor,
        card_mago,
        card_asesino,
        card_artillero,
        ship_defensor,
        ship_mago,
        ship_asesino,
        ship_artillero,
        rank_s,
        rank_a,
        rank_b,
        rank_c,
        skill_textures,
        class_ability_textures,
        snd_player_laser,
        snd_player_death,
        snd_player_damage,
        snd_enemy_death,
        snd_boss_death,
        snd_powerup_pickup,
        bgm_space,
        menu_music,
        game_over_music,
    });

    // Campo de estrellas cósmicas
    let mut rng = rand::thread_rng();
    for _ in 0..130 {
        let x = rng.gen_range(-380.0..380.0);
        let y = rng.gen_range(-650.0..650.0);
        let tier = rng.gen_range(0..10);
        let (speed, size, alpha) = if tier < 6 {
            (
                rng.gen_range(16.0..35.0),
                rng.gen_range(1.2..2.0),
                rng.gen_range(0.35..0.65),
            )
        } else if tier < 9 {
            (
                rng.gen_range(40.0..75.0),
                rng.gen_range(2.0..2.8),
                rng.gen_range(0.70..0.90),
            )
        } else {
            (
                rng.gen_range(85.0..140.0),
                rng.gen_range(2.8..4.0),
                rng.gen_range(0.85..1.0),
            )
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.88, 0.94, 1.0, alpha),
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                // El campo de nombre se dibuja con egui. Mantener el mundo 2D
                // oculto en los menús evita que Android muestre un frame de la
                // escena al cambiar el tamaño de la ventana por el teclado.
                visibility: Visibility::Hidden,
                transform: Transform::from_xyz(x, y, -20.0),
                ..default()
            },
            BackgroundStar { speed },
        ));
    }
}

fn menu_world_visibility(
    state: Res<State<AppState>>,
    mut visibility_queries: ParamSet<(
        Query<&mut Visibility, With<BackgroundStar>>,
        Query<&mut Visibility, With<BackgroundPlanet>>,
    )>,
) {
    let show_world = *state.get() == AppState::Playing;
    let visibility = if show_world {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    for mut star_visibility in visibility_queries.p0().iter_mut() {
        *star_visibility = visibility;
    }
    for mut planet_visibility in visibility_queries.p1().iter_mut() {
        *planet_visibility = visibility;
    }
}

// ============================================================================
// Setup Playing
// ============================================================================

fn setup_menu_music(
    mut commands: Commands,
    assets: Option<Res<GameAssets>>,
    existing: Query<(), With<MenuMusic>>,
) {
    if !existing.is_empty() {
        return;
    }

    let Some(assets) = assets else {
        // En el primer OnEnter(NameInput), GameAssets todavía puede no existir.
        // El sistema Startup.after(setup_app) se encargará de crear la música.
        return;
    };

    commands.spawn((
        AudioBundle {
            source: assets.menu_music.clone(),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.58)),
        },
        MenuMusic,
    ));
}

fn cleanup_menu_music(mut commands: Commands, query: Query<Entity, With<MenuMusic>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_game_over_music(mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioBundle {
            source: assets.game_over_music.clone(),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.62)),
        },
        GameOverMusic,
    ));
}

fn cleanup_game_over_music(mut commands: Commands, query: Query<Entity, With<GameOverMusic>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_playing(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut timers: ResMut<GameTimers>,
    mut skill_draft: ResMut<SkillDraftOptions>,
) {
    current_player.max_health = current_player.ship_class.max_health();
    current_player.health = current_player.ship_class.max_health();
    current_player.max_shield = current_player.ship_class.max_shield();
    current_player.shield = current_player.ship_class.max_shield();
    current_player.triple_shot_timer = 0.0;
    current_player.score = 0;
    current_player.wave = 1;
    current_player.time_elapsed = 0.0;
    current_player.enemies_killed = 0;
    current_player.status_message = format!(
        "MISION INICIADA: {}",
        current_player.ship_class.short_name()
    );
    current_player.status_timer = 2.8;
    current_player.invulnerable_timer = 0.0;
    current_player.combo_count = 0;
    current_player.combo_timer = 0.0;
    current_player.combo_multiplier = 1;
    current_player.max_combo = 0;
    current_player.experience = 0;
    current_player.class_upgrade_count = 0;
    current_player.class_ability_levels = [0; 3];
    refresh_experience_target(&mut current_player);
    skill_draft.is_active = false;
    skill_draft.options.clear();
    current_player.active_skills.clear();
    current_player.passive_skills.clear();
    current_player.active_cooldown_1 = 0.0;
    current_player.active_cooldown_2 = 0.0;
    current_player.beam_active_timer = 0.0;
    current_player.panic_recovery_used = false;
    current_player.class_ability_cooldowns = [0.0; 3];
    current_player.damage_reduction_timer = 0.0;
    current_player.damage_reduction_mult = 1.0;
    current_player.class_fire_rate_timer = 0.0;
    current_player.class_fire_rate_mult = 1.0;
    current_player.class_damage_timer = 0.0;
    current_player.class_damage_mult = 1.0;
    current_player.class_speed_timer = 0.0;
    current_player.class_speed_mult = 1.0;
    current_player.hunter_mark_timer = 0.0;
    current_player.hunter_mark_crit_bonus = 0.0;
    current_player.fire_rate_mult = 1.0;
    current_player.damage_mult = 1.0;
    current_player.crit_chance = 0.0;
    current_player.speed_mult = 1.0;
    current_player.cooldown_reduction = 1.0;
    current_player.powerup_drop_mult = 1.0;

    timers.player_shoot = Timer::from_seconds(
        current_player.ship_class.fire_interval(),
        TimerMode::Repeating,
    );
    timers.enemy_spawn.reset();
    timers.wave_timer.reset();
    timers.boss_spawn_timer.reset();
    timers.formation_timer.reset();

    // Música de combate espacial en bucle continuo
    commands.spawn((
        AudioBundle {
            source: assets.bgm_space.clone(),
            settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new(0.82)),
        },
        BackgroundMusic,
    ));

    // Seleccionar textura de nave correspondiente a la clase elegida
    let ship_texture = match current_player.ship_class {
        ShipClass::Defensor => assets.ship_defensor.clone(),
        ShipClass::Mago => assets.ship_mago.clone(),
        ShipClass::Asesino => assets.ship_asesino.clone(),
        ShipClass::Artillero => assets.ship_artillero.clone(),
    };

    // Nave Principal
    commands.spawn((
        SpriteBundle {
            texture: ship_texture,
            sprite: Sprite {
                custom_size: Some(Vec2::new(95.0, 75.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -260.0, 10.0),
            ..default()
        },
        Player,
    ));

    // Escudo de impacto
    commands.spawn((
        SpriteBundle {
            texture: assets.shield_dome.clone(),
            visibility: Visibility::Hidden,
            sprite: Sprite {
                color: Color::srgba(0.5, 0.95, 1.0, 0.9),
                custom_size: Some(Vec2::new(138.0, 118.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -260.0, 9.5),
            ..default()
        },
        PlayerShieldVisual,
    ));

    // Planetas de fondo
    let mut rng = rand::thread_rng();
    for (idx, planet_tex) in assets.planets.iter().enumerate() {
        let x = rng.gen_range(-220.0..220.0);
        let y = 350.0 + (idx as f32) * 450.0;
        let scale = rng.gen_range(180.0..280.0);
        let speed = rng.gen_range(15.0..32.0);

        commands.spawn((
            SpriteBundle {
                texture: planet_tex.clone(),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(scale)),
                    color: Color::srgba(1.0, 1.0, 1.0, 0.75),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -10.0),
                ..default()
            },
            BackgroundPlanet { speed },
        ));
    }
}

fn cleanup_playing(
    mut commands: Commands,
    query_player: Query<Entity, With<Player>>,
    query_shield: Query<Entity, With<PlayerShieldVisual>>,
    query_enemies: Query<Entity, With<Enemy>>,
    query_lasers: Query<Entity, With<Laser>>,
    query_enemy_lasers: Query<Entity, With<EnemyLaser>>,
    query_particles: Query<Entity, With<ExplosionParticle>>,
    query_powerups: Query<Entity, With<PowerUpItem>>,
    query_experience: Query<Entity, With<ExperienceOrb>>,
    query_planets: Query<Entity, With<BackgroundPlanet>>,
    query_bgm: Query<Entity, With<BackgroundMusic>>,
    query_thrusters: Query<Entity, With<ThrusterParticle>>,
    query_floating: Query<Entity, With<FloatingText>>,
    mut screen_shake: ResMut<ScreenShake>,
) {
    screen_shake.timer = 0.0;
    screen_shake.intensity = 0.0;

    for e in query_player.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_shield.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_enemies.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_lasers.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_enemy_lasers.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_particles.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_thrusters.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_floating.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_powerups.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_experience.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_planets.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_bgm.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ============================================================================
// Sistemas de Control y Disparos
// ============================================================================

fn touch_inside_rect(touch: Vec2, left: f32, top: f32, right: f32, bottom: f32) -> bool {
    touch.x >= left && touch.x <= right && touch.y >= top && touch.y <= bottom
}

fn class_ability_index_at(position: Vec2, window: &Window) -> Option<usize> {
    let right = window.width() - 12.0;
    let bottom = window.height() - 78.0;
    let row_left = right - 132.0;
    let top_button_left = row_left + 34.0;

    if touch_inside_rect(
        position,
        top_button_left,
        bottom - 151.0,
        top_button_left + 64.0,
        bottom - 75.0,
    ) {
        Some(0)
    } else if touch_inside_rect(position, row_left, bottom - 76.0, row_left + 64.0, bottom) {
        Some(1)
    } else if touch_inside_rect(position, row_left + 68.0, bottom - 76.0, right, bottom) {
        Some(2)
    } else {
        None
    }
}

fn player_input_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    current_player: Res<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if skill_draft.is_active {
        return;
    }

    if let Ok(mut transform) = query.get_single_mut() {
        let dt = time.delta_seconds();
        let class_speed = if current_player.class_speed_timer > 0.0 {
            current_player.class_speed_mult
        } else {
            1.0
        };
        let speed = current_player.ship_class.speed() * current_player.speed_mult * class_speed;

        if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= speed * dt;
        }
        if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += speed * dt;
        }
        if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y += speed * dt;
        }
        if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= speed * dt;
        }

        if let Ok(window) = windows.get_single() {
            // El dedo que empieza sobre un control pertenece a ese control.
            // Si empezó en el espacio de juego, conserva el control aunque
            // después se arrastre por encima de los botones.
            let touch_starts_on_control = |position: Vec2| {
                let class_ability_button = class_ability_index_at(position, window).is_some();

                let active_skill_width =
                    (current_player.active_skills.len() as f32 * 74.0).min(148.0);
                let active_skill_button = !current_player.active_skills.is_empty()
                    && touch_inside_rect(
                        position,
                        20.0,
                        window.height() - 175.0,
                        20.0 + active_skill_width,
                        window.height() - 85.0,
                    );

                class_ability_button || active_skill_button
            };

            let touch_position = touches
                .iter()
                .find(|touch| !touch_starts_on_control(touch.start_position()))
                .map(|touch| touch.position())
                .or_else(|| {
                    touches
                        .iter_just_pressed()
                        .find(|touch| !touch_starts_on_control(touch.start_position()))
                        .map(|touch| touch.position())
                });

            if let Some(touch) = touch_position {
                let target_x = touch.x - window.width() * 0.5;
                let target_y = window.height() * 0.5 - touch.y + 45.0;
                let lerp_speed = match current_player.ship_class {
                    ShipClass::Asesino => 26.0,
                    ShipClass::Mago => 22.0,
                    ShipClass::Defensor => 16.0,
                    ShipClass::Artillero => 18.0,
                };
                transform.translation.x = transform
                    .translation
                    .x
                    .lerp(target_x, (dt * lerp_speed).min(1.0));
                transform.translation.y = transform
                    .translation
                    .y
                    .lerp(target_y, (dt * lerp_speed).min(1.0));
            }
        }

        transform.translation.x = transform.translation.x.clamp(-260.0, 260.0);
        transform.translation.y = transform.translation.y.clamp(-380.0, 320.0);
    }
}

fn shield_visual_system(
    time: Res<Time>,
    mut current_player: ResMut<CurrentPlayer>,
    mut player_query: Query<(&Transform, &mut Sprite), (With<Player>, Without<PlayerShieldVisual>)>,
    mut shield_query: Query<
        (&mut Transform, &mut Visibility, &mut Sprite),
        With<PlayerShieldVisual>,
    >,
    skill_draft: Res<SkillDraftOptions>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();

    if current_player.shield_hit_timer > 0.0 {
        current_player.shield_hit_timer -= dt;
    }
    if current_player.hull_hit_timer > 0.0 {
        current_player.hull_hit_timer -= dt;
    }
    if current_player.invulnerable_timer > 0.0 {
        current_player.invulnerable_timer -= dt;
    }

    if let Ok((player_tr, mut player_sprite)) = player_query.get_single_mut() {
        if current_player.hull_hit_timer > 0.0 {
            player_sprite.color = Color::srgb(1.0, 0.35, 0.35);
        } else {
            player_sprite.color = Color::WHITE;
        }

        if let Ok((mut shield_tr, mut visibility, mut shield_sprite)) =
            shield_query.get_single_mut()
        {
            shield_tr.translation = player_tr.translation;
            shield_tr.translation.z = 9.5;

            if current_player.shield_hit_timer > 0.0 {
                *visibility = Visibility::Visible;
                let progress = (current_player.shield_hit_timer / 0.28).clamp(0.0, 1.0);
                let scale = 1.0 + (1.0 - progress) * 0.12;
                shield_tr.scale = Vec3::splat(scale);
                shield_sprite.color = Color::srgba(0.5, 0.95, 1.0, progress * 0.95);
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn spawn_laser_projectile(
    commands: &mut Commands,
    position: Vec2,
    velocity: Vec2,
    size: Vec2,
    color: Color,
    damage: f32,
    is_crit: bool,
    is_beam: bool,
) {
    let rotation = Quat::from_rotation_z(-velocity.x.atan2(velocity.y));
    let mut projectile = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 8.0),
                rotation,
                ..default()
            },
            ..default()
        },
        Laser {
            velocity,
            damage,
            is_crit,
        },
    ));

    if is_beam {
        projectile.insert(ActiveBeam);
    }
}

fn player_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
    query: Query<&Transform, With<Player>>,
) {
    if skill_draft.is_active {
        return;
    }

    let fire_rate_bonus = if current_player.class_fire_rate_timer > 0.0 {
        current_player.class_fire_rate_mult
    } else {
        1.0
    };
    let effective_interval = current_player.ship_class.fire_interval()
        / (current_player.fire_rate_mult * fire_rate_bonus);
    timers
        .player_shoot
        .set_duration(std::time::Duration::from_secs_f32(
            effective_interval.max(0.05),
        ));
    timers.player_shoot.tick(time.delta());

    if current_player.triple_shot_timer > 0.0 {
        current_player.triple_shot_timer -= time.delta_seconds();
    }
    if current_player.beam_active_timer > 0.0 {
        current_player.beam_active_timer -= time.delta_seconds();
    }

    if !timers.player_shoot.just_finished() {
        return;
    }

    if let Ok(player_tr) = query.get_single() {
        play_sound(&mut commands, assets.snd_player_laser.clone(), 0.14);
        let position = player_tr.translation.truncate();
        let ship_class = current_player.ship_class;
        let class_damage_bonus = if current_player.class_damage_timer > 0.0 {
            current_player.class_damage_mult
        } else {
            1.0
        };

        let mut damage =
            ship_class.laser_damage() * current_player.damage_mult * class_damage_bonus;
        if current_player
            .passive_skills
            .contains(&SkillId::MutexOverdrive)
            && current_player.shield <= 0.0
        {
            damage *= 1.30;
        }

        let mut rng = rand::thread_rng();
        let crit_chance = current_player.crit_chance
            + if current_player.hunter_mark_timer > 0.0 {
                current_player.hunter_mark_crit_bonus
            } else {
                0.0
            };
        let is_crit = rng.gen_bool(crit_chance.min(0.9) as f64);
        if is_crit {
            damage *= 3.0;
        }

        let color = if is_crit {
            Color::srgb(1.0, 0.9, 0.2)
        } else {
            ship_class.laser_color()
        };
        let mut size = ship_class.laser_size();
        if current_player.passive_skills.contains(&SkillId::MacroRules) {
            size *= 1.30;
        }

        // Cada clase tiene una silueta de disparo propia:
        // Defensor = un cañón pesado, Mago = orbes, Asesino = doble aguja,
        // Artillero = abanico de tres misiles.
        let mut pattern: Vec<(f32, f32, f32)> = match ship_class {
            ShipClass::Defensor => vec![(0.0, 0.0, 1.20)],
            ShipClass::Mago => vec![(-0.045, -10.0, 0.82), (0.045, 10.0, 0.82)],
            ShipClass::Asesino => vec![(-0.015, -17.0, 0.72), (0.015, 17.0, 0.72)],
            ShipClass::Artillero => vec![(-0.18, -16.0, 0.84), (0.0, 0.0, 1.0), (0.18, 16.0, 0.84)],
        };

        if current_player.triple_shot_timer > 0.0
            || current_player
                .passive_skills
                .contains(&SkillId::AsyncMultithread)
        {
            match ship_class {
                ShipClass::Defensor => {
                    pattern.extend([(-0.12, -18.0, 0.72), (0.12, 18.0, 0.72)]);
                }
                ShipClass::Mago | ShipClass::Asesino => {
                    pattern.push((0.0, 0.0, 0.90));
                }
                ShipClass::Artillero => {
                    pattern.extend([(-0.30, -25.0, 0.66), (0.30, 25.0, 0.66)]);
                }
            }
        }

        if current_player.beam_active_timer > 0.0 {
            spawn_laser_projectile(
                &mut commands,
                position + Vec2::new(0.0, 35.0),
                Vec2::new(0.0, 1250.0),
                Vec2::new(30.0, 130.0),
                Color::srgb(0.20, 0.95, 1.0),
                damage * 3.0,
                false,
                true,
            );
            return;
        }

        let projectile_speed = match ship_class {
            ShipClass::Defensor => 760.0,
            ShipClass::Mago => 980.0,
            ShipClass::Asesino => 1120.0,
            ShipClass::Artillero => 780.0,
        };
        for (angle, offset_x, damage_mult) in pattern {
            let velocity = Vec2::new(
                angle.sin() * projectile_speed,
                angle.cos() * projectile_speed,
            );
            spawn_laser_projectile(
                &mut commands,
                position + Vec2::new(offset_x, 24.0),
                velocity,
                size,
                color,
                damage * damage_mult,
                is_crit,
                false,
            );
        }
    }
}

fn passive_skills_system(
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();

    // Actualizar Cooldowns de Habilidades Activas
    if current_player.active_cooldown_1 > 0.0 {
        current_player.active_cooldown_1 = (current_player.active_cooldown_1 - dt).max(0.0);
    }
    if current_player.active_cooldown_2 > 0.0 {
        current_player.active_cooldown_2 = (current_player.active_cooldown_2 - dt).max(0.0);
    }
    for cooldown in &mut current_player.class_ability_cooldowns {
        *cooldown = (*cooldown - dt).max(0.0);
    }
    current_player.damage_reduction_timer = (current_player.damage_reduction_timer - dt).max(0.0);
    current_player.class_fire_rate_timer = (current_player.class_fire_rate_timer - dt).max(0.0);
    current_player.class_damage_timer = (current_player.class_damage_timer - dt).max(0.0);
    current_player.class_speed_timer = (current_player.class_speed_timer - dt).max(0.0);
    current_player.hunter_mark_timer = (current_player.hunter_mark_timer - dt).max(0.0);

    // Static Lifetime: Regenerar 1% HP/s
    if current_player
        .passive_skills
        .contains(&SkillId::StaticLifetime)
    {
        timers.regen_timer.tick(time.delta());
        if timers.regen_timer.just_finished() && current_player.health < current_player.max_health {
            let regen_amount = current_player.max_health * 0.01;
            current_player.health =
                (current_player.health + regen_amount).min(current_player.max_health);
        }
    }
}

fn laser_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    skill_draft: Res<SkillDraftOptions>,
    mut query: Query<(Entity, &mut Transform, &Laser)>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();
    for (entity, mut transform, laser) in query.iter_mut() {
        transform.translation.x += laser.velocity.x * dt;
        transform.translation.y += laser.velocity.y * dt;
        if transform.translation.y > 450.0 || transform.translation.x.abs() > 340.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_laser_system(
    mut commands: Commands,
    time: Res<Time>,
    skill_draft: Res<SkillDraftOptions>,
    mut query: Query<(Entity, &mut Transform, &EnemyLaser)>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();
    for (entity, mut transform, laser) in query.iter_mut() {
        transform.translation.x += laser.velocity.x * dt;
        transform.translation.y += laser.velocity.y * dt;
        if transform.translation.y < -450.0 {
            commands.entity(entity).despawn();
        }
    }
}

// ============================================================================
// Generacion y Movimiento de Enemigos y Jefes
// ============================================================================

fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    assets: Res<GameAssets>,
    current_player: Res<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
) {
    if skill_draft.is_active {
        return;
    }

    timers.enemy_spawn.tick(time.delta());
    if timers.enemy_spawn.just_finished() {
        if assets.regular_enemies.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let enemy_idx = rng.gen_range(0..assets.regular_enemies.len());
        let texture = assets.regular_enemies[enemy_idx].clone();

        let x = rng.gen_range(-240.0..240.0);
        let speed_factor = 1.0 + (current_player.time_elapsed * 0.007).min(1.8);
        let base_speed = rng.gen_range(110.0..185.0) * speed_factor;
        let health = (35.0 + (current_player.wave as f32 * 14.0))
            * (1.0 + current_player.time_elapsed * 0.004);
        let size = rng.gen_range(62.0..82.0);
        let score_val = 100 + (enemy_idx as u32 * 15);

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, 420.0, 5.0),
                ..default()
            },
            Enemy {
                health,
                max_health: health,
                speed: base_speed,
                score_value: score_val,
                size,
                is_boss: false,
                is_enraged: false,
                shoot_timer: rng.gen_range(1.5..3.0),
                dir_x: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            },
        ));
    }

    timers.formation_timer.tick(time.delta());
    if timers.formation_timer.just_finished() && !assets.regular_enemies.is_empty() {
        let mut rng = rand::thread_rng();
        let speed_factor = 1.0 + (current_player.time_elapsed * 0.007).min(1.8);
        let base_speed = 135.0 * speed_factor;
        let health = 30.0 + (current_player.wave as f32 * 12.0);
        let size = 66.0;

        if rng.gen_bool(0.5) {
            let v_offsets = [
                (0.0, 430.0),
                (-55.0, 475.0),
                (55.0, 475.0),
                (-110.0, 520.0),
                (110.0, 520.0),
            ];
            let tex =
                assets.regular_enemies[rng.gen_range(0..assets.regular_enemies.len())].clone();
            for (off_x, spawn_y) in v_offsets {
                commands.spawn((
                    SpriteBundle {
                        texture: tex.clone(),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(size)),
                            ..default()
                        },
                        transform: Transform::from_xyz(off_x, spawn_y, 5.0),
                        ..default()
                    },
                    Enemy {
                        health,
                        max_health: health,
                        speed: base_speed,
                        score_value: 120,
                        size,
                        is_boss: false,
                        is_enraged: false,
                        shoot_timer: rng.gen_range(2.0..4.0),
                        dir_x: 0.0,
                    },
                ));
            }
        }
    }
}

fn boss_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
    query_bosses: Query<&Enemy>,
) {
    if skill_draft.is_active {
        return;
    }

    timers.boss_spawn_timer.tick(time.delta());

    let has_boss = query_bosses.iter().any(|e| e.is_boss);
    if timers.boss_spawn_timer.just_finished() && !has_boss && !assets.boss_enemies.is_empty() {
        let mut rng = rand::thread_rng();
        let boss_idx = rng.gen_range(0..assets.boss_enemies.len());
        let texture = assets.boss_enemies[boss_idx].clone();

        let boss_hp = 850.0 + (current_player.wave as f32 * 450.0);
        current_player.status_message =
            "¡ALERTA DE AMENAZA! APARECE JEFE BORROW CHECKER".to_string();
        current_player.status_timer = 4.0;
        trigger_vibration(300);

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(175.0, 145.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 420.0, 6.0),
                ..default()
            },
            Enemy {
                health: boss_hp,
                max_health: boss_hp,
                speed: 65.0,
                score_value: 1500,
                size: 145.0,
                is_boss: true,
                is_enraged: false,
                shoot_timer: 1.2,
                dir_x: 1.0,
            },
        ));
    }
}

fn enemy_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    current_player: Res<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
    mut query: Query<(Entity, &mut Transform, &mut Enemy)>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();
    let mut rng = rand::thread_rng();

    for (entity, mut transform, mut enemy) in query.iter_mut() {
        if enemy.is_boss {
            let target_y = 230.0;
            if transform.translation.y > target_y {
                transform.translation.y -= enemy.speed * dt;
            } else {
                let speed_x = if enemy.is_enraged { 170.0 } else { 95.0 };
                transform.translation.x += enemy.dir_x * speed_x * dt;

                if transform.translation.x > 210.0 {
                    transform.translation.x = 210.0;
                    enemy.dir_x = -1.0;
                } else if transform.translation.x < -210.0 {
                    transform.translation.x = -210.0;
                    enemy.dir_x = 1.0;
                }
            }

            enemy.shoot_timer -= dt;
            if enemy.shoot_timer <= 0.0 {
                enemy.shoot_timer = if enemy.is_enraged { 0.7 } else { 1.3 };
                let pos = transform.translation;

                if enemy.is_enraged {
                    for &angle in &[-0.4_f32, -0.2, 0.0, 0.2, 0.4] {
                        let vx = angle.sin() * 320.0;
                        let vy = -angle.cos() * 320.0;
                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::srgb(1.0, 0.15, 0.15),
                                    custom_size: Some(Vec2::new(9.0, 24.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(pos.x, pos.y - 40.0, 7.0),
                                ..default()
                            },
                            EnemyLaser {
                                velocity: Vec2::new(vx, vy),
                            },
                        ));
                    }
                } else {
                    for offset_x in &[-35.0, 35.0] {
                        commands.spawn((
                            SpriteBundle {
                                sprite: Sprite {
                                    color: Color::srgb(1.0, 0.4, 0.1),
                                    custom_size: Some(Vec2::new(8.0, 22.0)),
                                    ..default()
                                },
                                transform: Transform::from_xyz(pos.x + offset_x, pos.y - 40.0, 7.0),
                                ..default()
                            },
                            EnemyLaser {
                                velocity: Vec2::new(0.0, -360.0),
                            },
                        ));
                    }
                }
            }
        } else {
            transform.translation.y -= enemy.speed * dt;
            if enemy.dir_x != 0.0 {
                transform.translation.x += enemy.dir_x * 45.0 * dt;
                if transform.translation.x > 250.0 || transform.translation.x < -250.0 {
                    enemy.dir_x *= -1.0;
                }
            }

            if current_player.time_elapsed > 40.0 {
                enemy.shoot_timer -= dt;
                if enemy.shoot_timer <= 0.0 {
                    enemy.shoot_timer = rng.gen_range(2.5..5.0);
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 0.4, 0.1),
                                custom_size: Some(Vec2::new(6.0, 18.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(
                                transform.translation.x,
                                transform.translation.y - 20.0,
                                7.0,
                            ),
                            ..default()
                        },
                        EnemyLaser {
                            velocity: Vec2::new(0.0, -340.0),
                        },
                    ));
                }
            }

            if transform.translation.y < -430.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ============================================================================
// Dificultad y Oleadas
// ============================================================================

fn difficulty_and_wave_system(
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();
    current_player.time_elapsed += dt;

    if current_player.status_timer > 0.0 {
        current_player.status_timer -= dt;
    }

    timers.wave_timer.tick(time.delta());
    if timers.wave_timer.just_finished() {
        current_player.wave += 1;
        current_player.status_message = format!("OLEADA {}", current_player.wave);
        current_player.status_timer = 2.0;

        let new_interval =
            (1.1 - (current_player.time_elapsed * 0.005) - (current_player.wave as f32 * 0.06))
                .max(0.32);
        timers
            .enemy_spawn
            .set_duration(std::time::Duration::from_secs_f32(new_interval));
    }
}

// ============================================================================
// PowerUps (Habilidades Flotantes)
// ============================================================================

fn powerup_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut screen_shake: ResMut<ScreenShake>,
    skill_draft: Res<SkillDraftOptions>,
    mut query_powerups: Query<(Entity, &mut Transform, &PowerUpItem)>,
    query_player: Query<&Transform, (With<Player>, Without<PowerUpItem>, Without<Enemy>)>,
    mut query_enemies: Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: Query<Entity, With<EnemyLaser>>,
) {
    if skill_draft.is_active {
        return;
    }
    let dt = time.delta_seconds();

    for (p_entity, mut p_tr, powerup) in query_powerups.iter_mut() {
        p_tr.translation.y -= powerup.speed * dt;
        p_tr.rotate_z(1.4 * dt);
        let pulse = (time.elapsed_seconds() * 4.0).sin() * 0.08 + 1.0;
        p_tr.scale = Vec3::splat(pulse);

        if let Ok(player_tr) = query_player.get_single() {
            let p_pos = player_tr.translation.truncate();
            let item_pos = p_tr.translation.truncate();

            if p_pos.distance(item_pos) < 46.0 {
                match powerup.kind {
                    PowerUpType::TripleShot => {
                        current_player.triple_shot_timer = 12.0;
                        current_player.status_message = "DISPARO TRIPLE OBTENIDO".to_string();
                    }
                    PowerUpType::Shield => {
                        current_player.shield =
                            (current_player.shield + 50.0).min(current_player.max_shield);
                        current_player.shield_hit_timer = 0.45;
                        current_player.status_message = "ESCUDO RECARGADO".to_string();
                    }
                    PowerUpType::Health => {
                        current_player.health =
                            (current_player.health + 40.0).min(current_player.max_health);
                        current_player.status_message = "CASCO REPARADO (+40 HP)".to_string();
                    }
                    PowerUpType::Nuke => {
                        trigger_nuke_effect(
                            &mut commands,
                            &assets,
                            &mut current_player,
                            &mut screen_shake,
                            &mut query_enemies,
                            &query_enemy_lasers,
                        );
                    }
                }
                play_sound(&mut commands, assets.snd_powerup_pickup.clone(), 0.48);
                current_player.status_timer = 2.5;
                spawn_spark(&mut commands, item_pos);
                commands.entity(p_entity).despawn();
                continue;
            }
        }

        if p_tr.translation.y < -420.0 {
            commands.entity(p_entity).despawn();
        }
    }
}

fn spawn_experience_orb(commands: &mut Commands, assets: &GameAssets, position: Vec2, value: u32) {
    let pulse_phase = rand::thread_rng().gen_range(0.0..std::f32::consts::TAU);
    commands.spawn((
        SpriteBundle {
            texture: assets.experience_orb.clone(),
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(25.0)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 8.6),
            ..default()
        },
        ExperienceOrb {
            value,
            speed: 82.0,
            pulse_phase,
        },
    ));
}

fn gain_experience(current_player: &mut CurrentPlayer, amount: u32) {
    if current_player.class_upgrade_count >= 9 {
        return;
    }

    current_player.experience += amount;
    refresh_experience_target(current_player);
}

fn experience_orb_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
    mut query_orbs: Query<(Entity, &mut Transform, &ExperienceOrb)>,
    query_player: Query<&Transform, (With<Player>, Without<ExperienceOrb>)>,
) {
    if skill_draft.is_active {
        return;
    }

    let Ok(player_transform) = query_player.get_single() else {
        return;
    };
    let player_position = player_transform.translation.truncate();
    let dt = time.delta_seconds();

    for (orb_entity, mut orb_transform, orb) in query_orbs.iter_mut() {
        let orb_position = orb_transform.translation.truncate();
        let distance = player_position.distance(orb_position);

        orb_transform.rotate_z(3.5 * dt);
        let pulse = (time.elapsed_seconds() * 5.5 + orb.pulse_phase).sin() * 0.12 + 1.0;
        orb_transform.scale = Vec3::splat(pulse);
        if distance < 150.0 {
            let direction = (player_position - orb_position).normalize_or_zero();
            orb_transform.translation += (direction * (orb.speed + 180.0 * dt)).extend(0.0) * dt;
        } else {
            orb_transform.translation.y -= orb.speed * dt;
        }

        if distance < 42.0 {
            gain_experience(&mut current_player, orb.value);
            current_player.status_message = format!("+{} XP", orb.value);
            current_player.status_timer = 0.8;
            play_sound(&mut commands, assets.snd_powerup_pickup.clone(), 0.18);
            spawn_spark(&mut commands, orb_position);
            commands.entity(orb_entity).despawn();
        } else if orb_transform.translation.y < -430.0 {
            commands.entity(orb_entity).despawn();
        }
    }
}

fn trigger_nuke_effect(
    commands: &mut Commands,
    assets: &GameAssets,
    current_player: &mut CurrentPlayer,
    screen_shake: &mut ScreenShake,
    query_enemies: &mut Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: &Query<Entity, With<EnemyLaser>>,
) {
    screen_shake.timer = 0.32;
    screen_shake.intensity = 8.5;
    trigger_vibration(250);

    for laser_e in query_enemy_lasers.iter() {
        commands.entity(laser_e).despawn();
    }

    let mut hit_boss = false;
    for (enemy_e, mut enemy, enemy_tr) in query_enemies.iter_mut() {
        let e_pos = enemy_tr.translation.truncate();
        if enemy.is_boss {
            hit_boss = true;
            enemy.health -= 150.0;
            spawn_bevy_explosion(commands, e_pos);
            if enemy.health <= 0.0 {
                trigger_vibration(280);
                play_sound(commands, assets.snd_boss_death.clone(), 0.70);
                current_player.score += 1500;
                current_player.enemies_killed += 1;
                spawn_experience_orb(commands, assets, e_pos, 5);
                current_player.status_message = "BORROW CHECKER SUPERADO (+1500 PTS)".to_string();
                spawn_floating_text(
                    commands,
                    e_pos,
                    "+1500 BORROW CHECKER",
                    Color::srgb(1.0, 0.85, 0.2),
                );
                commands.entity(enemy_e).despawn();
            } else {
                if !enemy.is_enraged && enemy.health <= enemy.max_health * 0.5 {
                    enemy.is_enraged = true;
                    current_player.status_message =
                        "ALERTA CRITICA: BORROW CHECKER EN MODO FURIA".to_string();
                } else {
                    current_player.status_message = "EMP IMPACTO AL JEFE (-150 HP)".to_string();
                }
                play_sound(commands, assets.snd_enemy_death.clone(), 0.40);
                spawn_floating_text(commands, e_pos, "-150 EMP", Color::srgb(0.4, 0.95, 1.0));
            }
        } else {
            spawn_bevy_explosion(commands, e_pos);
            play_sound(commands, assets.snd_enemy_death.clone(), 0.28);
            current_player.score += 75;
            current_player.enemies_killed += 1;
            spawn_experience_orb(commands, assets, e_pos, 1);
            commands.entity(enemy_e).despawn();
        }
    }

    spawn_bevy_explosion(commands, Vec2::ZERO);
    if !hit_boss {
        current_player.score += 300;
        current_player.status_message = "CARGO CLEAN: BOMBA EMP DETONADA".to_string();
    }
}

fn activate_class_ability(
    ability: ClassAbility,
    ability_level: u8,
    commands: &mut Commands,
    assets: &GameAssets,
    current_player: &mut CurrentPlayer,
    player_position: Vec2,
    screen_shake: &mut ScreenShake,
    query_enemies: &mut Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: &Query<Entity, With<EnemyLaser>>,
) {
    let level_bonus = ability_level.min(3) as f32;
    let ability_color = {
        let [r, g, b] = ability.color_rgb();
        Color::srgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    };

    match ability {
        ClassAbility::BastionProtocol => {
            current_player.shield =
                (current_player.shield + 65.0 + level_bonus * 15.0).min(current_player.max_shield);
            current_player.damage_reduction_timer = 5.0 + level_bonus * 0.4;
            current_player.damage_reduction_mult = 0.35;
            current_player.shield_hit_timer = 0.6;
            current_player.status_message = "BASTIÓN MUTEX: DAÑO REDUCIDO 5S".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(120);
        }
        ClassAbility::KineticPulse => {
            trigger_nuke_effect(
                commands,
                assets,
                current_player,
                screen_shake,
                query_enemies,
                query_enemy_lasers,
            );
            current_player.status_message = "PULSO CINÉTICO: ¡LÍNEA DESPEJADA!".to_string();
            current_player.status_timer = 2.5;
        }
        ClassAbility::HeavyOverdrive => {
            current_player.class_fire_rate_timer = 5.0 + level_bonus * 0.5;
            current_player.class_fire_rate_mult = 1.75 + level_bonus * 0.12;
            current_player.class_damage_timer = 5.0 + level_bonus * 0.5;
            current_player.class_damage_mult = 1.25 + level_bonus * 0.05;
            current_player.status_message = "SOBRECARGA DEL TITÁN: CAÑÓN AL MÁXIMO".to_string();
            current_player.status_timer = 3.0;
            trigger_vibration(100);
        }
        ClassAbility::ArcaneNova => {
            let damage =
                current_player.ship_class.laser_damage() * current_player.damage_mult * 1.35;
            let orb_count = 12 + ability_level.min(3) as usize * 4;
            for index in 0..orb_count {
                let angle = index as f32 * std::f32::consts::TAU / orb_count as f32;
                let velocity = Vec2::new(angle.sin() * 620.0, angle.cos() * 620.0);
                spawn_laser_projectile(
                    commands,
                    player_position,
                    velocity,
                    Vec2::new(9.0, 30.0),
                    ability_color,
                    damage,
                    false,
                    false,
                );
            }
            spawn_bevy_explosion(commands, player_position);
            screen_shake.timer = 0.22;
            screen_shake.intensity = 5.0;
            current_player.status_message = "NOVA ARCANA: ORBES EN TODAS DIRECCIONES".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(100);
        }
        ClassAbility::ZeroCostBeam => {
            current_player.beam_active_timer = 3.0 + level_bonus * 0.5;
            current_player.status_message = format!(
                "RAYO ZERO-COST ACTIVADO ({:.1}S)",
                current_player.beam_active_timer
            );
            current_player.status_timer = 3.0;
            trigger_vibration(200);
        }
        ClassAbility::AetherShield => {
            current_player.shield = current_player.max_shield;
            current_player.shield_hit_timer = 0.65;
            let duration = 1.5 + level_bonus * 0.25;
            current_player.invulnerable_timer = current_player.invulnerable_timer.max(duration);
            current_player.status_message = format!("ESCUDO ÉTER: INVULNERABLE {:.1}S", duration);
            current_player.status_timer = 2.5;
            trigger_vibration(100);
        }
        ClassAbility::PhaseShift => {
            let duration = 2.0 + level_bonus * 0.3;
            current_player.invulnerable_timer = current_player.invulnerable_timer.max(duration);
            current_player.class_speed_timer = duration;
            current_player.class_speed_mult = 2.1 + level_bonus * 0.15;
            current_player.status_message = "FASE ASÍNCRONA: VELOCIDAD EXTREMA".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(140);
        }
        ClassAbility::TwinFang => {
            current_player.class_fire_rate_timer = 4.0 + level_bonus * 0.5;
            current_player.class_fire_rate_mult = 2.0 + level_bonus * 0.15;
            current_player.class_damage_timer = 4.0 + level_bonus * 0.5;
            current_player.class_damage_mult = 1.15 + level_bonus * 0.05;
            current_player.status_message = "COLMILLO DOBLE: RÁFAGA VERDE x2".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(100);
        }
        ClassAbility::HunterMark => {
            current_player.hunter_mark_timer = 6.0 + level_bonus;
            current_player.hunter_mark_crit_bonus = 0.55 + level_bonus * 0.06;
            current_player.status_message = "MARCA DEL CAZADOR: CRÍTICOS ACTIVADOS".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(100);
        }
        ClassAbility::MissileSalvo => {
            let damage = current_player.ship_class.laser_damage()
                * current_player.damage_mult
                * (1.85 + level_bonus * 0.15);
            for (angle, offset_x) in [
                (-0.30_f32, -24.0_f32),
                (-0.15_f32, -12.0_f32),
                (0.0_f32, 0.0_f32),
                (0.15_f32, 12.0_f32),
                (0.30_f32, 24.0_f32),
            ] {
                let velocity = Vec2::new(angle.sin() * 700.0, angle.cos() * 700.0);
                spawn_laser_projectile(
                    commands,
                    player_position + Vec2::new(offset_x, 25.0),
                    velocity,
                    Vec2::new(13.0, 42.0),
                    ability_color,
                    damage,
                    false,
                    false,
                );
            }
            current_player.status_message = "SALVA DE MISILES: 5 PROYECTILES".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(160);
        }
        ClassAbility::CargoBombardment => {
            trigger_nuke_effect(
                commands,
                assets,
                current_player,
                screen_shake,
                query_enemies,
                query_enemy_lasers,
            );
            current_player.status_message = "BOMBARDEO CARGO: EMP DETONADO".to_string();
            current_player.status_timer = 2.5;
        }
        ClassAbility::FortifyArmor => {
            current_player.health =
                (current_player.health + 35.0 + level_bonus * 10.0).min(current_player.max_health);
            current_player.shield =
                (current_player.shield + 65.0 + level_bonus * 15.0).min(current_player.max_shield);
            current_player.damage_reduction_timer = 4.0 + level_bonus * 0.3;
            current_player.damage_reduction_mult = 0.50;
            current_player.status_message = "BLINDAJE CARGO: CASCO Y ESCUDO REPARADOS".to_string();
            current_player.status_timer = 2.5;
            trigger_vibration(100);
        }
    }
}

fn spawn_powerup_item(commands: &mut Commands, assets: &GameAssets, pos: Vec2, kind: PowerUpType) {
    let texture = match kind {
        PowerUpType::TripleShot => assets.powerup_triple.clone(),
        PowerUpType::Shield => assets.powerup_shield.clone(),
        PowerUpType::Health => assets.powerup_health.clone(),
        PowerUpType::Nuke => assets.powerup_nuke.clone(),
    };

    commands.spawn((
        SpriteBundle {
            texture,
            sprite: Sprite {
                custom_size: Some(Vec2::splat(42.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 8.5),
            ..default()
        },
        PowerUpItem { kind, speed: 75.0 },
    ));
}

// ============================================================================
// Colisiones y Muerte del Jefe -> Trigger de Draft
// ============================================================================

fn apply_damage_to_player(
    commands: &mut Commands,
    assets: &GameAssets,
    current_player: &mut ResMut<CurrentPlayer>,
    mut damage: f32,
) {
    if current_player.invulnerable_timer > 0.0 {
        return;
    }

    if current_player.damage_reduction_timer > 0.0 {
        damage *= current_player.damage_reduction_mult;
    }

    // Panic Recovery Check
    if current_player
        .passive_skills
        .contains(&SkillId::PanicRecovery)
        && !current_player.panic_recovery_used
    {
        let total_hp = current_player.health + current_player.shield;
        if total_hp <= damage {
            current_player.panic_recovery_used = true;
            current_player.invulnerable_timer = 2.5;
            current_player.status_message =
                "¡PANIC RECOVERY ACTIVADO! (INVULNERABLE 2.5S)".to_string();
            current_player.status_timer = 2.5;
            spawn_spark(commands, Vec2::ZERO);
            trigger_vibration(200);
            return;
        }
    }

    current_player.invulnerable_timer = 0.35;
    current_player.combo_count = 0;
    current_player.combo_multiplier = 1;

    trigger_vibration(50);
    play_sound(commands, assets.snd_player_damage.clone(), 0.40);

    // Borrow Checker: Reflejar 25% de daño a enemigos
    if current_player
        .passive_skills
        .contains(&SkillId::BorrowChecker)
    {
        let reflect_dmg = damage * 0.25;
        spawn_floating_text(
            commands,
            Vec2::ZERO,
            &format!("REFLEJO {:.0}", reflect_dmg),
            Color::srgb(0.2, 1.0, 0.5),
        );
    }

    if current_player.shield > 0.0 {
        current_player.shield_hit_timer = 0.28;
        if current_player.shield >= damage {
            current_player.shield -= damage;
        } else {
            let leftover = damage - current_player.shield;
            current_player.shield = 0.0;
            current_player.health -= leftover;
            current_player.hull_hit_timer = 0.18;
        }
    } else {
        current_player.health -= damage;
        current_player.hull_hit_timer = 0.18;
    }
}

fn collision_system(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut screen_shake: ResMut<ScreenShake>,
    mut next_state: ResMut<NextState<AppState>>,
    mut leaderboard: ResMut<Leaderboard>,
    mut skill_draft: ResMut<SkillDraftOptions>,
    mut query_enemies: Query<(Entity, &Transform, &mut Enemy)>,
    query_lasers: Query<(Entity, &Transform, &Laser)>,
    query_enemy_lasers: Query<(Entity, &Transform), With<EnemyLaser>>,
    query_player: Query<&Transform, With<Player>>,
) {
    if skill_draft.is_active {
        return;
    }
    let mut rng = rand::thread_rng();

    // Láseres del Jugador vs Enemigos / Jefes
    for (laser_entity, laser_tr, laser) in query_lasers.iter() {
        let l_pos = laser_tr.translation.truncate();

        for (enemy_entity, enemy_tr, mut enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            let hit_radius = enemy.size * 0.45;

            if l_pos.distance(e_pos) < hit_radius {
                commands.entity(laser_entity).despawn();
                enemy.health -= laser.damage;

                if enemy.is_boss
                    && !enemy.is_enraged
                    && enemy.health <= enemy.max_health * 0.5
                    && enemy.health > 0.0
                {
                    enemy.is_enraged = true;
                    current_player.status_message =
                        "ALERTA CRITICA: BORROW CHECKER EN MODO FURIA".to_string();
                    current_player.status_timer = 4.0;
                    screen_shake.timer = 0.28;
                    screen_shake.intensity = 6.5;
                    trigger_vibration(180);
                }

                if enemy.health <= 0.0 {
                    current_player.combo_count += 1;
                    current_player.combo_timer = 2.4;
                    current_player.combo_multiplier = match current_player.combo_count {
                        0..=2 => 1,
                        3..=5 => 2,
                        6..=9 => 3,
                        10..=14 => 4,
                        _ => 5,
                    };
                    if current_player.combo_count > current_player.max_combo {
                        current_player.max_combo = current_player.combo_count;
                    }

                    let pts = enemy.score_value * current_player.combo_multiplier;
                    current_player.score += pts;
                    current_player.enemies_killed += 1;

                    spawn_bevy_explosion(&mut commands, e_pos);
                    spawn_experience_orb(
                        &mut commands,
                        &assets,
                        e_pos,
                        if enemy.is_boss { 5 } else { 1 },
                    );

                    let drop_prob = 0.24 * current_player.powerup_drop_mult;
                    let should_drop = enemy.is_boss || rng.gen_bool(drop_prob.min(0.85) as f64);
                    if should_drop {
                        let kind = match rng.gen_range(0..4) {
                            0 => PowerUpType::TripleShot,
                            1 => PowerUpType::Shield,
                            2 => PowerUpType::Health,
                            _ => PowerUpType::Nuke,
                        };
                        spawn_powerup_item(&mut commands, &assets, e_pos, kind);
                    }

                    if enemy.is_boss {
                        screen_shake.timer = 0.35;
                        screen_shake.intensity = 9.5;
                        trigger_vibration(280);
                        play_sound(&mut commands, assets.snd_boss_death.clone(), 0.70);
                        current_player.status_message =
                            "¡JEFE DERROTADO! ELIGE TU HABILIDAD".to_string();
                        current_player.status_timer = 3.0;

                        // Lanzar Modal de Draft (3 Habilidades Aleatorias)
                        let all_skills = vec![
                            SkillId::OverclockMutex,
                            SkillId::AsyncMultithread,
                            SkillId::ZeroCostBeam,
                            SkillId::UnsafeBlock,
                            SkillId::BorrowChecker,
                            SkillId::PatternMatching,
                            SkillId::CargoClean,
                            SkillId::ArcMutex,
                            SkillId::TokioReactor,
                            SkillId::VectorCapacity,
                            SkillId::PanicRecovery,
                            SkillId::MutexOverdrive,
                            SkillId::StaticLifetime,
                            SkillId::MacroRules,
                            SkillId::OptionSome,
                            SkillId::ZeroCostAbstraction,
                        ];
                        let mut chosen = Vec::new();
                        while chosen.len() < 3 {
                            let cand = all_skills[rng.gen_range(0..all_skills.len())];
                            if !chosen.contains(&cand) {
                                chosen.push(cand);
                            }
                        }
                        skill_draft.options = chosen;
                        skill_draft.is_active = true;
                    } else {
                        play_sound(&mut commands, assets.snd_enemy_death.clone(), 0.28);
                    }

                    commands.entity(enemy_entity).despawn();
                } else {
                    spawn_spark(&mut commands, l_pos);
                }
                break;
            }
        }
    }

    if let Ok(player_tr) = query_player.get_single() {
        let p_pos = player_tr.translation.truncate();

        for (elaser_entity, elaser_tr) in query_enemy_lasers.iter() {
            let el_pos = elaser_tr.translation.truncate();
            if p_pos.distance(el_pos) < 32.0 {
                commands.entity(elaser_entity).despawn();
                apply_damage_to_player(&mut commands, &assets, &mut current_player, 16.0);
                spawn_spark(&mut commands, el_pos);
            }
        }

        for (enemy_entity, enemy_tr, enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            if p_pos.distance(e_pos) < (32.0 + enemy.size * 0.35) {
                let dmg = if enemy.is_boss { 45.0 } else { 24.0 };
                apply_damage_to_player(&mut commands, &assets, &mut current_player, dmg);
                spawn_bevy_explosion(&mut commands, e_pos);
                if !enemy.is_boss {
                    spawn_experience_orb(&mut commands, &assets, e_pos, 1);
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }

    if current_player.health <= 0.0 {
        current_player.health = 0.0;
        trigger_vibration(400);
        play_sound(&mut commands, assets.snd_player_death.clone(), 0.85);
        leaderboard.add_score(
            current_player.name.clone(),
            current_player.score,
            current_player.wave,
        );
        next_state.set(AppState::GameOver);
    }
}

// ============================================================================
// Particulas Bevy
// ============================================================================

fn spawn_bevy_explosion(commands: &mut Commands, pos: Vec2) {
    let mut rng = rand::thread_rng();

    for _ in 0..28 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(90.0..340.0);
        let size = rng.gen_range(5.0..12.0);
        let life = rng.gen_range(0.35..0.75);

        let color = match rng.gen_range(0..4) {
            0 => Color::srgb(1.0, 0.35, 0.1),
            1 => Color::srgb(1.0, 0.85, 0.2),
            2 => Color::srgb(0.2, 0.9, 1.0),
            _ => Color::WHITE,
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 12.0),
                ..default()
            },
            ExplosionParticle {
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                lifetime: Timer::from_seconds(life, TimerMode::Once),
                initial_size: size,
            },
        ));
    }
}

fn spawn_spark(commands: &mut Commands, pos: Vec2) {
    let mut rng = rand::thread_rng();
    for _ in 0..8 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(60.0..180.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.4, 0.95, 1.0),
                    custom_size: Some(Vec2::splat(4.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 11.0),
                ..default()
            },
            ExplosionParticle {
                velocity: Vec2::new(angle.cos() * speed, angle.sin() * speed),
                lifetime: Timer::from_seconds(0.2, TimerMode::Once),
                initial_size: 4.0,
            },
        ));
    }
}

fn particle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut Sprite, &mut ExplosionParticle)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, mut sprite, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += particle.velocity.x * dt;
            transform.translation.y += particle.velocity.y * dt;
            let progress = particle.lifetime.fraction();
            let size = particle.initial_size * (1.0 - progress);
            sprite.custom_size = Some(Vec2::splat(size));
        }
    }
}

fn thruster_particle_system(
    mut commands: Commands,
    time: Res<Time>,
    query_player: Query<&Transform, With<Player>>,
    mut query_particles: Query<
        (Entity, &mut Transform, &mut Sprite, &mut ThrusterParticle),
        Without<Player>,
    >,
) {
    let dt = time.delta_seconds();
    let mut rng = rand::thread_rng();

    if let Ok(player_tr) = query_player.get_single() {
        let p_pos = player_tr.translation;
        for _ in 0..2 {
            let offset_x = rng.gen_range(-12.0..12.0);
            let vx = rng.gen_range(-15.0..15.0);
            let vy = rng.gen_range(-180.0..-100.0);
            let size = rng.gen_range(4.0..7.5);
            let life = rng.gen_range(0.18..0.35);

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgba(0.2, 0.85, 1.0, 0.85),
                        custom_size: Some(Vec2::splat(size)),
                        ..default()
                    },
                    transform: Transform::from_xyz(p_pos.x + offset_x, p_pos.y - 32.0, 9.0),
                    ..default()
                },
                ThrusterParticle {
                    velocity: Vec2::new(vx, vy),
                    lifetime: Timer::from_seconds(life, TimerMode::Once),
                    initial_size: size,
                },
            ));
        }
    }

    for (entity, mut transform, mut sprite, mut particle) in query_particles.iter_mut() {
        particle.lifetime.tick(time.delta());
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += particle.velocity.x * dt;
            transform.translation.y += particle.velocity.y * dt;
            let progress = particle.lifetime.fraction();
            let size = particle.initial_size * (1.0 - progress);
            sprite.custom_size = Some(Vec2::splat(size));
        }
    }
}

fn camera_shake_system(
    time: Res<Time>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query_camera: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(mut camera_tr) = query_camera.get_single_mut() {
        if screen_shake.timer > 0.0 {
            screen_shake.timer -= time.delta_seconds();
            let mut rng = rand::thread_rng();
            let shake_x = rng.gen_range(-1.0..1.0) * screen_shake.intensity;
            let shake_y = rng.gen_range(-1.0..1.0) * screen_shake.intensity;
            camera_tr.translation.x = shake_x;
            camera_tr.translation.y = shake_y;
        } else {
            camera_tr.translation.x = 0.0;
            camera_tr.translation.y = 0.0;
        }
    }
}

fn floating_text_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FloatingText)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, mut floating) in query.iter_mut() {
        floating.timer.tick(time.delta());
        if floating.timer.finished() {
            commands.entity(entity).despawn();
        } else {
            transform.translation.x += floating.velocity.x * dt;
            transform.translation.y += floating.velocity.y * dt;
        }
    }
}

fn combo_system(
    time: Res<Time>,
    mut current_player: ResMut<CurrentPlayer>,
    skill_draft: Res<SkillDraftOptions>,
) {
    if skill_draft.is_active {
        return;
    }
    if current_player.combo_timer > 0.0 {
        current_player.combo_timer -= time.delta_seconds();
        if current_player.combo_timer <= 0.0 {
            current_player.combo_count = 0;
            current_player.combo_multiplier = 1;
        }
    }
}

fn spawn_floating_text(commands: &mut Commands, pos: Vec2, _text: &str, _color: Color) {
    let mut rng = rand::thread_rng();
    let vx = rng.gen_range(-20.0..20.0);
    let vy = rng.gen_range(40.0..70.0);

    commands.spawn((
        SpatialBundle {
            transform: Transform::from_xyz(pos.x, pos.y + 15.0, 15.0),
            ..default()
        },
        FloatingText {
            timer: Timer::from_seconds(0.85, TimerMode::Once),
            velocity: Vec2::new(vx, vy),
        },
    ));
}

fn planet_system(time: Res<Time>, mut query: Query<(&mut Transform, &BackgroundPlanet)>) {
    let dt = time.delta_seconds();
    for (mut transform, planet) in query.iter_mut() {
        transform.translation.y -= planet.speed * dt;
        if transform.translation.y < -750.0 {
            let mut rng = rand::thread_rng();
            transform.translation.y = 750.0;
            transform.translation.x = rng.gen_range(-220.0..220.0);
        }
    }
}

fn star_system(time: Res<Time>, mut query: Query<(&mut Transform, &BackgroundStar)>) {
    let dt = time.delta_seconds();
    for (mut transform, star) in query.iter_mut() {
        transform.translation.y -= star.speed * dt;
        if transform.translation.y < -660.0 {
            let mut rng = rand::thread_rng();
            transform.translation.y = 660.0;
            transform.translation.x = rng.gen_range(-380.0..380.0);
        }
    }
}

// ============================================================================
// UI CON EGUI: Selección de Nave en Formato Vertical Completo
// ============================================================================

fn ui_name_input(
    mut contexts: EguiContexts,
    mut current_player: ResMut<CurrentPlayer>,
    leaderboard: Res<Leaderboard>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<AppState>>,
    mut windows: Query<&mut Window>,
) {
    let card_defensor_id = contexts.add_image(assets.card_defensor.clone_weak());
    let card_mago_id = contexts.add_image(assets.card_mago.clone_weak());
    let card_asesino_id = contexts.add_image(assets.card_asesino.clone_weak());
    let card_artillero_id = contexts.add_image(assets.card_artillero.clone_weak());
    let primary_orange = egui::Color32::from_rgb(255, 140, 0);
    let secondary_white = egui::Color32::WHITE;

    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(8, 8, 8)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(45.0);
                        ui.heading(
                            egui::RichText::new("FERRIS SPACE DEFENDER")
                                .size(24.0)
                                .color(primary_orange)
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new("RUST PERU COMMUNITY EDITION")
                                .size(12.0)
                                .color(secondary_white)
                                .strong(),
                        );
                        ui.add_space(10.0);

                        // Card Nombre del Piloto con Teclado Táctil Nativo
                        egui::Frame::default()
                            .fill(egui::Color32::from_rgb(18, 18, 18))
                            .stroke(egui::Stroke::new(1.0_f32, primary_orange))
                            .rounding(8.0)
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width().min(340.0));
                                ui.label(
                                    egui::RichText::new("IDENTIFICADOR DEL PILOTO")
                                        .size(12.0)
                                        .color(secondary_white)
                                        .strong(),
                                );
                                ui.add_space(4.0);

                                // El campo ocupaba solo 220x36 px dentro de una tarjeta de 340 px.
                                // En una pantalla táctil eso hacía que muchos toques sobre el
                                // placeholder no llegaran al TextEdit. Lo hacemos casi del ancho
                                // completo y más alto para que toda la zona amarilla sea fácil de tocar.
                                let input_width = ui.available_width().min(320.0);
                                let text_edit_response = ui.add_sized(
                                    egui::vec2(input_width, 52.0),
                                    egui::TextEdit::singleline(&mut current_player.name)
                                        .hint_text(
                                            egui::RichText::new("Toca para escribir tu nombre...")
                                                .color(secondary_white),
                                        )
                                        .text_color(secondary_white)
                                        .font(egui::FontId::proportional(16.0))
                                        .margin(egui::vec2(8.0, 6.0))
                                        .desired_width(input_width),
                                );

                                // Pedimos el foco explícitamente al primer toque. Esto evita
                                // depender de que egui y Android resuelvan el foco en el mismo
                                // frame y hace que el teclado aparezca de forma consistente.
                                if text_edit_response.clicked() {
                                    ctx.memory_mut(|memory| {
                                        memory.request_focus(text_edit_response.id)
                                    });
                                }

                                let input_has_focus = text_edit_response.has_focus();
                                if let Ok(mut window) = windows.get_single_mut() {
                                    if input_has_focus || text_edit_response.clicked() {
                                        window.ime_enabled = true;
                                        window.ime_position = Vec2::new(
                                            text_edit_response.rect.left(),
                                            text_edit_response.rect.bottom(),
                                        );
                                    } else if !ctx.wants_keyboard_input() {
                                        // Solo apagamos el IME cuando egui ya no tiene ningún
                                        // control de texto activo. Así no cortamos el teclado
                                        // durante la transición inicial del foco.
                                        window.ime_enabled = false;
                                    }
                                }

                                ui.add_space(4.0);
                                ui.horizontal(|ui| {
                                    if ui
                                        .button(
                                            egui::RichText::new("Borrar").color(secondary_white),
                                        )
                                        .clicked()
                                    {
                                        current_player.name.clear();
                                    }
                                    if ui
                                        .button(
                                            egui::RichText::new("Aleatorio").color(secondary_white),
                                        )
                                        .clicked()
                                    {
                                        let n = [
                                            "Ferris_Pro",
                                            "Rustacean",
                                            "Async_King",
                                            "Cargo_Runner",
                                            "Borrow_God",
                                        ];
                                        let mut rng = rand::thread_rng();
                                        current_player.name =
                                            n[rng.gen_range(0..n.len())].to_string();
                                    }
                                });
                            });

                        ui.add_space(14.0);
                        ui.heading(
                            egui::RichText::new("SELECCIONA TU CLASE DE NAVE")
                                .size(16.0)
                                .color(primary_orange)
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new(
                                "Desliza verticalmente para explorar las 4 clases de naves",
                            )
                            .size(11.0)
                            .color(secondary_white),
                        );
                        ui.add_space(8.0);

                        // Lista Vertical de Cartas de Clase a Ancho Completo (Solo Imágenes Interactivas)
                        let class_items = [
                            (ShipClass::Defensor, card_defensor_id),
                            (ShipClass::Mago, card_mago_id),
                            (ShipClass::Asesino, card_asesino_id),
                            (ShipClass::Artillero, card_artillero_id),
                        ];

                        let avail_w = ui.available_width();
                        let card_w = (avail_w - 20.0).clamp(280.0, 340.0);
                        let card_h = card_w * (340.0 / 220.0);

                        for (s_class, card_tex_id) in class_items {
                            let is_selected = current_player.ship_class == s_class;
                            let frame_stroke = if is_selected {
                                egui::Stroke::new(3.5_f32, primary_orange)
                            } else {
                                egui::Stroke::new(1.0_f32, secondary_white)
                            };

                            egui::Frame::default()
                                .fill(if is_selected {
                                    egui::Color32::from_rgb(20, 20, 20)
                                } else {
                                    egui::Color32::from_rgb(10, 10, 10)
                                })
                                .stroke(frame_stroke)
                                .rounding(12.0)
                                .inner_margin(6.0)
                                .show(ui, |ui| {
                                    ui.set_max_width(card_w);
                                    let img_btn = ui.add(
                                        egui::Image::new(egui::load::SizedTexture::new(
                                            card_tex_id,
                                            egui::vec2(card_w - 12.0, card_h - 12.0),
                                        ))
                                        .sense(egui::Sense::click()),
                                    );
                                    if img_btn.clicked() {
                                        current_player.ship_class = s_class;
                                    }
                                });
                            ui.add_space(10.0);
                        }

                        ui.add_space(10.0);

                        let btn_start = ui.add(
                            egui::Button::new(
                                egui::RichText::new("INICIAR MISION EN EL ESPACIO")
                                    .size(17.0)
                                    .color(secondary_white)
                                    .strong(),
                            )
                            .min_size(egui::vec2(310.0, 48.0))
                            .fill(primary_orange),
                        );

                        if btn_start.clicked() {
                            if current_player.name.trim().is_empty() {
                                current_player.name = "Piloto_Rust".to_string();
                            }
                            if let Ok(mut window) = windows.get_single_mut() {
                                window.ime_enabled = false;
                            }
                            next_state.set(AppState::Playing);
                        }

                        ui.add_space(16.0);
                        ui.heading(
                            egui::RichText::new("TABLA DE LIDERES (TOP 5)")
                                .size(14.0)
                                .color(primary_orange),
                        );
                        ui.add_space(4.0);

                        egui::Frame::default()
                            .fill(egui::Color32::from_rgb(12, 12, 12))
                            .rounding(8.0)
                            .inner_margin(8.0)
                            .show(ui, |ui| {
                                egui::Grid::new("init_leaderboard")
                                    .striped(true)
                                    .min_col_width(65.0)
                                    .show(ui, |ui| {
                                        for (idx, entry) in
                                            leaderboard.entries.iter().take(5).enumerate()
                                        {
                                            ui.label(
                                                egui::RichText::new(format!("{}.", idx + 1))
                                                    .color(secondary_white),
                                            );
                                            ui.label(
                                                egui::RichText::new(&entry.name)
                                                    .color(secondary_white),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!("{} pts", entry.score))
                                                    .color(secondary_white),
                                            );
                                            ui.label(
                                                egui::RichText::new(format!("Ola {}", entry.wave))
                                                    .color(secondary_white),
                                            );
                                            ui.end_row();
                                        }
                                    });
                            });
                        ui.add_space(20.0);
                    });
                });
        });
}

// ============================================================================
// UI CON EGUI: Modal de Selection Rogue-lite al Derrotar Jefe
// ============================================================================

fn ui_skill_draft(
    mut contexts: EguiContexts,
    mut current_player: ResMut<CurrentPlayer>,
    mut skill_draft: ResMut<SkillDraftOptions>,
    assets: Res<GameAssets>,
) {
    let mut skill_tex_map = HashMap::new();
    for (&skill, handle) in &assets.skill_textures {
        let tid = contexts.add_image(handle.clone_weak());
        skill_tex_map.insert(skill, tid);
    }

    if !skill_draft.is_active {
        return;
    }

    let ctx = contexts.ctx_mut();
    let screen_rect = ctx.screen_rect();
    let avail_w = screen_rect.width();

    // Fondo oscurecido para congelar visualmente la batalla
    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 170)))
        .show(ctx, |_ui| {});

    // Modal de 3 cartas en posición CENTER_CENTER estática fija (0 saltos de interfaz)
    let card_w = ((avail_w - 40.0) / 3.2).clamp(90.0, 130.0);
    let card_h = card_w * (340.0 / 220.0);

    egui::Area::new(egui::Id::new("skill_draft_modal_fixed_cards"))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let options = skill_draft.options.clone();
                for &skill in &options {
                    if let Some(&tex_id) = skill_tex_map.get(&skill) {
                        let img_btn = ui.add(
                            egui::Image::new(egui::load::SizedTexture::new(
                                tex_id,
                                egui::vec2(card_w, card_h),
                            ))
                            .sense(egui::Sense::click()),
                        );

                        if img_btn.clicked() {
                            if skill.is_active() {
                                if !current_player.active_skills.contains(&skill) {
                                    current_player.active_skills.push(skill);
                                }
                            } else {
                                current_player.passive_skills.push(skill);
                                match skill {
                                    SkillId::OverclockMutex => {
                                        current_player.fire_rate_mult += 0.25
                                    }
                                    SkillId::UnsafeBlock => {
                                        current_player.damage_mult += 0.40;
                                        current_player.max_health *= 0.85;
                                        current_player.health =
                                            current_player.health.min(current_player.max_health);
                                    }
                                    SkillId::PatternMatching => current_player.crit_chance += 0.15,
                                    SkillId::TokioReactor => current_player.speed_mult += 0.20,
                                    SkillId::VectorCapacity => {
                                        current_player.max_shield += 50.0;
                                        current_player.shield += 50.0;
                                    }
                                    SkillId::OptionSome => current_player.powerup_drop_mult += 0.35,
                                    SkillId::ZeroCostAbstraction => {
                                        current_player.cooldown_reduction *= 0.80
                                    }
                                    _ => {}
                                }
                            }
                            skill_draft.is_active = false;
                        }
                    }
                    ui.add_space(10.0);
                }
            });
        });
}

fn handle_class_ability_press(
    idx: usize,
    ability: ClassAbility,
    commands: &mut Commands,
    assets: &GameAssets,
    current_player: &mut CurrentPlayer,
    screen_shake: &mut ScreenShake,
    query_enemies: &mut Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: &Query<Entity, With<EnemyLaser>>,
    query_player: &Query<&Transform, With<Player>>,
) {
    let level = current_player.class_ability_levels[idx].min(3);
    let upgrade_cost = class_ability_upgrade_cost(level);
    let can_upgrade = level < 3 && current_player.experience >= upgrade_cost;

    if can_upgrade {
        current_player.experience = current_player.experience.saturating_sub(upgrade_cost);
        current_player.class_ability_levels[idx] = (level + 1).min(3);
        current_player.class_upgrade_count = current_player.class_upgrade_count.saturating_add(1);
        refresh_experience_target(current_player);
        current_player.status_message = format!(
            "{} SUBE A NIVEL {}",
            ability.name(),
            current_player.class_ability_levels[idx]
        );
        current_player.status_timer = 2.0;
    } else if current_player.class_ability_cooldowns[idx] <= 0.0 {
        current_player.class_ability_cooldowns[idx] =
            ability.cooldown_for_level(level) * current_player.cooldown_reduction;
        if let Ok(player_transform) = query_player.get_single() {
            activate_class_ability(
                ability,
                level,
                commands,
                assets,
                current_player,
                player_transform.translation.truncate(),
                screen_shake,
                query_enemies,
                query_enemy_lasers,
            );
        }
    }
}

fn class_ability_touch_system(
    touches: Res<Touches>,
    windows: Query<&Window>,
    skill_draft: Res<SkillDraftOptions>,
    mut current_player: ResMut<CurrentPlayer>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query_enemies: Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: Query<Entity, With<EnemyLaser>>,
    query_player: Query<&Transform, With<Player>>,
) {
    if skill_draft.is_active || touches.iter().count() < 2 {
        return;
    }

    let Ok(window) = windows.get_single() else {
        return;
    };
    let class_abilities = current_player.ship_class.class_abilities();

    for touch in touches.iter_just_pressed() {
        let Some(idx) = class_ability_index_at(touch.start_position(), window) else {
            continue;
        };

        handle_class_ability_press(
            idx,
            class_abilities[idx],
            &mut commands,
            &assets,
            &mut current_player,
            &mut screen_shake,
            &mut query_enemies,
            &query_enemy_lasers,
            &query_player,
        );
        break;
    }
}

fn draw_class_ability_button(
    ui: &mut egui::Ui,
    idx: usize,
    ability: ClassAbility,
    tex_id: Option<egui::TextureId>,
    pulse_time: f32,
    allow_pointer_press: bool,
    current_player: &mut CurrentPlayer,
    commands: &mut Commands,
    assets: &GameAssets,
    screen_shake: &mut ScreenShake,
    query_enemies: &mut Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: &Query<Entity, With<EnemyLaser>>,
    query_player: &Query<&Transform, With<Player>>,
) {
    let cooldown = current_player.class_ability_cooldowns[idx];
    let level = current_player.class_ability_levels[idx].min(3);
    let upgrade_cost = class_ability_upgrade_cost(level);
    let can_upgrade = level < 3 && current_player.experience >= upgrade_cost;
    let is_ready = cooldown <= 0.0;
    let sense = if allow_pointer_press && (is_ready || can_upgrade) {
        egui::Sense::click()
    } else {
        egui::Sense::hover()
    };

    let (rect, mut response) = ui.allocate_exact_size(egui::vec2(64.0, 76.0), sense);
    let image_rect =
        egui::Rect::from_min_max(rect.min, egui::pos2(rect.right(), rect.top() + 62.0));

    // El icono queda flotando sin panel ni borde permanente. Solo se dibuja
    // una capa temporal cuando la habilidad está en cooldown.
    if let Some(tex_id) = tex_id {
        let tint = if is_ready {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_gray(105)
        };
        ui.painter().image(
            tex_id,
            image_rect.shrink(2.0),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            tint,
        );
    }

    if !is_ready {
        ui.painter().rect_filled(
            image_rect,
            8.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 105),
        );
        ui.painter().text(
            image_rect.center(),
            egui::Align2::CENTER_CENTER,
            format!("{:.1}", cooldown),
            egui::FontId::proportional(12.0),
            egui::Color32::WHITE,
        );
    }

    let [r, g, b] = ability.color_rgb();
    let level_color = egui::Color32::from_rgb(r, g, b);
    let pulse = (pulse_time * 6.0).sin() * 0.5 + 0.5;
    for level_index in 0..3 {
        let center = egui::pos2(
            rect.left() + rect.width() * (level_index as f32 + 1.0) / 4.0,
            rect.bottom() - 7.0,
        );
        if level_index < level as usize {
            ui.painter().circle_filled(center, 3.0, level_color);
        } else if can_upgrade && level_index == level as usize {
            let alpha = (120.0 + pulse * 135.0) as u8;
            let radius = 3.0 + pulse * 1.6;
            ui.painter().circle_filled(
                center,
                radius,
                egui::Color32::from_rgba_unmultiplied(r, g, b, alpha),
            );
            ui.painter().circle_stroke(
                center,
                radius + 1.5,
                egui::Stroke::new(1.0_f32, level_color),
            );
        } else {
            ui.painter().circle_stroke(
                center,
                3.0,
                egui::Stroke::new(1.0_f32, egui::Color32::from_gray(110)),
            );
        }
    }

    response = response.on_hover_text(if can_upgrade {
        format!(
            "MEJORAR {} a nivel {} · cuesta {} XP",
            ability.name(),
            level + 1,
            upgrade_cost
        )
    } else {
        ability.description().to_string()
    });
    if response.clicked() && allow_pointer_press {
        handle_class_ability_press(
            idx,
            ability,
            commands,
            assets,
            current_player,
            screen_shake,
            query_enemies,
            query_enemy_lasers,
            query_player,
        );
    }
}

// ============================================================================
// UI CON EGUI: HUD de Juego y Botones Táctiles para Habilidades Activas
// ============================================================================

fn ui_playing_hud(
    mut contexts: EguiContexts,
    time: Res<Time>,
    touches: Res<Touches>,
    mut current_player: ResMut<CurrentPlayer>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut screen_shake: ResMut<ScreenShake>,
    mut query_enemies: Query<
        (Entity, &mut Enemy, &Transform),
        (Without<Player>, Without<PowerUpItem>),
    >,
    query_enemy_lasers: Query<Entity, With<EnemyLaser>>,
    query_player: Query<&Transform, With<Player>>,
) {
    let allow_pointer_press = touches.iter().count() < 2;
    let mut skill_tex_map = HashMap::new();
    for (&skill, handle) in &assets.skill_textures {
        let tid = contexts.add_image(handle.clone_weak());
        skill_tex_map.insert(skill, tid);
    }
    let mut class_ability_tex_map = HashMap::new();
    for (&ability, handle) in &assets.class_ability_textures {
        let tid = contexts.add_image(handle.clone_weak());
        class_ability_tex_map.insert(ability, tid);
    }

    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_hud")
        .frame(
            egui::Frame::default()
                .fill(egui::Color32::from_rgba_unmultiplied(2, 2, 4, 220))
                .inner_margin(egui::Margin {
                    left: 10.0,
                    right: 10.0,
                    top: 45.0,
                    bottom: 8.0,
                }),
        )
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                // Linea unica: HP - ESC - PTS - OLA
                ui.horizontal(|ui| {
                    let health_frac =
                        (current_player.health / current_player.max_health).clamp(0.0, 1.0);
                    let bar_color = if health_frac > 0.5 {
                        egui::Color32::GREEN
                    } else if health_frac > 0.25 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };
                    ui.label(
                        egui::RichText::new(format!("HP {:.0}%", health_frac * 100.0))
                            .size(11.0)
                            .color(bar_color)
                            .strong(),
                    );
                    ui.add(
                        egui::ProgressBar::new(health_frac)
                            .fill(bar_color)
                            .desired_width(45.0),
                    );

                    if current_player.shield > 0.0 {
                        ui.add_space(4.0);
                        let shield_frac =
                            (current_player.shield / current_player.max_shield).clamp(0.0, 1.0);
                        ui.label(
                            egui::RichText::new(format!("ESC {:.0}", current_player.shield))
                                .size(11.0)
                                .color(egui::Color32::from_rgb(80, 200, 255))
                                .strong(),
                        );
                        ui.add(
                            egui::ProgressBar::new(shield_frac)
                                .fill(egui::Color32::from_rgb(80, 200, 255))
                                .desired_width(40.0),
                        );
                    }

                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(format!("PTS: {}", current_player.score))
                            .color(egui::Color32::from_rgb(255, 215, 50))
                            .strong()
                            .size(12.0),
                    );

                    ui.add_space(6.0);
                    ui.label(
                        egui::RichText::new(format!("OLA: {}", current_player.wave))
                            .color(egui::Color32::LIGHT_BLUE)
                            .strong()
                            .size(12.0),
                    );
                });

                let xp_frac = if current_player.experience_to_next == 0 {
                    0.0
                } else {
                    current_player.experience as f32 / current_player.experience_to_next as f32
                };
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!(
                            "XP {}/{}",
                            current_player.experience, current_player.experience_to_next
                        ))
                        .size(10.0)
                        .color(egui::Color32::from_rgb(120, 225, 255))
                        .strong(),
                    );
                    ui.add(
                        egui::ProgressBar::new(xp_frac.clamp(0.0, 1.0))
                            .fill(egui::Color32::from_rgb(40, 190, 235))
                            .desired_width(95.0),
                    );
                });

                // Barra de vida de Jefe
                for (_, enemy, _) in query_enemies.iter() {
                    if enemy.is_boss {
                        ui.add_space(2.0);
                        let boss_frac = (enemy.health / enemy.max_health).clamp(0.0, 1.0);
                        let bar_color = if enemy.is_enraged {
                            egui::Color32::RED
                        } else {
                            egui::Color32::from_rgb(255, 140, 0)
                        };
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("JEFE:")
                                    .color(bar_color)
                                    .size(12.0)
                                    .strong(),
                            );
                            ui.add(
                                egui::ProgressBar::new(boss_frac)
                                    .fill(bar_color)
                                    .desired_width(170.0),
                            );
                        });
                    }
                }
            });
        });

    // Panel Inferior Elevado (Safe Area para la barra de navegación de Android)
    if !current_player.active_skills.is_empty() {
        egui::Area::new(egui::Id::new("active_skill_touch_area"))
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(20.0, -85.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    let active_skills = current_player.active_skills.clone();
                    for (idx, &skill) in active_skills.iter().enumerate() {
                        let is_cd = if idx == 0 {
                            current_player.active_cooldown_1 > 0.0
                        } else {
                            current_player.active_cooldown_2 > 0.0
                        };

                        if let Some(&tex_id) = skill_tex_map.get(&skill) {
                            let frame_color = if is_cd {
                                egui::Color32::from_rgb(30, 35, 45)
                            } else {
                                egui::Color32::from_rgb(0, 220, 255)
                            };

                            egui::Frame::default()
                                .fill(if is_cd {
                                    egui::Color32::from_rgba_unmultiplied(10, 12, 20, 200)
                                } else {
                                    egui::Color32::from_rgba_unmultiplied(20, 40, 70, 220)
                                })
                                .stroke(egui::Stroke::new(
                                    if is_cd { 1.0_f32 } else { 2.5_f32 },
                                    frame_color,
                                ))
                                .rounding(8.0)
                                .inner_margin(4.0)
                                .show(ui, |ui| {
                                    let tint = if is_cd {
                                        egui::Color32::from_gray(100)
                                    } else {
                                        egui::Color32::WHITE
                                    };

                                    let img_btn = ui.add(
                                        egui::Image::new(egui::load::SizedTexture::new(
                                            tex_id,
                                            egui::vec2(58.0, 82.0),
                                        ))
                                        .tint(tint)
                                        .sense(egui::Sense::click()),
                                    );

                                    if img_btn.clicked() && !is_cd {
                                        if idx == 0 {
                                            current_player.active_cooldown_1 = skill.cooldown()
                                                * current_player.cooldown_reduction;
                                        } else {
                                            current_player.active_cooldown_2 = skill.cooldown()
                                                * current_player.cooldown_reduction;
                                        }

                                        match skill {
                                            SkillId::ZeroCostBeam => {
                                                current_player.beam_active_timer = 3.0;
                                                current_player.status_message =
                                                    "ZERO-COST BEAM ACTIVADO (3S)".to_string();
                                                current_player.status_timer = 3.0;
                                                trigger_vibration(200);
                                            }
                                            SkillId::CargoClean => {
                                                trigger_nuke_effect(
                                                    &mut commands,
                                                    &assets,
                                                    &mut current_player,
                                                    &mut screen_shake,
                                                    &mut query_enemies,
                                                    &query_enemy_lasers,
                                                );
                                            }
                                            _ => {}
                                        }
                                    }
                                });
                        }
                        ui.add_space(12.0);
                    }
                });
            });
    }

    // Tres habilidades exclusivas de la nave seleccionada, siempre visibles
    // en la esquina inferior derecha y separadas del draft roguelite. Forman
    // una disposición triangular: una arriba y dos abajo.
    let class_abilities = current_player.ship_class.class_abilities();
    egui::Area::new(egui::Id::new("class_ability_touch_area"))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-12.0, -78.0))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                draw_class_ability_button(
                    ui,
                    0,
                    class_abilities[0],
                    class_ability_tex_map.get(&class_abilities[0]).copied(),
                    time.elapsed_seconds(),
                    allow_pointer_press,
                    &mut current_player,
                    &mut commands,
                    &assets,
                    &mut screen_shake,
                    &mut query_enemies,
                    &query_enemy_lasers,
                    &query_player,
                );
                ui.add_space(-1.0);
                ui.horizontal(|ui| {
                    draw_class_ability_button(
                        ui,
                        1,
                        class_abilities[1],
                        class_ability_tex_map.get(&class_abilities[1]).copied(),
                        time.elapsed_seconds(),
                        allow_pointer_press,
                        &mut current_player,
                        &mut commands,
                        &assets,
                        &mut screen_shake,
                        &mut query_enemies,
                        &query_enemy_lasers,
                        &query_player,
                    );
                    ui.add_space(4.0);
                    draw_class_ability_button(
                        ui,
                        2,
                        class_abilities[2],
                        class_ability_tex_map.get(&class_abilities[2]).copied(),
                        time.elapsed_seconds(),
                        allow_pointer_press,
                        &mut current_player,
                        &mut commands,
                        &assets,
                        &mut screen_shake,
                        &mut query_enemies,
                        &query_enemy_lasers,
                        &query_player,
                    );
                });
            });
        });
}

// ============================================================================
// UI CON EGUI: Game Over con Carta / Medalla de Rango de Torneo
// ============================================================================

fn ui_game_over(
    mut contexts: EguiContexts,
    current_player: Res<CurrentPlayer>,
    assets: Res<GameAssets>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let rank_s_id = contexts.add_image(assets.rank_s.clone_weak());
    let rank_a_id = contexts.add_image(assets.rank_a.clone_weak());
    let rank_b_id = contexts.add_image(assets.rank_b.clone_weak());
    let rank_c_id = contexts.add_image(assets.rank_c.clone_weak());
    let primary_orange = egui::Color32::from_rgb(255, 140, 0);
    let secondary_white = egui::Color32::WHITE;

    let ctx = contexts.ctx_mut();
    let viewport_height = ctx.screen_rect().height();

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(6, 6, 6)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        // La carta es el elemento principal de la pantalla y conserva
                        // su proporción original (1024 x 1536), aprovechando el ancho
                        // disponible sin empujar los controles fuera de la pantalla.
                        let card_width = (ui.available_width() - 24.0).clamp(220.0, 340.0);
                        let card_height = card_width * 1.5;

                        // Centra verticalmente la tarjeta junto con sus controles inferiores.
                        // El valor se calcula con la altura real de la pantalla para adaptarse
                        // a distintos celulares y orientaciones.
                        let content_height = card_height + 16.0 + 42.0 + 12.0 + 24.0 + 20.0;
                        let top_space = ((viewport_height - content_height) * 0.5).max(8.0);
                        ui.add_space(top_space);

                        let rank_tex_id = if current_player.score >= 200000 {
                            rank_s_id
                        } else if current_player.score >= 75000 {
                            rank_a_id
                        } else if current_player.score >= 25000 {
                            rank_b_id
                        } else {
                            rank_c_id
                        };

                        // Solo la imagen de la tarjeta: sin borde, fondo, margen ni padding.
                        ui.add(egui::Image::new(egui::load::SizedTexture::new(
                            rank_tex_id,
                            egui::vec2(card_width, card_height),
                        )));

                        ui.add_space(16.0);

                        let btn_retry = ui.add(
                            egui::Button::new(
                                egui::RichText::new("MENÚ")
                                    .size(15.0)
                                    .color(secondary_white)
                                    .strong(),
                            )
                            .min_size(egui::vec2(280.0, 42.0))
                            .fill(primary_orange),
                        );

                        if btn_retry.clicked() {
                            next_state.set(AppState::NameInput);
                        }

                        ui.add_space(12.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "PUNTAJE FINAL: {} PTS",
                                current_player.score
                            ))
                            .size(18.0)
                            .color(secondary_white)
                            .strong(),
                        );
                        ui.add_space(20.0);
                    });
                });
        });
}
