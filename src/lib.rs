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
        // Registros de muestra si está vacío
        Self {
            entries: vec![
                ScoreEntry {
                    name: "Ferris_Pro".to_string(),
                    score: 3500,
                    wave: 5,
                    date: "Top 1".to_string(),
                },
                ScoreEntry {
                    name: "Rustacean_PE".to_string(),
                    score: 2200,
                    wave: 3,
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
                "Anónimo".to_string()
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
    pub wave: u32,
    pub enemies_killed: u32,
}

impl Default for CurrentPlayer {
    fn default() -> Self {
        Self {
            name: String::new(),
            score: 0,
            health: 100.0,
            max_health: 100.0,
            wave: 1,
            enemies_killed: 0,
        }
    }
}

// ============================================================================
// Texturas y Recursos del Juego
// ============================================================================

#[derive(Resource)]
pub struct GameAssets {
    pub player_ship: Handle<Image>,
    pub enemies: Vec<Handle<Image>>,
    pub planets: Vec<Handle<Image>>,
}

// Timers para generación de eventos
#[derive(Resource)]
pub struct GameTimers {
    pub enemy_spawn: Timer,
    pub player_shoot: Timer,
    pub wave_timer: Timer,
}

impl Default for GameTimers {
    fn default() -> Self {
        Self {
            enemy_spawn: Timer::from_seconds(1.1, TimerMode::Repeating),
            player_shoot: Timer::from_seconds(0.18, TimerMode::Repeating),
            wave_timer: Timer::from_seconds(18.0, TimerMode::Repeating),
        }
    }
}

// ============================================================================
// Componentes
// ============================================================================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub score_value: u32,
    pub size: f32,
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
                title: "🦀 Ferris Space Defender - Rust Perú 🇵🇪".to_string(),
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
                enemy_spawn_system,
                enemy_movement_system,
                collision_system,
                particle_system,
                planet_system,
                star_system,
                wave_system,
                ui_playing_hud,
            )
                .run_if(in_state(AppState::Playing)),
        )
        .add_systems(OnExit(AppState::Playing), cleanup_playing)
        .run();
}

fn setup_app(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Cámara 2D
    commands.spawn(Camera2dBundle::default());

    // Cargar texturas del jugador, enemigos y planetas
    let player_ship = asset_server.load("textures/player/ship.png");

    let mut enemies = Vec::new();
    for i in 1..=10 {
        enemies.push(asset_server.load(format!("textures/enemies/enemy_{}.png", i)));
    }

    let mut planets = Vec::new();
    for i in 1..=3 {
        planets.push(asset_server.load(format!("textures/backgrounds/planet_{}.png", i)));
    }

    commands.insert_resource(GameAssets {
        player_ship,
        enemies,
        planets,
    });

    // Crear campo de estrellas cósmicas de fondo
    let mut rng = rand::thread_rng();
    for _ in 0..90 {
        let x = rng.gen_range(-380.0..380.0);
        let y = rng.gen_range(-650.0..650.0);
        let speed = rng.gen_range(25.0..80.0);
        let size = rng.gen_range(1.5..3.2);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgba(0.8, 0.9, 1.0, rng.gen_range(0.3..0.95)),
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
// Inicio de Partida (Setup Playing)
// ============================================================================

fn setup_playing(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut current_player: ResMut<CurrentPlayer>,
    mut timers: ResMut<GameTimers>,
) {
    // Reiniciar estadísticas
    current_player.health = current_player.max_health;
    current_player.score = 0;
    current_player.wave = 1;
    current_player.enemies_killed = 0;

    timers.enemy_spawn.reset();
    timers.player_shoot.reset();
    timers.wave_timer.reset();

    // 1. Nave Principal (con tu textura real)
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

    // 2. Planetas de fondo flotando
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
    query_enemies: Query<Entity, With<Enemy>>,
    query_lasers: Query<Entity, With<Laser>>,
    query_particles: Query<Entity, With<ExplosionParticle>>,
    query_planets: Query<Entity, With<BackgroundPlanet>>,
) {
    for e in query_player.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_enemies.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_lasers.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_particles.iter() {
        commands.entity(e).despawn_recursive();
    }
    for e in query_planets.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ============================================================================
// Sistemas de Gameplay (Movimiento, Láseres, Enemigos)
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
        let speed = 420.0;

        // 1. Control con Teclado (Desktop)
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

        // 2. Control Táctil (Mobile / Touch Arrastre)
        if let Some(touch) = touches.first_pressed_position() {
            if let Ok(window) = windows.get_single() {
                // Convertir posición del touch a coordenadas Bevy 2D (origen en el centro)
                let target_x = touch.x - window.width() * 0.5;
                let target_y = window.height() * 0.5 - touch.y + 40.0; // Offset para que el dedo no tape la nave
                transform.translation.x = transform.translation.x.lerp(target_x, (dt * 18.0).min(1.0));
                transform.translation.y = transform.translation.y.lerp(target_y, (dt * 18.0).min(1.0));
            }
        }

        // Limitar dentro de la pantalla
        transform.translation.x = transform.translation.x.clamp(-260.0, 260.0);
        transform.translation.y = transform.translation.y.clamp(-380.0, 320.0);
    }
}

fn player_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    query: Query<&Transform, With<Player>>,
) {
    timers.player_shoot.tick(time.delta());
    if timers.player_shoot.just_finished() {
        if let Ok(player_tr) = query.get_single() {
            let pos = player_tr.translation;

            // Disparo doble de plasma cian/azul
            for offset_x in &[-18.0, 18.0] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(0.15, 0.85, 1.0), // Láser neón
                            custom_size: Some(Vec2::new(7.0, 26.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(pos.x + offset_x, pos.y + 22.0, 8.0),
                        ..default()
                    },
                    Laser,
                ));
            }
        }
    }
}

