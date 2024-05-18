use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

mod camera;
mod player;
fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        bevy_xpbd_3d::prelude::PhysicsPlugins::default(),
        bevy_inspector_egui::quick::WorldInspectorPlugin::default(),
    ))
    .add_plugins(camera::CameraPlugin)
    .insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 50.0,
    })
    .insert_resource(player::PlayerData::default())
    .add_systems(Startup, setup)
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(bevy::prelude::Cuboid::new(50.0, 0.5, 50.0).mesh()),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(0.0, -0.25, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cuboid(50.0, 0.5, 50.0),
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::ORANGE,
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..default()
    });
}
