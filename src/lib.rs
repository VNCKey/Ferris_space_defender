use eframe::egui;
use rand::Rng;
use std::time::Instant;

// ============================================================================
// Tipos y Estructuras del Juego
// ============================================================================

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Paused,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Scout,         // Rápido, zig-zag
    Cruiser,       // Lento, dispara láseres
    Kamikaze,      // Sigilo y embestida veloz
    AsteroidLarge, // Se divide en 2 medianos
    AsteroidSmall, // Fragmento
    ShieldBearer,  // Protege a los demás
    Boss,          // Jefe nodriza con múltiples fases de disparo
}

#[derive(Clone, Copy, PartialEq)]
pub enum PowerUpType {
    TripleShot,
    HomingMissiles,
    TeslaBeam,
    CompanionDrone,
    Shield,
    Nuke,
    Health,
}

struct Star {
    x: f32,
    y: f32,
    speed: f32,
    size: f32,
    alpha: u8,
    color: egui::Color32,
}

struct Nebula {
    pos: egui::Pos2,
    radius: f32,
    color: egui::Color32,
    speed: f32,
}

struct Bullet {
    pos: egui::Pos2,
    vel: egui::Vec2,
    is_player: bool,
    damage: f32,
    radius: f32,
    color: egui::Color32,
    is_missile: bool,
}

struct Enemy {
    pos: egui::Pos2,
    vel: egui::Vec2,
    enemy_type: EnemyType,
    health: f32,
    max_health: f32,
    size: f32,
    rotation: f32,
    shoot_timer: f32,
}

struct Particle {
    pos: egui::Pos2,
    vel: egui::Vec2,
    color: egui::Color32,
    life: f32,
    max_life: f32,
    size: f32,
}

struct PowerUp {
    pos: egui::Pos2,
    vel: egui::Vec2,
    kind: PowerUpType,
    timer: f32,
}

struct FloatingText {
    pos: egui::Pos2,
    text: String,
    color: egui::Color32,
    life: f32,
    max_life: f32,
}

struct Drone {
    angle: f32,
    fire_timer: f32,
}

// ============================================================================
// Estado Principal de la Aplicación
// ============================================================================

pub struct SpaceDefenderGame {
    state: GameState,
    player_pos: egui::Pos2,
    player_health: f32,
    max_health: f32,
    player_shield: f32,
    
    // Potenciadores temporales
    triple_shot_timer: f32,
    missile_timer: f32,
    tesla_timer: f32,
    drone: Option<Drone>,

    // Estadísticas y progreso
    score: u32,
    high_score: u32,
    combo: u32,
    max_combo: u32,
    enemies_killed: u32,
    wave: u32,
    wave_timer: f32,
    
    // Entidades del mundo
    stars: Vec<Star>,
    nebulae: Vec<Nebula>,
    bullets: Vec<Bullet>,
    enemies: Vec<Enemy>,
    particles: Vec<Particle>,
    powerups: Vec<PowerUp>,
    floating_texts: Vec<FloatingText>,

    // Tiempos y generadores
    last_frame: Instant,
    fire_cooldown: f32,
    missile_cooldown: f32,
    spawn_timer: f32,
    screen_shake: f32,
    boss_active: bool,
}

impl Default for SpaceDefenderGame {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        
        let mut stars = Vec::with_capacity(160);
        for _ in 0..160 {
            let star_colors = [
                egui::Color32::from_rgb(220, 240, 255),
                egui::Color32::from_rgb(150, 200, 255),
                egui::Color32::from_rgb(255, 230, 180),
                egui::Color32::from_rgb(255, 180, 220),
            ];
            stars.push(Star {
                x: rng.gen_range(0.0..1000.0),
                y: rng.gen_range(0.0..1000.0),
                speed: rng.gen_range(25.0..180.0),
                size: rng.gen_range(1.0..3.5),
                alpha: rng.gen_range(90..255),
                color: star_colors[rng.gen_range(0..star_colors.len())],
            });
        }

        let mut nebulae = Vec::new();
        for _ in 0..5 {
            nebulae.push(Nebula {
                pos: egui::pos2(rng.gen_range(50.0..900.0), rng.gen_range(0.0..900.0)),
                radius: rng.gen_range(120.0..240.0),
                color: match rng.gen_range(0..3) {
                    0 => egui::Color32::from_rgba_unmultiplied(80, 20, 140, 25),
                    1 => egui::Color32::from_rgba_unmultiplied(10, 60, 120, 30),
                    _ => egui::Color32::from_rgba_unmultiplied(120, 30, 60, 20),
                },
                speed: rng.gen_range(10.0..30.0),
            });
        }

        Self {
            state: GameState::Playing,
            player_pos: egui::pos2(200.0, 520.0),
            player_health: 100.0,
            max_health: 100.0,
            player_shield: 0.0,
            triple_shot_timer: 0.0,
            missile_timer: 0.0,
            tesla_timer: 0.0,
            drone: None,
            score: 0,
            high_score: 0,
            combo: 1,
            max_combo: 1,
            enemies_killed: 0,
            wave: 1,
            wave_timer: 0.0,
            stars,
            nebulae,
            bullets: Vec::new(),
            enemies: Vec::new(),
            particles: Vec::new(),
            powerups: Vec::new(),
            floating_texts: Vec::new(),
            last_frame: Instant::now(),
            fire_cooldown: 0.0,
            missile_cooldown: 0.0,
            spawn_timer: 0.0,
            screen_shake: 0.0,
            boss_active: false,
        }
    }
}

impl SpaceDefenderGame {
    pub fn reset_game(&mut self, screen_center: egui::Pos2) {
        self.state = GameState::Playing;
        self.player_pos = screen_center;
        self.player_health = 100.0;
        self.player_shield = 0.0;
        self.triple_shot_timer = 0.0;
        self.missile_timer = 0.0;
        self.tesla_timer = 0.0;
        self.drone = None;
        self.score = 0;
        self.combo = 1;
        self.max_combo = 1;
        self.enemies_killed = 0;
        self.wave = 1;
        self.wave_timer = 0.0;
        self.bullets.clear();
        self.enemies.clear();
        self.particles.clear();
        self.powerups.clear();
        self.floating_texts.clear();
        self.boss_active = false;
        self.spawn_timer = 0.0;
        self.fire_cooldown = 0.0;
        self.missile_cooldown = 0.0;
        self.screen_shake = 0.0;

        self.spawn_floating_text(screen_center + egui::vec2(0.0, -60.0), "¡PREPÁRATE! OLEADA 1", egui::Color32::from_rgb(100, 255, 200), 1.8);
    }

