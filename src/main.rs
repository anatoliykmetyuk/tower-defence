use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    shooting_timer: Timer,
    bullet_offset: Vec3,
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    despawn_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TargetSpawner {
    spawn_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    value: i32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    direction: Vec3,
    speed: f32,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(spawn_camera)
        .add_startup_system(asset_loading)
        .add_system(spawn_targets)
        .add_system(tower_shooting)
        .add_system(bullet_despawn)
        .add_system(move_targets)
        .add_system(move_bullets)
        .add_system(target_death)
        .add_system(bullet_collision)
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
        .register_type::<Tower>()
        .register_type::<Lifetime>()
        .register_type::<Target>()
        .register_type::<Health>()
        .register_type::<TargetSpawner>()
        .register_type::<Bullet>()
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

fn tower_shooting(
    mut commands: Commands,
    game_assets: ResMut<GameAssets>,
    mut towers: Query<(Entity, &mut Tower, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    time: Res<Time>,
) {
    for (entity, mut tower, transform) in towers.iter_mut() {
        if tower.shooting_timer.tick(time.delta()).just_finished() {
            // Find the closest target and set the direction vector to point at it
            let direction = targets.iter()
                .map(|target_transform| target_transform.translation() - (transform.translation() + tower.bullet_offset))
                .min_by(|a, b| a.length().partial_cmp(&b.length()).unwrap());

            if let Some(direction) = direction {
                commands.entity(entity).with_children(|commands| {
                    commands.spawn(SceneBundle {
                        scene: game_assets.bullet_scene.clone(),
                        transform: Transform::from_translation(tower.bullet_offset),
                        ..default()
                    })
                    .insert(Bullet {
                        direction: direction.normalize(),
                        speed: 5.0,
                    })
                    .insert(Lifetime {
                        despawn_timer: Timer::from_seconds(3.0, TimerMode::Once),
                    })
                    .insert(Name::new("Bullet"));
                });
            }
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &mut Lifetime)>,
) {
    for (entity, mut lifetime) in bullets.iter_mut() {
        if lifetime.despawn_timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn asset_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        bullet_scene: asset_server.load("Tomato.glb#Scene0"),
    });
}

fn spawn_targets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut target_spawners: Query<(&mut TargetSpawner, &Transform)>,
    time: Res<Time>,
) {
    for (mut target_spawner, transform) in target_spawners.iter_mut() {
        if target_spawner.spawn_timer.tick(time.delta()).just_finished() {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),
                material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
                transform: transform.clone(),
                ..default()
            })
            .insert(Target {
                speed: 0.5,
            })
            .insert(Health {
                value: 5,
            })
            .insert(Lifetime {
                despawn_timer: Timer::from_seconds(10.0, TimerMode::Once),
            })
            .insert(Name::new("Target"));
        }
    }
}

fn move_targets(
    mut targets: Query<(&Target, &mut Transform)>,
    time: Res<Time>,
) {
    for (target, mut transform) in targets.iter_mut() {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
    }
}

fn target_death(
    mut commands: Commands,
    mut targets: Query<(Entity, &Health)>,
) {
    for (entity, health) in targets.iter_mut() {
        if health.value <= 0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullets: Query<(Entity, &GlobalTransform), With<Bullet>>,
    mut targets: Query<(&mut Health, &GlobalTransform), With<Target>>,
) {
    for (bullet_entity, bullet_transform) in &bullets {
        for (mut health, target_transform) in &mut targets {
            if Vec3::distance(bullet_transform.translation(), target_transform.translation()) <= 0.25 {
                health.value -= 1;
                commands.entity(bullet_entity).despawn_recursive();
                break;
            }
        }
    }
}
