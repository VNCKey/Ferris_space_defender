use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::Rng;
use serde::{Deserialize, Serialize};
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

#[derive(Resource)]
pub struct CurrentPlayer {
    pub name: String,
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
}

impl Default for CurrentPlayer {
    fn default() -> Self {
        Self {
            name: String::new(),
            score: 0,
            health: 100.0,
            max_health: 100.0,
            shield: 50.0,
            max_shield: 100.0,
            triple_shot_timer: 0.0,
            wave: 1,
            time_elapsed: 0.0,
            enemies_killed: 0,
            status_message: String::new(),
            status_timer: 0.0,
            shield_hit_timer: 0.0,
            hull_hit_timer: 0.0,
            invulnerable_timer: 0.0,
        }
    }
}

// ============================================================================
// Tipos de Habilidades / PowerUps
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PowerUpType {
    TripleShot, // Disparo triple en abanico
    Shield,     // Escudo de energia protector
    Health,     // Reparacion de casco
    Nuke,       // Bomba que limpia la pantalla
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
    pub regular_enemies: Vec<Handle<Image>>,
    pub boss_enemies: Vec<Handle<Image>>,
    pub planets: Vec<Handle<Image>>,
    // Efectos de Sonido
    pub snd_player_laser: Handle<AudioSource>,
    pub snd_player_death: Handle<AudioSource>,
    pub snd_player_damage: Handle<AudioSource>,
    pub snd_enemy_death: Handle<AudioSource>,
    pub snd_boss_death: Handle<AudioSource>,
    pub snd_powerup_pickup: Handle<AudioSource>,
}

#[derive(Resource)]
pub struct GameTimers {
    pub enemy_spawn: Timer,
    pub player_shoot: Timer,
    pub wave_timer: Timer,
    pub boss_spawn_timer: Timer,
}

impl Default for GameTimers {
    fn default() -> Self {
        Self {
            enemy_spawn: Timer::from_seconds(1.1, TimerMode::Repeating),
            player_shoot: Timer::from_seconds(0.18, TimerMode::Repeating),
            wave_timer: Timer::from_seconds(16.0, TimerMode::Repeating),
            boss_spawn_timer: Timer::from_seconds(24.0, TimerMode::Repeating),
        }
    }
}

// ============================================================================
// Componentes
// ============================================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerShieldVisual;

#[derive(Component)]
pub struct Laser {
    pub velocity: Vec2,
}

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
    pub shoot_timer: f32,
    pub dir_x: f32,
}

#[derive(Component)]
pub struct PowerUpItem {
    pub kind: PowerUpType,
    pub speed: f32,
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
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup_app)
        .add_systems(Update, ui_name_input.run_if(in_state(AppState::NameInput)))
        .add_systems(Update, ui_game_over.run_if(in_state(AppState::GameOver)))
        .add_systems(OnEnter(AppState::Playing), setup_playing)
        .add_systems(
            Update,
            (
                player_input_system,
                player_shoot_system,
                laser_movement_system,
                enemy_laser_system,
                enemy_spawn_system,
                boss_spawn_system,
                enemy_movement_system,
                powerup_system,
                collision_system,
                particle_system,
                planet_system,
                star_system,
                difficulty_and_wave_system,
                shield_visual_system,
                ui_playing_hud,
            )
                .run_if(in_state(AppState::Playing)),
        )
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

    commands.spawn(Camera2dBundle::default());

    let player_ship = asset_server.load("textures/player/ship.png");
    let shield_dome = asset_server.load("textures/effects/shield_dome.png");
    let powerup_triple = asset_server.load("textures/powerups/triple.png");
    let powerup_shield = asset_server.load("textures/powerups/shield.png");
    let powerup_health = asset_server.load("textures/powerups/health.png");
    let powerup_nuke = asset_server.load("textures/powerups/nuke.png");

    // Cargar efectos de sonido en formato OGG
    let snd_player_laser = asset_server.load("audio/laser_player.ogg");
    let snd_player_death = asset_server.load("audio/player_death.ogg");
    let snd_player_damage = asset_server.load("audio/player_damage.ogg");
    let snd_enemy_death = asset_server.load("audio/enemy_death.ogg");
    let snd_boss_death = asset_server.load("audio/boss_death.ogg");
    let snd_powerup_pickup = asset_server.load("audio/powerup_pickup.ogg");

    // 7 Enemigos regulares (enemy_1 a enemy_7)
    let mut regular_enemies = Vec::new();
    for i in 1..=7 {
        regular_enemies.push(asset_server.load(format!("textures/enemies/enemy_{}.png", i)));
    }

    // 3 Jefes Nodriza Gigantes (enemy_8, enemy_9, enemy_10)
    let mut boss_enemies = Vec::new();
    for i in 8..=10 {
        boss_enemies.push(asset_server.load(format!("textures/enemies/enemy_{}.png", i)));
    }

    // 3 Planetas espaciales de fondo
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
        regular_enemies,
        boss_enemies,
        planets,
        snd_player_laser,
        snd_player_death,
        snd_player_damage,
        snd_enemy_death,
        snd_boss_death,
        snd_powerup_pickup,
    });

    // Campo de estrellas cósmicas con 3 capas de profundidad
    let mut rng = rand::thread_rng();
    for _ in 0..130 {
        let x = rng.gen_range(-380.0..380.0);
        let y = rng.gen_range(-650.0..650.0);
        let tier = rng.gen_range(0..10);
        let (speed, size, alpha) = if tier < 6 {
            // Capa lejana: estrellas distantes y tenues
            (rng.gen_range(16.0..35.0), rng.gen_range(1.2..2.0), rng.gen_range(0.35..0.65))
        } else if tier < 9 {
            // Capa media: estrellas nítidas de velocidad media
            (rng.gen_range(40.0..75.0), rng.gen_range(2.0..2.8), rng.gen_range(0.70..0.90))
        } else {
            // Capa cercana: polvo cósmico veloz y brillante
            (rng.gen_range(85.0..140.0), rng.gen_range(2.8..4.0), rng.gen_range(0.85..1.0))
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.88, 0.94, 1.0, alpha),
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, -20.0),
                ..default()
            },
            BackgroundStar { speed },
        ));
    }
}

