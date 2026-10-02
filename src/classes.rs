use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShipClass {
    Defensor,
    Mago,
    Asesino,
    Artillero,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
