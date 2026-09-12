# 🚀 FerriSpace (Ferris Space Defender)

**FerriSpace** es un juego arcade móvil de disparos espaciales (Space Shooter Rogue-lite) desarrollado completamente en **Rust** utilizando el motor **Bevy** e interfaz integrada con **bevy_egui**.

El juego está protagonizado por **Ferris** (la mascota oficial de Rust) piloteando naves espaciales de combate para defender la galaxia de amenazas cósmicas y jefes espaciales como el temido **Borrow Checker**.

---

## 🛠️ Tecnologías y Requisitos

- **Lenguaje:** Rust 2021 Edition
- **Motor de Juego:** Bevy `0.14`
- **Interfaz Gráfica (UI):** `bevy_egui` `0.28` / `egui`
- **Empaquetado Android:** `cargo-apk` `0.10.0`
- **Android NDK:** NDK v27 (`27.0.12077973`)
- **Dispositivo Objetivo:** Android (ARM64-v8a)

---

## 🚀 Clases de Naves Disponibles

El juego cuenta con **4 clases de naves** seleccionables desde el menú de inicio, cada una con atributos, armas y estilos de juego únicos:

| Clase | Nombre Completo | HP | Escudo | Velocidad | Cadencia | Daño Láser | Estilo de Disparo |
|---|---|---|---|---|---|---|---|
| 🛡️ **Defensor** | **Mutex Titan** | 160 | 90 | 270 px/s | 0.28s | 55 PTS | Cañón pesado de plasma dorado |
| 🔮 **Mago** | **Zero-Cost Tecnomante** | 100 | 60 | 350 px/s | 0.16s | 32 PTS | Orbes mísitcos cian rápidos |
| ⚡ **Asesino** | **Tokio Async** | 80 | 40 | 440 px/s | 0.10s | 18 PTS | Cañón dual verde neón ultrarrápido |
| 💥 **Artillero** | **Cargo Buster** | 120 | 50 | 310 px/s | 0.22s | 35 PTS | Ráfaga triple en abanico rojo |

---

## ⚡ Sistema Rogue-lite de 16 Habilidades (Draft de 3 Cartas)

Al derrotar a un Jefe Espacial, el juego se pausa e invoca un modal de elección donde el jugador elige 1 de 3 cartas generadas aleatoriamente:

### 🔘 Habilidades Activables (con Botón Táctil en HUD)
1. **Zero-Cost Beam (`zero_cost_beam.png`):** Emite un rayo de plasma devastador continuo durante 3 segundos. *(Cooldown: 25s)*.
2. **Cargo Clean (`cargo_clean.png`):** Onda expansiva EMP que borra todos los proyectiles enemigos en pantalla. *(Cooldown: 18s)*.

### 🛡️ Habilidades Pasivas
3. **Overclock Mutex (`overclock_mutex.png`):** +25% Cadencia de disparo del láser.
4. **Async Multithread (`async_multithread.png`):** +20% Velocidad de proyectil y +1 línea láser adicional.
5. **Unsafe Block (`unsafe_block.png`):** +40% Daño de disparo (reduce la Salud Máxima en -15%).
6. **Borrow Checker (`borrow_checker.png`):** Refleja un 25% del daño recibido hacia enemigos cercanos.
7. **Pattern Matching (`pattern_matching.png`):** +15% Probabilidad de Golpe Crítico (hace 3x daño).
8. **Arc\<Mutex\> (`arc_mutex.png`):** Genera un mini-escudo orbital que bloquea disparos.
9. **Tokio Reactor (`tokio_reactor.png`):** +20% Velocidad de movimiento de la nave.
10. **Vector Capacity (`vector_capacity.png`):** +50 Puntos de Escudo Máximo adicional.
11. **Panic Recovery (`panic_recovery.png`):** Invulnerabilidad de 2.5s al recibir un impacto mortal *(1 vez por partida)*.
12. **Mutex Overdrive (`mutex_overdrive.png`):** +30% Daño total cuando el escudo está agotado (0 ESC).
13. **Static Lifetime (`static_lifetime.png`):** Regenera 1% de Salud del casco por segundo.
14. **Macro_Rules! (`macro_rules.png`):** Incrementa el tamaño y área de impacto de los proyectiles.
15. **Option::Some (`option_some.png`):** +35% Frecuencia de caída de Íconos Power-Up.
16. **Zero-Cost Abstraction (`zero_cost_abstraction.png`):** Reduce los tiempos de recarga en -20%.

---

## 🏅 Sistema de Torneo y Rangos Finales

Al finalizar la partida (Game Over), se evalúa la puntuación total del jugador para asignar una medalla de rango:

| Rango | Titulo | Criterio de Puntaje |
|---|---|---|
| 🥇 **Rango S** | Leyenda de Rust | Puntaje ≥ 200,000 pts |
| 🥈 **Rango A** | Comandante Senior | Puntaje 80,000 - 199,999 pts |
| 🥉 **Rango B** | Piloto Certificado | Puntaje 30,000 - 79,999 pts |
| 🎖️ **Rango C** | Cadete Space | Puntaje < 30,000 pts |

---

## 💻 Guía de Comandos de Desarrollo y Despliegue ADB

### 1. Verificar Código
```bash
cargo check
```

### 2. Compilar APK Release para Android
```bash
ANDROID_NDK_ROOT=/home/alek/Android/Sdk/ndk/27.0.12077973 cargo apk build --lib --release
```

### 3. Instalar/Actualizar en el Celular (Xiaomi / ADB)
```bash
adb -s s4lj4dlnnz7ho79l install --no-incremental -r target/release/apk/ferris_space_defender.apk
```

### 4. Lanzar la Aplicación en Pantalla
```bash
adb -s s4lj4dlnnz7ho79l shell am start -n com.ferriskey.spacedefender/android.app.NativeActivity
```

### 5. Copiar APK a la Carpeta Descargas del Celular (Para Compartir)
```bash
adb -s s4lj4dlnnz7ho79l push target/release/apk/ferris_space_defender.apk /sdcard/Download/FerriSpace.apk
```

---

## 🎨 Icono y Recursos Android (`res/mipmap`)

El icono del lanzador móvil se encuentra configurado en `Cargo.toml` (`[package.metadata.android.application]`) y `AndroidManifest.xml`:
- `res/mipmap-mdpi/ic_launcher.png` (48x48)
- `res/mipmap-hdpi/ic_launcher.png` (72x72)
- `res/mipmap-xhdpi/ic_launcher.png` (96x96)
- `res/mipmap-xxhdpi/ic_launcher.png` (144x144)
- `res/mipmap-xxxhdpi/ic_launcher.png` (192x192)