// ============================================================================
// Setup Playing
// ============================================================================

fn setup_playing(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut timers: ResMut<GameTimers>,
) {
    current_player.health = current_player.max_health;
    current_player.shield = 50.0;
    current_player.triple_shot_timer = 0.0;
    current_player.score = 0;
    current_player.wave = 1;
    current_player.time_elapsed = 0.0;
    current_player.enemies_killed = 0;
    current_player.status_message = "MISION INICIADA".to_string();
    current_player.status_timer = 2.5;
    current_player.invulnerable_timer = 0.0;

    timers.enemy_spawn.reset();
    timers.player_shoot.reset();
    timers.wave_timer.reset();
    timers.boss_spawn_timer.reset();

    // Nave Principal
    commands.spawn((
        SpriteBundle {
            texture: assets.player_ship.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(95.0, 75.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -260.0, 10.0),
            ..default()
        },
        Player,
    ));

    // Escudo de impacto (oculto por defecto, solo destella al recibir golpes)
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
    query_planets: Query<Entity, With<BackgroundPlanet>>,
) {
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
    for e in query_powerups.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_planets.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ============================================================================
// Sistemas de Control y Disparos
// ============================================================================

fn player_input_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    windows: Query<&Window>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let dt = time.delta_seconds();
        let speed = 440.0;

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

        if let Some(touch) = touches.first_pressed_position() {
            if let Ok(window) = windows.get_single() {
                let target_x = touch.x - window.width() * 0.5;
                let target_y = window.height() * 0.5 - touch.y + 45.0;
                transform.translation.x = transform.translation.x.lerp(target_x, (dt * 20.0).min(1.0));
                transform.translation.y = transform.translation.y.lerp(target_y, (dt * 20.0).min(1.0));
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
    mut shield_query: Query<(&mut Transform, &mut Visibility, &mut Sprite), With<PlayerShieldVisual>>,
) {
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
        // Destello rojo en la nave si el golpe dañó directamente el casco
        if current_player.hull_hit_timer > 0.0 {
            player_sprite.color = Color::srgb(1.0, 0.35, 0.35);
        } else {
            player_sprite.color = Color::WHITE;
        }

        if let Ok((mut shield_tr, mut visibility, mut shield_sprite)) = shield_query.get_single_mut() {
            shield_tr.translation = player_tr.translation;
            shield_tr.translation.z = 9.5;

            // OPCION B: El campo de fuerza solo es visible con un destello reactivo al recibir impacto
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

fn player_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
    query: Query<&Transform, With<Player>>,
) {
    timers.player_shoot.tick(time.delta());

    if current_player.triple_shot_timer > 0.0 {
        current_player.triple_shot_timer -= time.delta_seconds();
    }

    if timers.player_shoot.just_finished() {
        if let Ok(player_tr) = query.get_single() {
            play_sound(&mut commands, assets.snd_player_laser.clone(), 0.22);
            let pos = player_tr.translation;

            if current_player.triple_shot_timer > 0.0 {
                // DISPARO TRIPLE ACTIVO (3 proyectiles en abanico)
                let angles: &[(f32, f32)] = &[(-0.18, -14.0), (0.0, 0.0), (0.18, 14.0)];
                for &(angle, offset_x) in angles {
                    let vx = angle.sin() * 900.0;
                    let vy = angle.cos() * 900.0;
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 0.2, 0.9), // Plasma Magenta
                                custom_size: Some(Vec2::new(7.0, 26.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(pos.x + offset_x, pos.y + 24.0, 8.0),
                            ..default()
                        },
                        Laser {
                            velocity: Vec2::new(vx, vy),
                        },
                    ));
                }
            } else {
                // Disparo Doble Estandar
                for offset_x in &[-18.0, 18.0] {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.15, 0.85, 1.0),
                                custom_size: Some(Vec2::new(7.0, 26.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(pos.x + offset_x, pos.y + 24.0, 8.0),
                            ..default()
                        },
                        Laser {
                            velocity: Vec2::new(0.0, 880.0),
                        },
                    ));
                }
            }
        }
    }
}

