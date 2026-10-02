# 🦀 Ferris Space Defender

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
[![Engine](https://img.shields.io/badge/Engine-Bevy-blue.svg)](https://bevyengine.org/)
[![UI](https://img.shields.io/badge/UI-bevy__egui-purple.svg)](https://github.com/mvlabat/bevy_egui)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

**Ferris Space Defender** es un videojuego de disparos espacial arcade (Shoot 'em up / SHMUP) desarrollado totalmente en **Rust** utilizando el motor de videojuegos **Bevy Engine (ECS)** e interfaz gráfica **bevy_egui**.

Defiende la galaxia de hordas alienígenas controlando a Ferris con diferentes arquetipos de naves, habilidades especiales, oleadas dinámicas, enfrentamientos contra jefes finales y sistema de récords (Leaderboard).

---

## 🚀 Características Principales

- **4 Arquetipos de Naves Jugables:**
  - 🛡️ **Mutex Titan (Defensor):** Blindaje pesado, pulso cinético y protocolo bastión.
  - 🧙 **Zero-Cost Tecnomante (Mago):** Disparos de plasma místico, nova arcana y escudo de éter.
  - ⚡ **Tokio Async (Asesino):** Velocidad extrema, ráfaga dual y fase asíncrona.
  - 🚀 **Cargo Buster (Artillero):** Daño masivo en abanico, salva de misiles y bombardeo EMP.

- **Sistema de Oleadas y Jefes:**
  - Múltiples tipos de enemigos con IA de movimiento diferencial.
  - Enfrentamientos contra jefes gigantes con patrones de ataque avanzados y barras de vida.

- **Física y Efectos:**
  - Sistema de partículas para explosiones, propulsores e impactos.
  - Sistema de audio dinámico para disparo, impacto, escudos y música de fondo.

- **Tabla de Clasificación (Leaderboard):**
  - Guarda automáticamente las puntuaciones locales en formato JSON (`highscores.json`).

---

## 🛠️ Requisitos e Instalación

### Prerrequisitos
Tener instalado Rust y Cargo (versión 2021 o superior). Si no lo tienes, puedes instalarlo desde [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

En Linux (Ubuntu/Debian), instala las dependencias necesarias para Bevy:

```bash
sudo apt update
sudo apt install g++ pkg-config libx11-dev libasound2-dev libudev-dev libwayland-dev libxkbcommon-dev
```

---

## 🎮 Cómo Jugar

1. Clonar el repositorio:
   ```bash
   git clone https://github.com/VNCKey/Ferris_space_defender.git
   cd Ferris_space_defender
   ```

2. Compilar y ejecutar en modo Release (recomendado para máximo rendimiento):
   ```bash
   cargo run --release
   ```

### ⌨️ Controles
- **W, A, S, D** o **Flechas de Dirección:** Mover la nave.
- **ESPACIO:** Disparar láser principal.
- **1, 2, 3:** Activar Habilidades Especiales.
- **ESC:** Pausa.

---

## 📂 Arquitectura del Proyecto

El proyecto sigue el patrón **ECS (Entity Component System)** promovido por Bevy Engine:

```text
src/
├── lib.rs              # Punto de entrada de la librería y configuración de Plugins
├── state.rs            # Estados del juego (AppState) y Leaderboard
├── classes.rs          # Estadísticas y habilidades de los 4 arquetipos de nave
├── components.rs       # Componentes ECS (Player, Enemy, Boss, Laser, Particle)
├── resources.rs        # Recursos globales de Bevy (Score, Wave, Assets)
├── ui/                 # Interfaz de usuario con egui (Menú, HUD, Game Over)
└── systems/            # Lógica del juego (Player, Enemies, Combat, Audio)
```

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo `LICENSE` para más detalles.
