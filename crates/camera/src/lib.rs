use bevy::prelude::*;
use core::AppState;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3d::default(),
        Projection::Orthographic(OrthographicProjection {
            scale: 5.0,
            scaling_mode: bevy::camera::ScalingMode::FixedVertical { viewport_height: 2.0 },
            ..OrthographicProjection::default_3d()
        }),
        transform,
    ));
    
    // Light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