fn laser_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Laser)>,
) {
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
    mut query: Query<(Entity, &mut Transform, &EnemyLaser)>,
) {
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
) {
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
        let health = (35.0 + (current_player.wave as f32 * 14.0)) * (1.0 + current_player.time_elapsed * 0.004);
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
                shoot_timer: rng.gen_range(1.5..3.0),
                dir_x: if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            },
        ));
    }
}

// Spawn del Jefe Gigante
fn boss_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    existing_bosses: Query<&Enemy>,
) {
    // Si ya hay un jefe en pantalla, no generar otro
    let has_boss = existing_bosses.iter().any(|e| e.is_boss);
    if has_boss {
        return;
    }

    timers.boss_spawn_timer.tick(time.delta());
    if timers.boss_spawn_timer.just_finished() && !assets.boss_enemies.is_empty() {
        let mut rng = rand::thread_rng();
        let boss_idx = rng.gen_range(0..assets.boss_enemies.len());
        let texture = assets.boss_enemies[boss_idx].clone();

        let health = 350.0 + (current_player.wave as f32 * 120.0);
        let size = 175.0; // Jefe Gigante
        let score_val = 1500;

        current_player.status_message = "ALERTA: JEFE NODRIZA DETECTADO".to_string();
        current_player.status_timer = 3.5;

        commands.spawn((
            SpriteBundle {
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, 430.0, 6.0),
                ..default()
            },
            Enemy {
                health,
                max_health: health,
                speed: 65.0,
                score_value: score_val,
                size,
                is_boss: true,
                shoot_timer: 1.6,
                dir_x: 1.0,
            },
        ));
    }
}

fn enemy_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    current_player: Res<CurrentPlayer>,
    mut query: Query<(Entity, &mut Transform, &mut Enemy)>,
) {
    let dt = time.delta_seconds();
    let mut rng = rand::thread_rng();

    for (entity, mut transform, mut enemy) in query.iter_mut() {
        if enemy.is_boss {
            // Movimiento del Jefe: Baja hasta la parte superior y se mueve de lado a lado
            if transform.translation.y > 270.0 {
                transform.translation.y -= enemy.speed * dt;
            } else {
                transform.translation.x += enemy.dir_x * 120.0 * dt;
                if transform.translation.x > 210.0 {
                    enemy.dir_x = -1.0;
                } else if transform.translation.x < -210.0 {
                    enemy.dir_x = 1.0;
                }
            }

            // Disparo del Jefe: Ráfaga de láseres rojos
            enemy.shoot_timer -= dt;
            if enemy.shoot_timer <= 0.0 {
                enemy.shoot_timer = 1.3;
                let boss_pos = transform.translation;
                for &offset in &[-35.0, 0.0, 35.0] {
                    commands.spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(1.0, 0.2, 0.1), // Láser rojo enemigo
                                custom_size: Some(Vec2::new(8.0, 24.0)),
                                ..default()
                            },
                            transform: Transform::from_xyz(boss_pos.x + offset, boss_pos.y - 45.0, 7.0),
                            ..default()
                        },
                        EnemyLaser {
                            velocity: Vec2::new(offset * 2.0, -380.0),
                        },
                    ));
                }
            }
        } else {
            // Movimiento de Enemigo Normal
            transform.translation.y -= enemy.speed * dt;
            transform.translation.x += (transform.translation.y * 0.02).sin() * 55.0 * dt;

            // Disparo ocasional de enemigos avanzados (después de 45 segundos)
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
                            transform: Transform::from_xyz(transform.translation.x, transform.translation.y - 20.0, 7.0),
                            ..default()
                        },
                        EnemyLaser {
                            velocity: Vec2::new(0.0, -340.0),
                        },
                    ));
                }
            }

            // Si sobrepasa la parte inferior de la pantalla, desaparece sin dañar al jugador
            if transform.translation.y < -430.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}

