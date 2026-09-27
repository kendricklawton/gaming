use bevy::prelude::*;
use core::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_player)
           .add_systems(Update, move_player.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Component)]
pub struct Player;

fn setup_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane (Ground)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Player (Cube)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
    ));
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let Ok(mut player_transform) = query.single_mut() else { return; };
    let mut direction = Vec3::ZERO;
    let move_speed = 3.0 * time.delta_secs();

    let mut stick_x = 0.0;
    let mut stick_y = 0.0;

    for gamepad in gamepads.iter() {
        if gamepad.pressed(GamepadButton::DPadUp) {
            stick_y += 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadDown) {
            stick_y -= 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadRight) {
            stick_x += 1.0;
        }
        if gamepad.pressed(GamepadButton::DPadLeft) {
            stick_x -= 1.0;
        }

        let l_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap_or(0.0);
        let l_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap_or(0.0);

        stick_x += l_stick_x;
        stick_y += l_stick_y;
    }

    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        stick_y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
        stick_y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
        stick_x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
        stick_x -= 1.0;
    }

    direction.x -= stick_y;
    direction.z -= stick_y;
    direction.x += stick_x;
    direction.z -= stick_x;

    if direction.length() > 0.1 {
        let length_clamper = direction.length().min(1.0);
        direction = direction.normalize() * length_clamper;
        player_transform.translation += direction * move_speed;
    }
}