    fn spawn_floating_text(&mut self, pos: egui::Pos2, text: &str, color: egui::Color32, duration: f32) {
        self.floating_texts.push(FloatingText {
            pos,
            text: text.to_string(),
            color,
            life: duration,
            max_life: duration,
        });
    }

    fn spawn_explosion(&mut self, pos: egui::Pos2, color: egui::Color32, count: usize, speed_scale: f32) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let speed = rng.gen_range(40.0..280.0) * speed_scale;
            let life = rng.gen_range(0.3..0.9);
            self.particles.push(Particle {
                pos,
                vel: egui::vec2(angle.cos() * speed, angle.sin() * speed),
                color,
                life,
                max_life: life,
                size: rng.gen_range(2.0..7.0),
            });
        }
    }

    fn spawn_nuke_effect(&mut self, screen_rect: egui::Rect) {
        self.screen_shake = 18.0;
        let mut rng = rand::thread_rng();
        for _ in 0..120 {
            let x = rng.gen_range(screen_rect.min.x..screen_rect.max.x);
            let y = rng.gen_range(screen_rect.min.y..screen_rect.max.y);
            self.spawn_explosion(egui::pos2(x, y), egui::Color32::from_rgb(120, 255, 255), 4, 0.6);
        }
        let killed = self.enemies.len() as u32;
        self.enemies_killed += killed;
        self.enemies.clear();
        self.boss_active = false;
        self.score += 400 * self.combo;
        self.spawn_floating_text(screen_rect.center(), "💥 ¡SUPER BOMBA EMP ACTIVADA!", egui::Color32::from_rgb(255, 230, 80), 2.0);
    }

    fn update_physics(&mut self, dt: f32, bounds: egui::Rect) {
        let mut rng = rand::thread_rng();

        // 1. Nebulosas cósmicas de fondo
        for neb in &mut self.nebulae {
            neb.pos.y += neb.speed * dt;
            if neb.pos.y - neb.radius > bounds.max.y {
                neb.pos.y = bounds.min.y - neb.radius;
                neb.pos.x = rng.gen_range(bounds.min.x..bounds.max.x);
            }
        }

        // 2. Estrellas de fondo (Parallax)
        for star in &mut self.stars {
            star.y += star.speed * dt;
            if star.y > bounds.max.y {
                star.y = bounds.min.y;
                star.x = rng.gen_range(bounds.min.x..bounds.max.x);
            }
        }

        if self.state != GameState::Playing {
            return;
        }

        // 3. Temporizadores de PowerUps y Wave
        self.wave_timer += dt;
        if self.wave_timer > 30.0 && !self.boss_active {
            self.wave_timer = 0.0;
            self.wave += 1;
            self.spawn_floating_text(
                self.player_pos + egui::vec2(0.0, -80.0),
                &format!("🚀 ¡OLEADA {} INICIADA!", self.wave),
                egui::Color32::from_rgb(100, 255, 150),
                2.2,
            );
        }

        if self.triple_shot_timer > 0.0 { self.triple_shot_timer -= dt; }
        if self.missile_timer > 0.0 { self.missile_timer -= dt; }
        if self.tesla_timer > 0.0 { self.tesla_timer -= dt; }
        if self.screen_shake > 0.0 { self.screen_shake = (self.screen_shake - dt * 25.0).max(0.0); }
        
        self.fire_cooldown -= dt;
        self.missile_cooldown -= dt;
        self.spawn_timer += dt;

        // 4. Actualizar Dron de Apoyo (si está activo)
        let mut drone_bullets = Vec::new();
        if let Some(drone) = &mut self.drone {
            drone.angle += dt * 3.5;
            drone.fire_timer -= dt;
            if drone.fire_timer <= 0.0 {
                drone.fire_timer = 0.25;
                let drone_pos = self.player_pos + egui::vec2(drone.angle.cos() * 40.0, drone.angle.sin() * 40.0);
                drone_bullets.push(Bullet {
                    pos: drone_pos,
                    vel: egui::vec2(0.0, -650.0),
                    is_player: true,
                    damage: 14.0,
                    radius: 3.0,
                    color: egui::Color32::from_rgb(100, 255, 200),
                    is_missile: false,
                });
            }
        }
        self.bullets.extend(drone_bullets);

        // 5. Generación Progresiva de Enemigos
        let spawn_interval = (2.0 - (self.wave as f32 * 0.15)).max(0.5);
        if self.spawn_timer >= spawn_interval && !self.boss_active {
            self.spawn_timer = 0.0;
            let spawn_x = rng.gen_range(bounds.min.x + 30.0..bounds.max.x - 30.0);
            let roll = rng.gen_range(0..100);

            if self.score > 0 && self.score % 600 < 60 && !self.boss_active {
                // Jefe Nodriza
                self.boss_active = true;
                self.enemies.push(Enemy {
                    pos: egui::pos2(bounds.center().x, bounds.min.y - 70.0),
                    vel: egui::vec2(70.0, 35.0),
                    enemy_type: EnemyType::Boss,
                    health: 180.0 + (self.wave as f32 * 60.0),
                    max_health: 180.0 + (self.wave as f32 * 60.0),
                    size: 48.0,
                    rotation: 0.0,
                    shoot_timer: 1.0,
                });
                self.spawn_floating_text(bounds.center(), "⚠️ ALERTA: ¡NAVE NODRIZA JEFE DETECTADA! ⚠️", egui::Color32::from_rgb(255, 60, 60), 3.0);
                self.screen_shake = 10.0;
            } else if roll < 30 {
                // Scout
                self.enemies.push(Enemy {
                    pos: egui::pos2(spawn_x, bounds.min.y - 20.0),
                    vel: egui::vec2(rng.gen_range(-50.0..50.0), rng.gen_range(160.0..240.0)),
                    enemy_type: EnemyType::Scout,
                    health: 16.0,
                    max_health: 16.0,
                    size: 16.0,
                    rotation: 0.0,
                    shoot_timer: 0.0,
                });
            } else if roll < 55 {
                // Asteroide Grande
                self.enemies.push(Enemy {
                    pos: egui::pos2(spawn_x, bounds.min.y - 35.0),
                    vel: egui::vec2(rng.gen_range(-25.0..25.0), rng.gen_range(70.0..130.0)),
                    enemy_type: EnemyType::AsteroidLarge,
                    health: 40.0,
                    max_health: 40.0,
                    size: 28.0,
                    rotation: rng.gen_range(0.0..std::f32::consts::TAU),
                    shoot_timer: 0.0,
                });
            } else if roll < 75 {
                // Kamikaze rápido
                self.enemies.push(Enemy {
                    pos: egui::pos2(spawn_x, bounds.min.y - 20.0),
                    vel: egui::vec2(0.0, 280.0),
                    enemy_type: EnemyType::Kamikaze,
                    health: 12.0,
                    max_health: 12.0,
                    size: 14.0,
                    rotation: 0.0,
                    shoot_timer: 0.0,
                });
            } else {
                // Crucero de combate
                self.enemies.push(Enemy {
                    pos: egui::pos2(spawn_x, bounds.min.y - 30.0),
                    vel: egui::vec2(rng.gen_range(-35.0..35.0), rng.gen_range(60.0..100.0)),
                    enemy_type: EnemyType::Cruiser,
                    health: 60.0,
                    max_health: 60.0,
                    size: 30.0,
                    rotation: 0.0,
                    shoot_timer: 1.5,
                });
            }
        }

        // 6. Físicas de Misiles Teledirigidos
        let enemies_positions: Vec<egui::Pos2> = self.enemies.iter().map(|e| e.pos).collect();
        for b in &mut self.bullets {
            if b.is_missile && !enemies_positions.is_empty() {
                // Buscar enemigo más cercano
                let mut closest_dist = f32::MAX;
                let mut target_pos = None;
                for ep in &enemies_positions {
                    let d = b.pos.distance(*ep);
                    if d < closest_dist {
                        closest_dist = d;
                        target_pos = Some(*ep);
                    }
                }
                if let Some(target) = target_pos {
                    let dir = (target - b.pos).normalized();
                    let t = (dt * 8.0).min(1.0);
                    b.vel = b.vel * (1.0 - t) + (dir * 550.0) * t;
                }
            }
        }

        // 7. Partículas de propulsión
        if rng.gen_bool(0.7) {
            let p_color = if self.tesla_timer > 0.0 {
                egui::Color32::from_rgb(100, 255, 255)
            } else if self.triple_shot_timer > 0.0 {
                egui::Color32::from_rgb(255, 80, 255)
            } else {
                egui::Color32::from_rgb(100, 210, 255)
            };
            self.particles.push(Particle {
                pos: egui::pos2(self.player_pos.x + rng.gen_range(-6.0..6.0), self.player_pos.y + 18.0),
                vel: egui::vec2(rng.gen_range(-15.0..15.0), rng.gen_range(90.0..200.0)),
                color: p_color,
                life: 0.35,
                max_life: 0.35,
                size: rng.gen_range(3.0..6.5),
            });
        }

        // 8. Mover y filtrar balas
        for bullet in &mut self.bullets {
            bullet.pos += bullet.vel * dt;
        }
        self.bullets.retain(|b| bounds.expand(30.0).contains(b.pos));

        // 9. Mover y actualizar enemigos
        let mut enemy_bullets = Vec::new();
        for enemy in &mut self.enemies {
            enemy.pos += enemy.vel * dt;
            enemy.rotation += dt * 1.8;

            // Rebotar en bordes X
            if enemy.pos.x < bounds.min.x + enemy.size || enemy.pos.x > bounds.max.x - enemy.size {
                enemy.vel.x *= -1.0;
            }

            // Disparos
            if enemy.enemy_type == EnemyType::Cruiser {
                enemy.shoot_timer -= dt;
                if enemy.shoot_timer <= 0.0 {
                    enemy.shoot_timer = 1.6;
                    enemy_bullets.push(Bullet {
                        pos: enemy.pos + egui::vec2(0.0, enemy.size * 0.8),
                        vel: egui::vec2(0.0, 260.0),
                        is_player: false,
                        damage: 16.0,
                        radius: 5.0,
                        color: egui::Color32::from_rgb(255, 50, 80),
                        is_missile: false,
                    });
                }
            } else if enemy.enemy_type == EnemyType::Boss {
                enemy.shoot_timer -= dt;
                if enemy.shoot_timer <= 0.0 {
                    enemy.shoot_timer = 0.55;
                    // Disparo en abanico
                    for angle_deg in [-25.0, 0.0, 25.0] {
                        let rad = (angle_deg as f32).to_radians();
                        enemy_bullets.push(Bullet {
                            pos: enemy.pos + egui::vec2(0.0, enemy.size * 0.8),
                            vel: egui::vec2(rad.sin() * 280.0, rad.cos() * 280.0),
                            is_player: false,
                            damage: 18.0,
                            radius: 6.0,
                            color: egui::Color32::from_rgb(255, 60, 100),
                            is_missile: false,
                        });
                    }
                }
            }
        }
        self.bullets.extend(enemy_bullets);

        // 10. Colisiones de Balas del Jugador contra Enemigos
        let mut dead_enemies = Vec::new();
        let mut spawned_powerups = Vec::new();
        let mut spawned_fragments = Vec::new();
        let mut damage_texts = Vec::new();

        for (e_idx, enemy) in self.enemies.iter_mut().enumerate() {
            for bullet in self.bullets.iter_mut().filter(|b| b.is_player) {
                if bullet.pos.distance(enemy.pos) < enemy.size + bullet.radius {
                    enemy.health -= bullet.damage;
                    damage_texts.push((enemy.pos + egui::vec2(rng.gen_range(-15.0..15.0), -10.0), format!("-{}", bullet.damage as u32), egui::Color32::from_rgb(255, 220, 80)));
                    bullet.pos.y = -9999.0;

                    if enemy.health <= 0.0 {
                        dead_enemies.push(e_idx);
                        break;
                    }
                }
            }
        }

        for (p, t, c) in damage_texts {
            self.spawn_floating_text(p, &t, c, 0.6);
        }

        // 11. Procesar enemigos eliminados y fragmentación de asteroides
        dead_enemies.sort_unstable_by(|a, b| b.cmp(a));
        dead_enemies.dedup();
        for idx in dead_enemies {
            let enemy = self.enemies.remove(idx);
            self.enemies_killed += 1;

            let (score_add, exp_color, exp_count) = match enemy.enemy_type {
                EnemyType::Scout => (30, egui::Color32::from_rgb(255, 100, 50), 20),
                EnemyType::Kamikaze => (45, egui::Color32::from_rgb(255, 40, 40), 25),
                EnemyType::Cruiser => (80, egui::Color32::from_rgb(200, 70, 255), 35),
                EnemyType::AsteroidLarge => (40, egui::Color32::from_rgb(140, 145, 160), 25),
                EnemyType::AsteroidSmall => (25, egui::Color32::from_rgb(120, 125, 140), 15),
                EnemyType::ShieldBearer => (100, egui::Color32::from_rgb(80, 200, 255), 35),
                EnemyType::Boss => (600, egui::Color32::from_rgb(255, 215, 0), 75),
            };

            self.score += score_add * self.combo;
            self.combo += 1;
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
            if self.score > self.high_score {
                self.high_score = self.score;
            }

            self.spawn_explosion(enemy.pos, exp_color, exp_count, if enemy.enemy_type == EnemyType::Boss { 2.2 } else { 1.0 });

            if enemy.enemy_type == EnemyType::Boss {
                self.boss_active = false;
                self.wave += 1;
                self.screen_shake = 16.0;
                self.spawn_floating_text(enemy.pos, "👑 ¡JEFE DESTRUIDO! (+600)", egui::Color32::from_rgb(255, 220, 50), 2.5);
            }

            // Fragmentar asteroide grande en 2 pequeños
            if enemy.enemy_type == EnemyType::AsteroidLarge {
                spawned_fragments.push(Enemy {
                    pos: enemy.pos + egui::vec2(-14.0, 0.0),
                    vel: egui::vec2(-50.0, 110.0),
                    enemy_type: EnemyType::AsteroidSmall,
                    health: 15.0,
                    max_health: 15.0,
                    size: 14.0,
                    rotation: 0.0,
                    shoot_timer: 0.0,
                });
                spawned_fragments.push(Enemy {
                    pos: enemy.pos + egui::vec2(14.0, 0.0),
                    vel: egui::vec2(50.0, 110.0),
                    enemy_type: EnemyType::AsteroidSmall,
                    health: 15.0,
                    max_health: 15.0,
                    size: 14.0,
                    rotation: 0.0,
                    shoot_timer: 0.0,
                });
            }

            // Probabilidad de soltar PowerUp
            if rng.gen_bool(0.28) || enemy.enemy_type == EnemyType::Boss {
                let kind = match rng.gen_range(0..7) {
                    0 => PowerUpType::TripleShot,
                    1 => PowerUpType::HomingMissiles,
                    2 => PowerUpType::TeslaBeam,
                    3 => PowerUpType::CompanionDrone,
                    4 => PowerUpType::Shield,
                    5 => PowerUpType::Nuke,
                    _ => PowerUpType::Health,
                };
                spawned_powerups.push(PowerUp {
                    pos: enemy.pos,
                    vel: egui::vec2(0.0, 75.0),
                    kind,
                    timer: 0.0,
                });
            }
        }
        self.enemies.extend(spawned_fragments);
        self.powerups.extend(spawned_powerups);

        // 12. Colisiones y Daño al Jugador
        let mut total_damage = 0.0;
        let p_pos = self.player_pos;

        for bullet in self.bullets.iter_mut().filter(|b| !b.is_player) {
            if bullet.pos.distance(p_pos) < 18.0 + bullet.radius {
                bullet.pos.y = 9999.0;
                total_damage += bullet.damage;
            }
        }

        for enemy in &self.enemies {
            if enemy.pos.distance(p_pos) < 18.0 + enemy.size * 0.7 {
                total_damage += match enemy.enemy_type {
                    EnemyType::Kamikaze => 45.0,
                    EnemyType::Boss => 50.0,
                    _ => 25.0,
                };
            }
        }

        if total_damage > 0.0 {
            self.take_damage(total_damage);
        }

        // 13. Recoger PowerUps
        let mut trigger_nuke = false;
        let mut collected_texts = Vec::new();
        let max_hp = self.max_health;
        let p_shield = &mut self.player_shield;
        let p_health = &mut self.player_health;
        let triple_timer = &mut self.triple_shot_timer;
        let missile_timer = &mut self.missile_timer;
        let tesla_timer = &mut self.tesla_timer;
        let drone_opt = &mut self.drone;

        self.powerups.retain_mut(|p| {
            p.pos += p.vel * dt;
            p.timer += dt * 4.0;
            if p.pos.distance(p_pos) < 28.0 {
                match p.kind {
                    PowerUpType::TripleShot => {
                        *triple_timer = 9.0;
                        collected_texts.push((p_pos, "⚡ DISPARO TRIPLE ACTIVO", egui::Color32::from_rgb(255, 80, 255)));
                    }
                    PowerUpType::HomingMissiles => {
                        *missile_timer = 8.0;
                        collected_texts.push((p_pos, "🚀 MISILES TELEDIRIGIDOS", egui::Color32::from_rgb(255, 140, 50)));
                    }
                    PowerUpType::TeslaBeam => {
                        *tesla_timer = 7.0;
                        collected_texts.push((p_pos, "⚡ RAYO TESLA DESTRUCTOR", egui::Color32::from_rgb(100, 255, 255)));
                    }
                    PowerUpType::CompanionDrone => {
                        *drone_opt = Some(Drone { angle: 0.0, fire_timer: 0.0 });
                        collected_texts.push((p_pos, "🤖 DRON DE APOYO DESPLEGADO", egui::Color32::from_rgb(100, 255, 180)));
                    }
                    PowerUpType::Shield => {
                        *p_shield = (*p_shield + 50.0).min(100.0);
                        collected_texts.push((p_pos, "🛡️ ESCUDO AL MÁXIMO", egui::Color32::from_rgb(80, 200, 255)));
                    }
                    PowerUpType::Health => {
                        *p_health = (*p_health + 40.0).min(max_hp);
                        collected_texts.push((p_pos, "💖 CASCO REPARADO (+40)", egui::Color32::from_rgb(80, 255, 120)));
                    }
                    PowerUpType::Nuke => {
                        trigger_nuke = true;
                    }
                }
                false
            } else {
                p.pos.y < bounds.max.y + 20.0
            }
        });

        for (pos, text, color) in collected_texts {
            self.spawn_floating_text(pos + egui::vec2(0.0, -30.0), text, color, 1.4);
            self.spawn_explosion(pos, color, 15, 0.8);
        }

        if trigger_nuke {
            self.spawn_nuke_effect(bounds);
        }

        // 14. Textos flotantes
        self.floating_texts.retain_mut(|t| {
            t.pos.y -= dt * 35.0;
            t.life -= dt;
            t.life > 0.0
        });

        // 15. Partículas
        self.particles.retain_mut(|p| {
            p.pos += p.vel * dt;
            p.life -= dt;
            p.life > 0.0
        });

        // 16. Game Over
        if self.player_health <= 0.0 {
            self.state = GameState::GameOver;
            self.spawn_explosion(self.player_pos, egui::Color32::from_rgb(255, 50, 50), 60, 2.8);
            self.screen_shake = 22.0;
        }

        self.enemies.retain(|e| e.pos.y < bounds.max.y + 60.0);
    }

    fn take_damage(&mut self, amount: f32) {
        if self.combo > 5 {
            self.spawn_floating_text(self.player_pos, "¡COMBO PERDIDO!", egui::Color32::from_rgb(255, 100, 100), 1.0);
        }
        self.combo = 1;
        self.screen_shake = 10.0;
        if self.player_shield > 0.0 {
            let rem = amount - self.player_shield;
            self.player_shield = (self.player_shield - amount).max(0.0);
            if rem > 0.0 {
                self.player_health = (self.player_health - rem).max(0.0);
            }
            self.spawn_explosion(self.player_pos, egui::Color32::from_rgb(100, 200, 255), 12, 0.9);
        } else {
            self.player_health = (self.player_health - amount).max(0.0);
            self.spawn_explosion(self.player_pos, egui::Color32::from_rgb(255, 70, 70), 15, 1.2);
        }
    }

    fn fire_weapons(&mut self) {
        if self.fire_cooldown <= 0.0 {
            self.fire_cooldown = 0.15;

            // Disparo Principal
            if self.triple_shot_timer > 0.0 {
                self.bullets.push(Bullet {
                    pos: self.player_pos + egui::vec2(0.0, -22.0),
                    vel: egui::vec2(0.0, -680.0),
                    is_player: true,
                    damage: 22.0,
                    radius: 4.5,
                    color: egui::Color32::from_rgb(255, 80, 255),
                    is_missile: false,
                });
                self.bullets.push(Bullet {
                    pos: self.player_pos + egui::vec2(-14.0, -14.0),
                    vel: egui::vec2(-130.0, -640.0),
                    is_player: true,
                    damage: 20.0,
                    radius: 4.0,
                    color: egui::Color32::from_rgb(255, 140, 255),
                    is_missile: false,
                });
                self.bullets.push(Bullet {
                    pos: self.player_pos + egui::vec2(14.0, -14.0),
                    vel: egui::vec2(130.0, -640.0),
                    is_player: true,
                    damage: 20.0,
                    radius: 4.0,
                    color: egui::Color32::from_rgb(255, 140, 255),
                    is_missile: false,
                });
            } else {
                self.bullets.push(Bullet {
                    pos: self.player_pos + egui::vec2(-9.0, -18.0),
                    vel: egui::vec2(0.0, -620.0),
                    is_player: true,
                    damage: 18.0,
                    radius: 3.5,
                    color: egui::Color32::from_rgb(100, 230, 255),
                    is_missile: false,
                });
                self.bullets.push(Bullet {
                    pos: self.player_pos + egui::vec2(9.0, -18.0),
                    vel: egui::vec2(0.0, -620.0),
                    is_player: true,
                    damage: 18.0,
                    radius: 3.5,
                    color: egui::Color32::from_rgb(100, 230, 255),
                    is_missile: false,
                });
            }
        }

        // Disparo de Misiles Teledirigidos
        if self.missile_timer > 0.0 && self.missile_cooldown <= 0.0 {
            self.missile_cooldown = 0.45;
            self.bullets.push(Bullet {
                pos: self.player_pos + egui::vec2(-18.0, 0.0),
                vel: egui::vec2(-160.0, -350.0),
                is_player: true,
                damage: 35.0,
                radius: 5.5,
                color: egui::Color32::from_rgb(255, 160, 40),
                is_missile: true,
            });
            self.bullets.push(Bullet {
                pos: self.player_pos + egui::vec2(18.0, 0.0),
                vel: egui::vec2(160.0, -350.0),
                is_player: true,
                damage: 35.0,
                radius: 5.5,
                color: egui::Color32::from_rgb(255, 160, 40),
                is_missile: true,
            });
        }
    }
}

