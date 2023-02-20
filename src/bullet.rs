use bevy::prelude::*;
use crate::*;


#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bullet {
    pub direction: Vec3,
    pub speed: f32,
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Bullet>()
            .add_system(bullet_despawn)
            .add_system(move_bullets)
            .add_system(bullet_collision);
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

fn move_bullets(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>,
) {
    for (bullet, mut transform) in bullets.iter_mut() {
        transform.translation += bullet.direction * bullet.speed * time.delta_seconds();
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