fn laser_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform), With<Laser>>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform) in query.iter_mut() {
        transform.translation.y += 850.0 * dt;
        if transform.translation.y > 450.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    assets: Res<GameAssets>,
    current_player: Res<CurrentPlayer>,
) {
    timers.enemy_spawn.tick(time.delta());
    if timers.enemy_spawn.just_finished() {
        if assets.enemies.is_empty() {
            return;
        }

        let mut rng = rand::thread_rng();
        let enemy_idx = rng.gen_range(0..assets.enemies.len());
        let texture = assets.enemies[enemy_idx].clone();

        let x = rng.gen_range(-240.0..240.0);
        let base_speed = rng.gen_range(110.0..190.0) + (current_player.wave as f32 * 14.0);
        let health = 30.0 + (current_player.wave as f32 * 12.0);
        let size = rng.gen_range(65.0..88.0);
        let score_val = 100 + (enemy_idx as u32 * 20);

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
                speed: base_speed,
                score_value: score_val,
                size,
            },
        ));
    }
}

fn enemy_movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut current_player: ResMut<CurrentPlayer>,
    mut query: Query<(Entity, &mut Transform, &Enemy)>,
) {
    let dt = time.delta_seconds();
    for (entity, mut transform, enemy) in query.iter_mut() {
        transform.translation.y -= enemy.speed * dt;

        // Leve balanceo
        transform.translation.x += (transform.translation.y * 0.02).sin() * 45.0 * dt;

        // Si sobrepasa la parte inferior, daña al jugador
        if transform.translation.y < -430.0 {
            current_player.health -= 12.0;
            commands.entity(entity).despawn();
        }
    }
}

fn wave_system(
    time: Res<Time>,
    mut timers: ResMut<GameTimers>,
    mut current_player: ResMut<CurrentPlayer>,
) {
    timers.wave_timer.tick(time.delta());
    if timers.wave_timer.just_finished() {
        current_player.wave += 1;
        // Spawns más frecuentes por oleada
        let new_dur = (1.1 - (current_player.wave as f32 * 0.08)).max(0.4);
        timers.enemy_spawn.set_duration(std::time::Duration::from_secs_f32(new_dur));
    }
}

// ============================================================================
// Colisiones y Sistema de Explosiones Bevy
// ============================================================================

