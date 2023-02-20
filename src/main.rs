mod tower;
mod bullet;
mod target;

pub use tower::*;
pub use bullet::*;
pub use target::*;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    despawn_timer: Timer,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)
        .add_plugin(TowerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Bevy Tower Defence".to_string(),
                width: WIDTH,
                height: HEIGHT,
                resizable: false,
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .register_type::<Lifetime>()
        .run();
}

fn spawn_camera(mut commands : Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_basic_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    })
    .insert(Name::new("Ground"));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 1.0),
        ..default()
    })
    .insert(Tower {
        shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        bullet_offset: Vec3::new(0.0, 0.0, 0.6),
    })
    .insert(Name::new("Tower"));

    commands.spawn(TargetSpawner {
        spawn_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
    })
    .insert(Transform::from_xyz(-2.5, 0.5, 2.5))
    .insert(Name::new("Target Spawner"));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 15000.0,
            // color: Color::rgb(0.2, 0.5, 0.5),
            shadows_enabled: true,
            ..default()
        },
        ..default()
    })
    .insert(Name::new("Light"));
}

fn asset_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        bullet_scene: asset_server.load("Tomato.glb#Scene0"),
    });
}