// ============================================================================
// Dificultad Acelerada y Oleadas (Evita partidas infinitas)
// ============================================================================

fn difficulty_and_wave_system(
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
) {
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

        // Aceleracion exponencial de spawns
        let new_interval = (1.1 - (current_player.time_elapsed * 0.005) - (current_player.wave as f32 * 0.06)).max(0.32);
        timers.enemy_spawn.set_duration(std::time::Duration::from_secs_f32(new_interval));
    }
}

// ============================================================================
// PowerUps (Habilidades flotantes)
// ============================================================================

fn powerup_system(
    mut commands: Commands,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut query_powerups: Query<(Entity, &mut Transform, &PowerUpItem)>,
    query_player: Query<&Transform, (With<Player>, Without<PowerUpItem>, Without<Enemy>)>,
    mut query_enemies: Query<(Entity, &mut Enemy, &Transform), (Without<Player>, Without<PowerUpItem>)>,
    query_enemy_lasers: Query<Entity, With<EnemyLaser>>,
) {
    let dt = time.delta_seconds();

    for (p_entity, mut p_tr, powerup) in query_powerups.iter_mut() {
        p_tr.translation.y -= powerup.speed * dt;
        p_tr.rotate_z(1.4 * dt);
        let pulse = (time.elapsed_seconds() * 4.0).sin() * 0.08 + 1.0;
        p_tr.scale = Vec3::splat(pulse);

        if let Ok(player_tr) = query_player.get_single() {
            let p_pos = player_tr.translation.truncate();
            let item_pos = p_tr.translation.truncate();

            // Colision con el jugador
            if p_pos.distance(item_pos) < 46.0 {
                match powerup.kind {
                    PowerUpType::TripleShot => {
                        current_player.triple_shot_timer = 12.0;
                        current_player.status_message = "DISPARO TRIPLE OBTENIDO".to_string();
                    }
                    PowerUpType::Shield => {
                        current_player.shield = (current_player.shield + 50.0).min(current_player.max_shield);
                        current_player.shield_hit_timer = 0.45; // Destello de recarga
                        current_player.status_message = "ESCUDO RECARGADO".to_string();
                    }
                    PowerUpType::Health => {
                        current_player.health = (current_player.health + 40.0).min(current_player.max_health);
                        current_player.status_message = "CASCO REPARADO +40".to_string();
                    }
                    PowerUpType::Nuke => {
                        // 1. Limpiar todos los lásers enemigos de la pantalla
                        for laser_e in query_enemy_lasers.iter() {
                            commands.entity(laser_e).despawn();
                        }

                        // 2. Destruir enemigos comunes y aplicar daño masivo (150) a Jefes
                        let mut hit_boss = false;
                        for (enemy_e, mut enemy, enemy_tr) in query_enemies.iter_mut() {
                            let e_pos = enemy_tr.translation.truncate();
                            if enemy.is_boss {
                                hit_boss = true;
                                enemy.health -= 150.0;
                                spawn_bevy_explosion(&mut commands, e_pos);
                                if enemy.health <= 0.0 {
                                    play_sound(&mut commands, assets.snd_boss_death.clone(), 0.90);
                                    current_player.score += 1500;
                                    current_player.enemies_killed += 1;
                                    current_player.status_message = "JEFE ANIQUILADO CON EMP (+1500 PTS)".to_string();
                                    commands.entity(enemy_e).despawn();
                                } else {
                                    play_sound(&mut commands, assets.snd_enemy_death.clone(), 0.70);
                                    current_player.status_message = "EMP IMPACTO AL JEFE (-150 HP)".to_string();
                                }
                            } else {
                                spawn_bevy_explosion(&mut commands, e_pos);
                                play_sound(&mut commands, assets.snd_enemy_death.clone(), 0.45);
                                current_player.score += 75;
                                current_player.enemies_killed += 1;
                                commands.entity(enemy_e).despawn();
                            }
                        }

                        spawn_bevy_explosion(&mut commands, Vec2::ZERO);
                        if !hit_boss {
                            current_player.score += 300;
                            current_player.status_message = "BOMBA EMP DETONADA".to_string();
                        }
                    }
                }
                play_sound(&mut commands, assets.snd_powerup_pickup.clone(), 0.75);
                current_player.status_timer = 2.5;
                spawn_spark(&mut commands, item_pos);
                commands.entity(p_entity).despawn();
                continue;
            }
        }

        if p_tr.translation.y < -440.0 {
            commands.entity(p_entity).despawn();
        }
    }
}