fn collision_system(
    mut commands: Commands,
    mut current_player: ResMut<CurrentPlayer>,
    mut next_state: ResMut<NextState<AppState>>,
    mut leaderboard: ResMut<Leaderboard>,
    mut query_enemies: Query<(Entity, &Transform, &mut Enemy)>,
    query_lasers: Query<(Entity, &Transform), With<Laser>>,
    query_player: Query<&Transform, With<Player>>,
) {
    // 1. Láser vs Enemigo
    for (laser_entity, laser_tr) in query_lasers.iter() {
        let l_pos = laser_tr.translation.truncate();

        for (enemy_entity, enemy_tr, mut enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            let hit_radius = enemy.size * 0.45;

            if l_pos.distance(e_pos) < hit_radius {
                // Destruir el láser
                commands.entity(laser_entity).despawn();

                // Dañar al enemigo
                enemy.health -= 35.0;

                if enemy.health <= 0.0 {
                    // ¡Enemigo destruido!
                    current_player.score += enemy.score_value;
                    current_player.enemies_killed += 1;

                    // 💥 Explosión de partículas Bevy
                    spawn_bevy_explosion(&mut commands, e_pos);

                    commands.entity(enemy_entity).despawn();
                } else {
                    // Chispas de impacto
                    spawn_spark(&mut commands, l_pos);
                }
                break;
            }
        }
    }

    // 2. Enemigo vs Nave del Jugador
    if let Ok(player_tr) = query_player.get_single() {
        let p_pos = player_tr.translation.truncate();

        for (enemy_entity, enemy_tr, enemy) in query_enemies.iter_mut() {
            let e_pos = enemy_tr.translation.truncate();
            if p_pos.distance(e_pos) < (35.0 + enemy.size * 0.35) {
                current_player.health -= 25.0;
                spawn_bevy_explosion(&mut commands, e_pos);
                commands.entity(enemy_entity).despawn();
            }
        }
    }

    // Comprobar Game Over
    if current_player.health <= 0.0 {
        current_player.health = 0.0;
        leaderboard.add_score(
            current_player.name.clone(),
            current_player.score,
            current_player.wave,
        );
        next_state.set(AppState::GameOver);
    }
}

// Genera una explosión radial de partículas con Bevy ECS
fn spawn_bevy_explosion(commands: &mut Commands, pos: Vec2) {
    let mut rng = rand::thread_rng();

    // 25 chispas de fuego y plasma
    for _ in 0..26 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(90.0..320.0);
        let size = rng.gen_range(5.0..11.0);
        let life = rng.gen_range(0.35..0.7);

        let color = match rng.gen_range(0..3) {
            0 => Color::srgb(1.0, 0.45, 0.1), // Naranja ardiente
            1 => Color::srgb(1.0, 0.9, 0.2),  // Amarillo brillante
            _ => Color::srgb(0.2, 0.8, 1.0),  // Energía cian
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

        // Movimiento e inercia
        transform.translation.x += particle.velocity.x * dt;
        transform.translation.y += particle.velocity.y * dt;

        // Fricción
        particle.velocity *= 0.94;

        // Reducir escala y alpha progresivamente
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
        // Cuando el planeta sale por abajo, reaparece arriba
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
// Interfaz de Usuario con bevy_egui
// ============================================================================

fn ui_name_input(
    mut contexts: EguiContexts,
    mut current_player: ResMut<CurrentPlayer>,
    leaderboard: Res<Leaderboard>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(8, 12, 22)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(25.0);
                ui.heading(
                    egui::RichText::new("🦀 FERRIS SPACE DEFENDER 🚀")
                        .size(24.0)
                        .color(egui::Color32::from_rgb(255, 110, 40))
                        .strong(),
                );
                ui.label(
                    egui::RichText::new("¡Compite por el mejor puntaje y gana un polo de Rust Perú! 🇵🇪")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(180, 220, 255)),
                );
                ui.add_space(20.0);

                // Tarjeta de Ingreso de Nombre
                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(18, 26, 45))
                    .rounding(10.0)
                    .inner_margin(16.0)
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new("👤 Ingresa tu Nombre / Alias:")
                                .size(15.0)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        );
                        ui.add_space(8.0);

                        let text_box = ui.add_sized(
                            [250.0, 34.0],
                            egui::TextEdit::singleline(&mut current_player.name)
                                .hint_text("Tu nombre aquí...")
                                .font(egui::TextStyle::Heading),
                        );

                        ui.add_space(14.0);

                        let can_play = !current_player.name.trim().is_empty();
                        let btn = ui.add_enabled(
                            can_play,
                            egui::Button::new(
                                egui::RichText::new("▶ ¡EMPEZAR A JUGAR!")
                                    .size(16.0)
                                    .color(egui::Color32::WHITE)
                                    .strong(),
                            )
                            .min_size(egui::vec2(220.0, 44.0))
                            .fill(if can_play {
                                egui::Color32::from_rgb(220, 50, 40)
                            } else {
                                egui::Color32::from_rgb(60, 60, 75)
                            }),
                        );

                        if btn.clicked() || (text_box.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) && can_play) {
                            next_state.set(AppState::Playing);
                        }
                    });

                ui.add_space(25.0);

                // Tabla de Posiciones / Leaderboard
                ui.heading(
                    egui::RichText::new("🏆 TABLA DE LÍDERES (TOP CLASIFICACIÓN)")
                        .size(17.0)
                        .color(egui::Color32::from_rgb(255, 215, 0)),
                );
                ui.add_space(8.0);

                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(14, 20, 35))
                    .rounding(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        egui::Grid::new("leaderboard_grid")
                            .striped(true)
                            .min_col_width(70.0)
                            .show(ui, |ui| {
                                ui.label(egui::RichText::new("#").strong().color(egui::Color32::GRAY));
                                ui.label(egui::RichText::new("Estudiante").strong().color(egui::Color32::WHITE));
                                ui.label(egui::RichText::new("Puntaje").strong().color(egui::Color32::from_rgb(255, 200, 50)));
                                ui.label(egui::RichText::new("Oleada").strong().color(egui::Color32::LIGHT_BLUE));
                                ui.end_row();

                                for (idx, entry) in leaderboard.entries.iter().take(8).enumerate() {
                                    let rank_icon = match idx {
                                        0 => "🥇",
                                        1 => "🥈",
                                        2 => "🥉",
                                        _ => "  ",
                                    };
                                    ui.label(format!("{} {}", rank_icon, idx + 1));
                                    ui.label(&entry.name);
                                    ui.label(format!("{} pts", entry.score));
                                    ui.label(format!("Ola {}", entry.wave));
                                    ui.end_row();
                                }
                            });
                    });
            });
        });
}

