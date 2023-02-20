use bevy::prelude::*;
use crate::*;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
}

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Tower>()
            .add_system(tower_shooting);
    }
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