// ============================================================================
// Renderizado y Ciclo de Vida eframe::App
// ============================================================================

impl eframe::App for SpaceDefenderGame {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let dt = (now.duration_since(self.last_frame).as_secs_f32()).min(0.05);
        self.last_frame = now;

        ctx.request_repaint();

        #[cfg(target_os = "android")]
        ctx.set_pixels_per_point(2.2);

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(egui::Color32::from_rgb(4, 7, 12)))
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                self.update_physics(dt, rect);

                // Control Táctil y Teclado sin interferir con el layout
                if self.state == GameState::Playing {
                    ctx.input(|i| {
                        if let Some(pos) = i.pointer.interact_pos().or(i.pointer.hover_pos()) {
                            if i.pointer.is_decidedly_dragging() || i.pointer.primary_down() || i.pointer.primary_clicked() {
                                self.player_pos = self.player_pos.lerp(pos, (dt * 24.0).min(1.0));
                            }
                        }

                        let speed = 440.0 * dt;
                        if i.key_down(egui::Key::ArrowLeft) || i.key_down(egui::Key::A) {
                            self.player_pos.x -= speed;
                        }
                        if i.key_down(egui::Key::ArrowRight) || i.key_down(egui::Key::D) {
                            self.player_pos.x += speed;
                        }
                        if i.key_down(egui::Key::ArrowUp) || i.key_down(egui::Key::W) {
                            self.player_pos.y -= speed;
                        }
                        if i.key_down(egui::Key::ArrowDown) || i.key_down(egui::Key::S) {
                            self.player_pos.y += speed;
                        }
                    });

                    self.player_pos.x = self.player_pos.x.clamp(rect.min.x + 22.0, rect.max.x - 22.0);
                    self.player_pos.y = self.player_pos.y.clamp(rect.min.y + 45.0, rect.max.y - 45.0);

                    // Disparos automáticos continuos
                    self.fire_weapons();
                }

                let painter = ui.painter();

                // 1. Dibujar Nebulosas Cósmicas
                for neb in &self.nebulae {
                    painter.circle_filled(neb.pos, neb.radius, neb.color);
                }

                // 2. Dibujar Estrellas
                for star in &self.stars {
                    painter.circle_filled(
                        egui::pos2(star.x, star.y),
                        star.size,
                        egui::Color32::from_rgba_unmultiplied(star.color.r(), star.color.g(), star.color.b(), star.alpha),
                    );
                }

                // 3. Dibujar Rayo Tesla (si está activo)
                if self.state == GameState::Playing && self.tesla_timer > 0.0 {
                    for enemy in &self.enemies {
                        if enemy.pos.distance(self.player_pos) < 220.0 {
                            painter.line_segment(
                                [self.player_pos + egui::vec2(0.0, -18.0), enemy.pos],
                                egui::Stroke::new(3.0_f32, egui::Color32::from_rgb(100, 255, 255)),
                            );
                        }
                    }
                }

                // 4. Dibujar Partículas
                for p in &self.particles {
                    let progress = p.life / p.max_life;
                    let alpha = (progress * 255.0) as u8;
                    let color = egui::Color32::from_rgba_unmultiplied(p.color.r(), p.color.g(), p.color.b(), alpha);
                    painter.circle_filled(p.pos, p.size * progress, color);
                }

                // 5. Dibujar PowerUps
                for p in &self.powerups {
                    let pulse = (p.timer.sin() * 3.5) + 14.0;
                    let (color, label) = match p.kind {
                        PowerUpType::TripleShot => (egui::Color32::from_rgb(255, 50, 255), "⚡3X"),
                        PowerUpType::HomingMissiles => (egui::Color32::from_rgb(255, 140, 40), "🚀MIS"),
                        PowerUpType::TeslaBeam => (egui::Color32::from_rgb(80, 255, 255), "⚡TES"),
                        PowerUpType::CompanionDrone => (egui::Color32::from_rgb(80, 255, 180), "🤖DRN"),
                        PowerUpType::Shield => (egui::Color32::from_rgb(50, 200, 255), "🛡️ESC"),
                        PowerUpType::Nuke => (egui::Color32::from_rgb(255, 220, 50), "💣BOM"),
                        PowerUpType::Health => (egui::Color32::from_rgb(50, 255, 100), "💖VID"),
                    };
                    painter.circle_filled(p.pos, pulse, color.gamma_multiply(0.25));
                    painter.circle_stroke(p.pos, pulse, egui::Stroke::new(2.0_f32, color));
                    painter.text(p.pos, egui::Align2::CENTER_CENTER, label, egui::FontId::proportional(11.0), egui::Color32::WHITE);
                }

                // 6. Dibujar Balas y Misiles
                for b in &self.bullets {
                    if b.is_missile {
                        painter.circle_filled(b.pos, b.radius + 3.0, egui::Color32::from_rgb(255, 120, 30).gamma_multiply(0.4));
                        painter.circle_filled(b.pos, b.radius, egui::Color32::from_rgb(255, 200, 80));
                    } else {
                        painter.circle_filled(b.pos, b.radius + 2.0, b.color.gamma_multiply(0.4));
                        painter.circle_filled(b.pos, b.radius, egui::Color32::WHITE);
                    }
                }

                // 7. Dibujar Enemigos
                for enemy in &self.enemies {
                    match enemy.enemy_type {
                        EnemyType::Scout => {
                            let p1 = enemy.pos + egui::vec2(0.0, enemy.size);
                            let p2 = enemy.pos + egui::vec2(-enemy.size * 0.8, -enemy.size * 0.6);
                            let p3 = enemy.pos + egui::vec2(enemy.size * 0.8, -enemy.size * 0.6);
                            painter.add(egui::Shape::convex_polygon(
                                vec![p1, p2, p3],
                                egui::Color32::from_rgb(255, 75, 75),
                                egui::Stroke::new(1.5_f32, egui::Color32::from_rgb(255, 190, 190)),
                            ));
                        }
                        EnemyType::Kamikaze => {
                            let p1 = enemy.pos + egui::vec2(0.0, enemy.size * 1.2);
                            let p2 = enemy.pos + egui::vec2(-enemy.size * 0.6, -enemy.size);
                            let p3 = enemy.pos + egui::vec2(enemy.size * 0.6, -enemy.size);
                            painter.add(egui::Shape::convex_polygon(
                                vec![p1, p2, p3],
                                egui::Color32::from_rgb(255, 30, 30),
                                egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(255, 220, 80)),
                            ));
                        }
                        EnemyType::Cruiser => {
                            painter.rect_filled(
                                egui::Rect::from_center_size(enemy.pos, egui::vec2(enemy.size * 1.5, enemy.size * 1.2)),
                                5.0,
                                egui::Color32::from_rgb(180, 50, 255),
                            );
                            painter.circle_filled(enemy.pos, 7.0, egui::Color32::from_rgb(255, 220, 50));
                        }
                        EnemyType::AsteroidLarge | EnemyType::AsteroidSmall => {
                            painter.circle_filled(enemy.pos, enemy.size, egui::Color32::from_rgb(105, 110, 125));
                            painter.circle_stroke(enemy.pos, enemy.size, egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(160, 165, 180)));
                        }
                        EnemyType::ShieldBearer => {
                            painter.circle_filled(enemy.pos, enemy.size, egui::Color32::from_rgb(40, 120, 220));
                            painter.circle_stroke(enemy.pos, enemy.size + 6.0, egui::Stroke::new(2.5_f32, egui::Color32::from_rgb(80, 220, 255)));
                        }
                        EnemyType::Boss => {
                            painter.rect_filled(
                                egui::Rect::from_center_size(enemy.pos, egui::vec2(enemy.size * 2.4, enemy.size * 1.4)),
                                8.0,
                                egui::Color32::from_rgb(240, 35, 55),
                            );
                            painter.rect_stroke(
                                egui::Rect::from_center_size(enemy.pos, egui::vec2(enemy.size * 2.4, enemy.size * 1.4)),
                                8.0,
                                egui::Stroke::new(3.0_f32, egui::Color32::from_rgb(255, 215, 0)),
                                egui::StrokeKind::Outside,
                            );

                            // Barra de vida del Boss
                            let hp_pct = (enemy.health / enemy.max_health).clamp(0.0, 1.0);
                            let bar_rect = egui::Rect::from_min_size(
                                egui::pos2(rect.center().x - 130.0, rect.min.y + 44.0),
                                egui::vec2(260.0, 12.0),
                            );
                            painter.rect_filled(bar_rect, 4.0, egui::Color32::from_rgb(40, 10, 20));
                            painter.rect_filled(
                                egui::Rect::from_min_size(bar_rect.min, egui::vec2(bar_rect.width() * hp_pct, bar_rect.height())),
                                4.0,
                                egui::Color32::from_rgb(255, 50, 50),
                            );
                            painter.text(
                                egui::pos2(rect.center().x, bar_rect.min.y - 14.0),
                                egui::Align2::CENTER_CENTER,
                                "⚠️ MOTHERSHIP DREADNOUGHT ⚠️",
                                egui::FontId::proportional(13.0),
                                egui::Color32::from_rgb(255, 215, 0),
                            );
                        }
                    }
                }

                // 8. Dibujar Dron de Apoyo
                if self.state == GameState::Playing {
                    if let Some(drone) = &self.drone {
                        let drone_pos = self.player_pos + egui::vec2(drone.angle.cos() * 40.0, drone.angle.sin() * 40.0);
                        painter.circle_filled(drone_pos, 7.0, egui::Color32::from_rgb(50, 220, 180));
                        painter.circle_stroke(drone_pos, 7.0, egui::Stroke::new(1.5_f32, egui::Color32::WHITE));
                    }
                }

                // 9. Dibujar Nave del Jugador (Geometría Robusta y Brillante)
                if self.state == GameState::Playing {
                    let nose = self.player_pos + egui::vec2(0.0, -24.0);
                    let wing_l = self.player_pos + egui::vec2(-20.0, 18.0);
                    let wing_r = self.player_pos + egui::vec2(20.0, 18.0);
                    let base_center = self.player_pos + egui::vec2(0.0, 10.0);

                    // 1. Llama del propulsor animada
                    let flame_len = (self.last_frame.elapsed().as_secs_f32() * 40.0).sin().abs() * 8.0 + 16.0;
                    let flame_tip = self.player_pos + egui::vec2(0.0, 12.0 + flame_len);
                    painter.add(egui::Shape::convex_polygon(
                        vec![self.player_pos + egui::vec2(-8.0, 12.0), self.player_pos + egui::vec2(8.0, 12.0), flame_tip],
                        egui::Color32::from_rgb(255, 140, 30),
                        egui::Stroke::NONE,
                    ));
                    painter.add(egui::Shape::convex_polygon(
                        vec![self.player_pos + egui::vec2(-4.0, 12.0), self.player_pos + egui::vec2(4.0, 12.0), self.player_pos + egui::vec2(0.0, 12.0 + flame_len * 0.6)],
                        egui::Color32::from_rgb(255, 240, 100),
                        egui::Stroke::NONE,
                    ));

                    // 2. Alas Principales (Triángulo Convexo)
                    painter.add(egui::Shape::convex_polygon(
                        vec![nose, wing_l, wing_r],
                        egui::Color32::from_rgb(15, 110, 220),
                        egui::Stroke::new(2.5_f32, egui::Color32::from_rgb(100, 225, 255)),
                    ));

                    // 3. Fuselaje Central
                    painter.add(egui::Shape::convex_polygon(
                        vec![nose, self.player_pos + egui::vec2(-7.0, 14.0), base_center, self.player_pos + egui::vec2(7.0, 14.0)],
                        egui::Color32::from_rgb(30, 150, 255),
                        egui::Stroke::new(1.5_f32, egui::Color32::WHITE),
                    ));

                    // 4. Cabina / Cockpit Brillante
                    painter.circle_filled(self.player_pos + egui::vec2(0.0, -4.0), 6.0, egui::Color32::from_rgb(150, 240, 255));
                    painter.circle_filled(self.player_pos + egui::vec2(0.0, -5.0), 3.0, egui::Color32::WHITE);

                    // 5. Cañones de las alas
                    painter.circle_filled(wing_l + egui::vec2(0.0, -4.0), 3.5, egui::Color32::from_rgb(100, 255, 255));
                    painter.circle_filled(wing_r + egui::vec2(0.0, -4.0), 3.5, egui::Color32::from_rgb(100, 255, 255));

                    // 6. Escudo de Fuerza de Neón
                    if self.player_shield > 0.0 {
                        painter.circle_stroke(
                            self.player_pos,
                            30.0,
                            egui::Stroke::new(3.0_f32, egui::Color32::from_rgb(80, 220, 255)),
                        );
                        painter.circle_filled(self.player_pos, 30.0, egui::Color32::from_rgba_unmultiplied(80, 220, 255, 45));
                    }
                }

                // 10. Textos Flotantes (Daño, combos, powerups)
                for t in &self.floating_texts {
                    let progress = t.life / t.max_life;
                    let alpha = (progress * 255.0) as u8;
                    let color = egui::Color32::from_rgba_unmultiplied(t.color.r(), t.color.g(), t.color.b(), alpha);
                    painter.text(t.pos, egui::Align2::CENTER_CENTER, &t.text, egui::FontId::proportional(14.0), color);
                }

                // 11. HUD Superior Completo
                if self.state == GameState::Playing {
                    let hud_rect = egui::Rect::from_min_size(rect.min + egui::vec2(16.0, 16.0), egui::vec2(rect.width() - 32.0, 48.0));
                    
                    // Barra de Vida
                    let hp_pct = (self.player_health / self.max_health).clamp(0.0, 1.0);
                    let hp_bar = egui::Rect::from_min_size(hud_rect.min, egui::vec2(110.0, 12.0));
                    painter.rect_filled(hp_bar, 3.0, egui::Color32::from_rgb(30, 40, 55));
                    painter.rect_filled(
                        egui::Rect::from_min_size(hp_bar.min, egui::vec2(hp_bar.width() * hp_pct, hp_bar.height())),
                        3.0,
                        if hp_pct > 0.3 { egui::Color32::from_rgb(50, 230, 110) } else { egui::Color32::from_rgb(255, 60, 60) },
                    );

                    // Barra de Escudo (si tiene escudo)
                    if self.player_shield > 0.0 {
                        let sh_pct = (self.player_shield / 100.0).clamp(0.0, 1.0);
                        let sh_bar = egui::Rect::from_min_size(hp_bar.left_bottom() + egui::vec2(0.0, 4.0), egui::vec2(110.0, 6.0));
                        painter.rect_filled(sh_bar, 2.0, egui::Color32::from_rgb(20, 40, 60));
                        painter.rect_filled(
                            egui::Rect::from_min_size(sh_bar.min, egui::vec2(sh_bar.width() * sh_pct, sh_bar.height())),
                            2.0,
                            egui::Color32::from_rgb(80, 210, 255),
                        );
                    }

                    // Score y Combo
                    painter.text(
                        hp_bar.left_bottom() + egui::vec2(0.0, 16.0),
                        egui::Align2::LEFT_TOP,
                        format!("SCORE: {} (x{} COMBO)", self.score, self.combo),
                        egui::FontId::proportional(15.0),
                        egui::Color32::from_rgb(100, 210, 255),
                    );

                    // Wave y Kills
                    painter.text(
                        hud_rect.right_top(),
                        egui::Align2::RIGHT_TOP,
                        format!("OLEADA {}", self.wave),
                        egui::FontId::proportional(15.0),
                        egui::Color32::from_rgb(255, 200, 80),
                    );
                    painter.text(
                        hud_rect.right_top() + egui::vec2(0.0, 18.0),
                        egui::Align2::RIGHT_TOP,
                        format!("BAJAS: {}", self.enemies_killed),
                        egui::FontId::proportional(12.0),
                        egui::Color32::from_rgb(180, 190, 210),
                    );
                }

                // 12. Pantalla de Game Over
                if self.state == GameState::GameOver {
                    // Oscurecer fondo
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 190));

                        let center = rect.center();
                        painter.text(
                            center + egui::vec2(0.0, -90.0),
                            egui::Align2::CENTER_CENTER,
                            "💥 MISIÓN FALLIDA",
                            egui::FontId::proportional(30.0),
                            egui::Color32::from_rgb(255, 60, 60),
                        );
                        painter.text(
                            center + egui::vec2(0.0, -45.0),
                            egui::Align2::CENTER_CENTER,
                            format!("Puntaje Final: {}", self.score),
                            egui::FontId::proportional(22.0),
                            egui::Color32::from_rgb(255, 215, 0),
                        );
                        painter.text(
                            center + egui::vec2(0.0, -15.0),
                            egui::Align2::CENTER_CENTER,
                            format!("Récord Máximo: {}", self.high_score),
                            egui::FontId::proportional(15.0),
                            egui::Color32::from_rgb(100, 210, 255),
                        );
                        painter.text(
                            center + egui::vec2(0.0, 15.0),
                            egui::Align2::CENTER_CENTER,
                            format!("Bajas: {} | Máx Combo: x{}", self.enemies_killed, self.max_combo),
                            egui::FontId::proportional(14.0),
                            egui::Color32::from_rgb(180, 200, 220),
                        );

                        // Botón de reintento
                        let btn_rect = egui::Rect::from_center_size(center + egui::vec2(0.0, 75.0), egui::vec2(250.0, 54.0));
                        painter.rect_filled(btn_rect, 12.0, egui::Color32::from_rgb(35, 134, 54));
                        painter.rect_stroke(btn_rect, 12.0, egui::Stroke::new(2.0_f32, egui::Color32::WHITE), egui::StrokeKind::Outside);
                        painter.text(
                            btn_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            "🔄 TOCAR PARA REINTENTAR",
                            egui::FontId::proportional(15.0),
                            egui::Color32::WHITE,
                        );

                        ctx.input(|i| {
                            if i.pointer.primary_clicked() {
                                self.reset_game(rect.center() + egui::vec2(0.0, 100.0));
                            }
                        });
                }
            });
    }
}

// Punto de entrada para Android Native
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: android_activity::AndroidApp) {
    let mut options = eframe::NativeOptions::default();
    options.android_app = Some(app);
    let _ = eframe::run_native(
        "Ferris Defender",
        options,
        Box::new(|_cc| Ok(Box::new(SpaceDefenderGame::default()))),
    );
}