// Spawnea una capsula de PowerUp brillante en el mundo
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
                custom_size: Some(Vec2::splat(38.0)),
                ..default()
            },
            transform: Transform::from_xyz(pos.x, pos.y, 8.5),
            ..default()
        },
        PowerUpItem {
            kind,
            speed: 75.0,
        },
    ));
}

// ============================================================================
// Colisiones
// ============================================================================

fn apply_damage_to_player(
    commands: &mut Commands,
    assets: &GameAssets,
    current_player: &mut ResMut<CurrentPlayer>,
    damage: f32,
) {
    if current_player.invulnerable_timer > 0.0 {
        return;
    }
    current_player.invulnerable_timer = 0.35; // 350ms de gracia tras recibir impacto
    play_sound(commands, assets.snd_player_damage.clone(), 0.65);
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
    mut next_state: ResMut<NextState<AppState>>,
    mut leaderboard: ResMut<Leaderboard>,
    mut query_enemies: Query<(Entity, &Transform, &mut Enemy)>,
    query_lasers: Query<(Entity, &Transform), With<Laser>>,
    query_enemy_lasers: Query<(Entity, &Transform), With<EnemyLaser>>,
    query_player: Query<&Transform, With<Player>>,
) {
    let mut rng = rand::thread_rng();

    // 1. Láseres del Jugador vs Enemigos / Jefes
    for (laser_entity, laser_tr) in query_lasers.iter() {
        let l_pos = laser_tr.translation.truncate();

        for (enemy_entity, enemy_tr, mut enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            let hit_radius = enemy.size * 0.45;

            if l_pos.distance(e_pos) < hit_radius {
                commands.entity(laser_entity).despawn();
                enemy.health -= 35.0;

                if enemy.health <= 0.0 {
                    current_player.score += enemy.score_value;
                    current_player.enemies_killed += 1;

                    // Explosión de partículas Bevy
                    spawn_bevy_explosion(&mut commands, e_pos);

                    // Soltar PowerUp con probabilidad (o 100% si era Jefe)
                    let should_drop = enemy.is_boss || rng.gen_bool(0.24);
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
                        play_sound(&mut commands, assets.snd_boss_death.clone(), 0.90);
                        current_player.status_message = "JEFE DERROTADO (+1500 PTS)".to_string();
                        current_player.status_timer = 3.0;
                    } else {
                        play_sound(&mut commands, assets.snd_enemy_death.clone(), 0.55);
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

        // 2. Proyectiles Enemigos vs Jugador
        for (elaser_entity, elaser_tr) in query_enemy_lasers.iter() {
            let el_pos = elaser_tr.translation.truncate();
            if p_pos.distance(el_pos) < 32.0 {
                commands.entity(elaser_entity).despawn();
                apply_damage_to_player(&mut commands, &assets, &mut current_player, 16.0);
                spawn_spark(&mut commands, el_pos);
            }
        }

        // 3. Enemigo vs Jugador (Colisión directa de naves)
        for (enemy_entity, enemy_tr, enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            if p_pos.distance(e_pos) < (32.0 + enemy.size * 0.35) {
                let dmg = if enemy.is_boss { 45.0 } else { 24.0 };
                apply_damage_to_player(&mut commands, &assets, &mut current_player, dmg);
                spawn_bevy_explosion(&mut commands, e_pos);
                if !enemy.is_boss {
                    commands.entity(enemy_entity).despawn();
                }
            }
        }
    }

    // Comprobar Game Over
    if current_player.health <= 0.0 {
        current_player.health = 0.0;
        play_sound(&mut commands, assets.snd_player_death.clone(), 0.90);
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

        let color = match rng.gen_range(0..3) {
            0 => Color::srgb(1.0, 0.45, 0.1),
            1 => Color::srgb(1.0, 0.9, 0.2),
            _ => Color::srgb(0.2, 0.8, 1.0),
        };

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(size)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 15.0),
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
    for _ in 0..4 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(40.0..120.0);
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(1.0, 0.9, 0.5),
                    custom_size: Some(Vec2::splat(4.0)),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 16.0),
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

        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;
        particle.velocity *= 0.94;

        let progress = particle.lifetime.fraction_remaining();
        let current_size = particle.initial_size * progress;
        sprite.custom_size = Some(Vec2::splat(current_size));

        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// ============================================================================
// Fondo Espacial (Planetas y Estrellas)
// ============================================================================

fn planet_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<BackgroundPlanet>>,
) {
    let dt = time.delta_seconds();
    let mut rng = rand::thread_rng();
    for mut transform in query.iter_mut() {
        transform.translation.y -= 28.0 * dt;
        if transform.translation.y < -550.0 {
            transform.translation.y = 550.0;
            transform.translation.x = rng.gen_range(-220.0..220.0);
        }
    }
}

fn star_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &BackgroundStar)>,
) {
    let dt = time.delta_seconds();
    for (mut transform, star) in query.iter_mut() {
        transform.translation.y -= star.speed * dt;
        if transform.translation.y < -460.0 {
            transform.translation.y = 460.0;
        }
    }
}