fn ui_playing_hud(
    mut contexts: EguiContexts,
    current_player: Res<CurrentPlayer>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_hud")
        .frame(egui::Frame::default().fill(egui::Color32::from_rgba_unmultiplied(10, 15, 28, 200)))
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("👤 {}", current_player.name))
                        .color(egui::Color32::WHITE)
                        .strong(),
                );

                ui.separator();

                ui.label(
                    egui::RichText::new(format!("⭐ {} pts", current_player.score))
                        .color(egui::Color32::from_rgb(255, 215, 50))
                        .strong()
                        .size(16.0),
                );

                ui.separator();

                ui.label(format!("🌊 Ola {}", current_player.wave));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let health_frac = (current_player.health / current_player.max_health).clamp(0.0, 1.0);
                    let bar_color = if health_frac > 0.5 {
                        egui::Color32::GREEN
                    } else if health_frac > 0.25 {
                        egui::Color32::YELLOW
                    } else {
                        egui::Color32::RED
                    };

                    ui.label(format!("{:.0}%", health_frac * 100.0));
                    let bar = egui::ProgressBar::new(health_frac)
                        .fill(bar_color)
                        .desired_width(90.0);
                    ui.add(bar);
                    ui.label("❤️");
                });
            });
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
        .frame(egui::Frame::default().fill(egui::Color32::from_rgb(18, 10, 14)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                ui.heading(
                    egui::RichText::new("💥 ¡NAVE DESTRUIDA! 💥")
                        .size(26.0)
                        .color(egui::Color32::from_rgb(255, 60, 60))
                        .strong(),
                );
                ui.add_space(10.0);

                ui.label(
                    egui::RichText::new(format!("¡Buen intento, {}!", current_player.name))
                        .size(17.0)
                        .color(egui::Color32::WHITE),
                );

                ui.label(
                    egui::RichText::new(format!("PUNTAJE FINAL: {} PTS", current_player.score))
                        .size(22.0)
                        .color(egui::Color32::from_rgb(255, 215, 0))
                        .strong(),
                );

                ui.add_space(18.0);

                // Botón Siguiente Jugador
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("🔄 SIGUIENTE ESTUDIANTE (Registrar otro jugador)")
                                .size(15.0)
                                .color(egui::Color32::WHITE)
                                .strong(),
                        )
                        .min_size(egui::vec2(280.0, 46.0))
                        .fill(egui::Color32::from_rgb(30, 140, 230)),
                    )
                    .clicked()
                {
                    next_state.set(AppState::NameInput);
                }

                ui.add_space(25.0);

                // Tabla de líderes actualizada
                ui.heading(
                    egui::RichText::new("🏆 CLASIFICACIÓN DEL EVENTO")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(255, 200, 50)),
                );
                ui.add_space(6.0);

                egui::Frame::default()
                    .fill(egui::Color32::from_rgb(25, 18, 26))
                    .rounding(8.0)
                    .inner_margin(12.0)
                    .show(ui, |ui| {
                        egui::Grid::new("gameover_leaderboard_grid")
                            .striped(true)
                            .min_col_width(75.0)
                            .show(ui, |ui| {
                                ui.label("#");
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

                                    ui.label(format!("{}", idx + 1));
                                    ui.label(name_text);
                                    ui.label(format!("{} pts", entry.score));
                                    ui.label(format!("Ola {}", entry.wave));
                                    ui.end_row();
                                }
                            });
                    });
            });
        });
}
