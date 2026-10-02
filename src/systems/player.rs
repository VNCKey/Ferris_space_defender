use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;
use crate::classes::*;

pub fn player_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    current_player: Res<CurrentPlayer>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        let speed = current_player.ship_class.speed() * current_player.speed_mult * current_player.class_speed_mult;
        transform.translation += direction * speed * time.delta_seconds();

        // Limites de pantalla
        transform.translation.x = transform.translation.x.clamp(-350.0, 350.0);
        transform.translation.y = transform.translation.y.clamp(-600.0, 600.0);
    }
}