// ============================================================================
// Interfaz de Usuario Limpia (Sin Iconos ni Emojis)
// ============================================================================

fn ui_name_input(
    mut contexts: EguiContexts,
    mut current_player: ResMut<CurrentPlayer>,
    leaderboard: Res<Leaderboard>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(3, 4, 6)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(10.0);
                        ui.heading(
                            egui::RichText::new("FERRIS SPACE DEFENDER")
                                .size(21.0)
                                .color(egui::Color32::from_rgb(255, 110, 40))
                                .strong(),
                        );
                        ui.label(
                            egui::RichText::new("Torneo de Programadores - Rust Peru")
                                .size(13.0)
                                .color(egui::Color32::from_rgb(180, 220, 255)),
                        );
                        ui.add_space(8.0);

                        ui.scope(|ui| {
                            ui.set_max_width(330.0);

                            // Tarjeta de Ingreso de Nombre
                            egui::Frame::default()
                                .fill(egui::Color32::from_rgb(18, 26, 45))
                                .rounding(10.0)
                                .inner_margin(egui::Margin::symmetric(10.0, 12.0))
                                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(45, 65, 105)))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new("Nombre del Concursante:")
                                            .size(13.0)
                                            .color(egui::Color32::WHITE)
                                            .strong(),
                                    );
                                    ui.add_space(4.0);

                                    // Caja de visualización del nombre
                                    egui::Frame::default()
                                        .fill(egui::Color32::from_rgb(10, 15, 28))
                                        .rounding(6.0)
                                        .stroke(egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(255, 180, 50)))
                                        .inner_margin(8.0)
                                        .show(ui, |ui| {
                                            ui.horizontal(|ui| {
                                                let display_name = if current_player.name.is_empty() {
                                                    "Toca las teclas abajo...".to_string()
                                                } else {
                                                    current_player.name.clone()
                                                };
                                                let text_color = if current_player.name.is_empty() {
                                                    egui::Color32::GRAY
                                                } else {
                                                    egui::Color32::from_rgb(255, 220, 80)
                                                };
                                                ui.label(
                                                    egui::RichText::new(display_name)
                                                        .size(17.0)
                                                        .color(text_color)
                                                        .strong(),
                                                );
                                            });
                                        });

                                    ui.add_space(8.0);

                                    // Teclado Arcade
                                    ui.spacing_mut().item_spacing = egui::vec2(2.5, 3.5);

                                    let rows: &[&[&str]] = &[
                                        &["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"],
                                        &["A", "S", "D", "F", "G", "H", "J", "K", "L", "N"],
                                        &["Z", "X", "C", "V", "B", "N", "M", "1", "2", "3"],
                                    ];

                                    for row in rows {
                                        ui.horizontal(|ui| {
                                            for &key in *row {
                                                let btn = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(key)
                                                            .size(13.0)
                                                            .color(egui::Color32::WHITE)
                                                            .strong(),
                                                    )
                                                    .min_size(egui::vec2(27.0, 30.0))
                                                    .fill(egui::Color32::from_rgb(32, 48, 80)),
                                                );
                                                if btn.clicked() && current_player.name.len() < 14 {
                                                    current_player.name.push_str(key);
                                                }
                                            }
                                        });
                                    }

                                    ui.add_space(4.0);

                                    ui.horizontal(|ui| {
                                        if ui.add(
                                            egui::Button::new(egui::RichText::new("Borrar").size(12.0).color(egui::Color32::WHITE).strong())
                                                .min_size(egui::vec2(85.0, 30.0))
                                                .fill(egui::Color32::from_rgb(180, 50, 50)),
                                        ).clicked() {
                                            current_player.name.pop();
                                        }
                                        if ui.add(
                                            egui::Button::new(egui::RichText::new("Espacio").size(12.0).color(egui::Color32::WHITE).strong())
                                                .min_size(egui::vec2(85.0, 30.0))
                                                .fill(egui::Color32::from_rgb(50, 75, 115)),
                                        ).clicked() && current_player.name.len() < 14 {
                                            current_player.name.push(' ');
                                        }
                                        if ui.add(
                                            egui::Button::new(egui::RichText::new("Aleatorio").size(12.0).color(egui::Color32::WHITE).strong())
                                                .min_size(egui::vec2(95.0, 30.0))
                                                .fill(egui::Color32::from_rgb(40, 130, 90)),
                                        ).clicked() {
                                            let nicknames = ["Ferris_Dev", "Rustacean", "Async_Pro", "Borrow_King", "Cargo_Run", "Lima_Coder", "Byte_Master"];
                                            let mut rng = rand::thread_rng();
                                            current_player.name = nicknames[rng.gen_range(0..nicknames.len())].to_string();
                                        }
                                    });

                                    ui.add_space(10.0);

                                    let btn_start = ui.add(
                                        egui::Button::new(
                                            egui::RichText::new("EMPEZAR A JUGAR")
                                                .size(16.0)
                                                .color(egui::Color32::WHITE)
                                                .strong(),
                                        )
                                        .min_size(egui::vec2(280.0, 44.0))
                                        .fill(egui::Color32::from_rgb(220, 50, 40)),
                                    );

                                    if btn_start.clicked() {
                                        if current_player.name.trim().is_empty() {
                                            current_player.name = "Piloto_Rust".to_string();
                                        }
                                        next_state.set(AppState::Playing);
                                    }
                                });

                            ui.add_space(14.0);

                            // Leaderboard
                            ui.heading(
                                egui::RichText::new("TABLA DE LIDERES (TOP 8)")
                                    .size(15.0)
                                    .color(egui::Color32::from_rgb(255, 215, 0)),
                            );
                            ui.add_space(5.0);

                            egui::Frame::default()
                                .fill(egui::Color32::from_rgb(14, 20, 35))
                                .rounding(8.0)
                                .inner_margin(10.0)
                                .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(35, 50, 80)))
                                .show(ui, |ui| {
                                    egui::Grid::new("leaderboard_grid")
                                        .striped(true)
                                        .min_col_width(62.0)
                                        .show(ui, |ui| {
                                            ui.label(egui::RichText::new("Pos").strong().color(egui::Color32::GRAY));
                                            ui.label(egui::RichText::new("Estudiante").strong().color(egui::Color32::WHITE));
                                            ui.label(egui::RichText::new("Puntaje").strong().color(egui::Color32::from_rgb(255, 200, 50)));
                                            ui.label(egui::RichText::new("Oleada").strong().color(egui::Color32::LIGHT_BLUE));
                                            ui.end_row();

                                            for (idx, entry) in leaderboard.entries.iter().take(8).enumerate() {
                                                ui.label(format!("{}.", idx + 1));
                                                ui.label(&entry.name);
                                                ui.label(format!("{} pts", entry.score));
                                                ui.label(format!("Ola {}", entry.wave));
                                                ui.end_row();
                                            }
                                        });
                                });
                            ui.add_space(20.0);
                        });
                    });
                });
        });
}

