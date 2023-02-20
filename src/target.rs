use bevy::prelude::*;
use crate::*;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TargetSpawner {
    pub spawn_timer: Timer,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: i32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Target>()
            .register_type::<TargetSpawner>()
            .register_type::<Health>()
            .add_system(spawn_targets)
            .add_system(move_targets)
            .add_system(target_death);
    }
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