fn ui_playing_hud(
    mut contexts: EguiContexts,
    current_player: Res<CurrentPlayer>,
    boss_query: Query<&Enemy>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_hud")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgba_unmultiplied(2, 2, 4, 210)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("PILOTO: {}", current_player.name))
                        .color(egui::Color32::WHITE)
                        .strong(),
                );

                ui.separator();

                ui.label(
                    egui::RichText::new(format!("PTS: {}", current_player.score))
                        .color(egui::Color32::from_rgb(255, 215, 50))
                        .strong()
                        .size(15.0),
                );

                ui.separator();

                ui.label(format!("OLA: {}", current_player.wave));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Barra de Vida
                    let health_frac = (current_player.health / current_player.max_health).clamp(0.0, 1.0);
                    let bar_color = if health_frac > 0.5 {
                        egui::Color32::GREEN
                    } else if health_frac > 0.25 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };

                    ui.label(format!("HP {:.0}%", health_frac * 100.0));
                    ui.add(egui::ProgressBar::new(health_frac).fill(bar_color).desired_width(55.0));

                    // Barra de Escudo si está activo
                    if current_player.shield > 0.0 {
                        let shield_frac = (current_player.shield / current_player.max_shield).clamp(0.0, 1.0);
                        ui.label(format!("ESC {:.0}", current_player.shield));
                        ui.add(egui::ProgressBar::new(shield_frac).fill(egui::Color32::from_rgb(80, 200, 255)).desired_width(45.0));
                    }
                });
            });

            // Banner inferior del HUD: Avisos de estado y barra de jefe
            if current_player.status_timer > 0.0 || current_player.triple_shot_timer > 0.0 {
                ui.horizontal(|ui| {
                    if current_player.triple_shot_timer > 0.0 {
                        ui.label(
                            egui::RichText::new(format!("TRIPLE SHOT: {:.1}s", current_player.triple_shot_timer))
                                .color(egui::Color32::from_rgb(255, 80, 255))
                                .strong(),
                        );
                    }
                    if current_player.status_timer > 0.0 {
                        ui.label(
                            egui::RichText::new(&current_player.status_message)
                                .color(egui::Color32::from_rgb(255, 220, 80))
                                .strong(),
                        );
                    }
                });
            }

            // Si hay un Jefe activo, mostrar barra de vida del Jefe
            for enemy in boss_query.iter() {
                if enemy.is_boss {
                    let boss_frac = (enemy.health / enemy.max_health).clamp(0.0, 1.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("JEFE NODRIZA:")
                                .color(egui::Color32::from_rgb(255, 50, 50))
                                .strong(),
                        );
                        ui.add(
                            egui::ProgressBar::new(boss_frac)
                                .fill(egui::Color32::from_rgb(255, 30, 30))
                                .desired_width(180.0),
                        );
                        ui.label(format!("{:.0}%", boss_frac * 100.0));
                    });
                }
            }
        });
}

fn ui_game_over(
    mut contexts: EguiContexts,
    current_player: Res<CurrentPlayer>,
    leaderboard: Res<Leaderboard>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(4, 2, 4)))
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.add_space(15.0);
                        ui.heading(
                            egui::RichText::new("NAVE DESTRUIDA")
                                .size(24.0)
                                .color(egui::Color32::from_rgb(255, 60, 60))
                                .strong(),
                        );
                        ui.add_space(8.0);

                        ui.label(
                            egui::RichText::new(format!("Buen intento, {}", current_player.name))
                                .size(16.0)
                                .color(egui::Color32::WHITE),
                        );

                        ui.label(
                            egui::RichText::new(format!("PUNTAJE FINAL: {} PUNTOS", current_player.score))
                                .size(22.0)
                                .color(egui::Color32::from_rgb(255, 215, 0))
                                .strong(),
                        );

                        ui.label(format!("Sobreviviste: {:.0} segundos | Oleada: {}", current_player.time_elapsed, current_player.wave));

                        ui.add_space(14.0);

                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new("SIGUIENTE CONCURSANTE (Nuevo intento)")
                                        .size(14.0)
                                        .color(egui::Color32::WHITE)
                                        .strong(),
                                )
                                .min_size(egui::vec2(260.0, 42.0))
                                .fill(egui::Color32::from_rgb(30, 140, 230)),
                            )
                            .clicked()
                        {
                            next_state.set(AppState::NameInput);
                        }

                        ui.add_space(18.0);

                        ui.heading(
                            egui::RichText::new("CLASIFICACION GENERAL DEL EVENTO")
                                .size(15.0)
                                .color(egui::Color32::from_rgb(255, 200, 50)),
                        );
                        ui.add_space(6.0);

                        egui::Frame::default()
                            .fill(egui::Color32::from_rgb(25, 18, 26))
                            .rounding(8.0)
                            .inner_margin(10.0)
                            .show(ui, |ui| {
                                egui::Grid::new("gameover_leaderboard_grid")
                                    .striped(true)
                                    .min_col_width(65.0)
                                    .show(ui, |ui| {
                                        ui.label("Pos");
                                        ui.label("Estudiante");
                                        ui.label("Puntaje");
                                        ui.label("Oleada");
                                        ui.end_row();

                                        for (idx, entry) in leaderboard.entries.iter().take(10).enumerate() {
                                            let is_current = entry.name == current_player.name && entry.score == current_player.score;
                                            let name_text = if is_current {
                                                egui::RichText::new(&entry.name).color(egui::Color32::GREEN).strong()
                                            } else {
                                                egui::RichText::new(&entry.name).color(egui::Color32::WHITE)
                                            };

                                            ui.label(format!("{}.", idx + 1));
                                            ui.label(name_text);
                                            ui.label(format!("{} pts", entry.score));
                                            ui.label(format!("Ola {}", entry.wave));
                                            ui.end_row();
                                        }
                                    });
                            });
                        ui.add_space(20.0);
                    });
                });
        });
}
